use crate::node::*;
use crate::tree::*;
use std::f32::consts::{PI, FRAC_PI_2};
use serde_json::Value;
use amethyst:: {
    Error,
    assets,
    core::{self, math::{Vector3, Point3, UnitQuaternion }},
    derive,
    ecs::prelude::{
        Entity, Join, Read, ReadStorage, System, Write, WriteStorage
    },
    input,
    prelude::*,
    renderer::{self, rendy::{self, mesh::*}, light, palette, camera, shape, debug_drawing::{DebugLines, DebugLinesComponent, DebugLinesParams}, plugins },
    ui::{self, get_default_font, FontHandle, TtfFormat, FontAsset, UiText, UiTransform, Anchor},
    utils::{self, scene},
    window,
};

use std::collections::VecDeque;

use crate::component::*;


pub type GodsPrefabData = scene::BasicScenePrefab<(Vec<Position>, Vec<Normal>, Vec<TexCoord>)>;


#[derive(Default)]
pub struct Loading {
    progress: assets::ProgressCounter,
    prefab: Option<assets::Handle<assets::Prefab<GodsPrefabData>>>,
    wood: Option<Value>,
    font: Option<FontHandle>
}

impl Loading {
    pub fn new(raw: &Value) -> Loading {
        let mut state = Loading::default();
        state.wood = Some(raw.clone());
        state
    }
}


impl SimpleState for Loading {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        self.prefab = Some(data.world.exec(|loader: assets::PrefabLoader<'_, GodsPrefabData>|{
            loader.load("prefab/godsnode.ron", assets::RonFormat, &mut self.progress)
        }));

        {
            let loader = data.world.read_resource::<assets::Loader>();
            let fonts = data.world.read_resource::<assets::AssetStorage<FontAsset>>();
            self.font = Some(loader.load("font/square.ttf", TtfFormat, &mut self.progress, &fonts));
        }

        data.world.exec(|mut creator: ui::UiCreator<'_>| {
            creator.create("ui/fps.ron", &mut self.progress);
            creator.create("ui/loading.ron", &mut self.progress);
        });

        data.world.register::<GodsNode>();
        data.world.register::<DebugLinesComponent>();

    }

    fn update(&mut self, data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        match self.progress.complete() {
            assets::Completion::Failed => {
                println!("Failed loading assets: {:?}", self.progress.errors());
                Trans::Quit
            }
            assets::Completion::Complete => {
                println!("Assets loaded, swapping stats");
                if let Some(entity) = data.world.exec(|finder: ui::UiFinder<'_>| finder.find("loading")) {
                    let _ = data.world.delete_entity(entity);
                }
                let raw = self.wood.as_ref().unwrap();
                let mut woods = Forest::new();
                println!("Adding new wood");
                woods.add_wood(&raw);
                let font = self.font.clone().unwrap().clone();
                Trans::Switch(Box::new(Show { scene: self.prefab.as_ref().unwrap().clone(), woods, font }))
            }
            assets::Completion::Loading => Trans::None
        }
    }
}

pub struct Show {
    scene: assets::Handle<assets::Prefab<GodsPrefabData>>,
    woods: Forest,
    font: FontHandle,
}

impl SimpleState for Show {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        // data.world.create_entity().with(self.scene.clone()).build();
        // data.world.create_entity().with(self.scene.clone()).build();
        let w = data.world;

        // load assets
        let (mesh, albedo) = {
            let mesh = w.exec(|loader: assets::AssetLoaderSystemData<'_, renderer::Mesh>| {
                loader.load_from_data(
                    shape::Shape::Sphere(32, 32)
                        .generate::<(Vec<Position>, Vec<Normal>, Vec<Tangent>, Vec<TexCoord>)>(None)
                        .into(),
                    (),
                )
            });
            let albedo = w.exec(|loader: assets::AssetLoaderSystemData<'_, renderer::Texture>| {
                loader.load_from_data(
                    rendy::texture::palette::load_from_linear_rgba(palette::LinSrgba::new(1.0, 1.0, 1.0, 0.5)).into(),
                    (),
                )
            });

            (mesh, albedo)
        };

        { 
            // Add lights
            let mut light1_transform = core::Transform::default();
            light1_transform.set_translation_xyz(6.0, -6.0, -6.0);
           

            let light1: light::Light = light::PointLight {
                intensity: 5.0,
                color: palette::Srgb::new(0.0, 0.3, 0.7),
                ..light::PointLight::default()
            }.into();
            w.create_entity()
                .with(light1)
                .with(light1_transform)
                .build();
        }

        {
            // Add camera
            let mut transform = core::Transform::default();
            transform.set_translation_xyz(0.0, 10.0, -50.0);
            transform.prepend_rotation_y_axis(PI);
            // transform.prepend_rotation_y_axis(PI * 0.03);
            transform.prepend_rotation_x_axis(PI * 0.08);
            // transform.prepend_rotation_z_axis(PI * 0.03);
            let (width, height) = {
                let dim = w.read_resource::<window::ScreenDimensions>();
                (dim.width(), dim.height())
            };
            let entity = w.create_entity()
                .with(camera::Camera::standard_3d(width, height))
                .with(transform)
                .build();
            w.insert(renderer::ActiveCamera {
                entity: Some(entity),
            });
        }

        // Add debug lines
        // Setup debug lines as a resource
        w.insert(DebugLines::new());
        // Configure width of lines. Optional step
        w.insert(DebugLinesParams { line_width: 2.0 });

        let mut debug_lines_component = DebugLinesComponent::with_capacity(100);

        macro_rules! draw_line {
            ($begin: expr, $end: expr) => {
                {
                    debug_lines_component.add_direction(
                        $begin,
                        $end,
                        palette::Srgba::new(200.0, 200.0, 200.23, 1.0),
                    );
                }
            }
        }

        macro_rules! add_rotated_circle {
            ($center: expr, $radius: expr, $points: expr, $rotation: expr, $color: expr) => {
                {
                    let mut prev = None;

                    for i in 0..=$points {
                        let a = PI * 2.0 / ($points as f32) * (i as f32);
                        let x = $radius * a.cos();
                        let y = $radius * a.sin();
                        let point = Vector3::new(x, y, 0.0);
                        let point = $center + $rotation * point;

                        if let Some(prev) = prev {
                            debug_lines_component.add_line(prev, point, $color);
                        }

                        prev = Some(point);
                    }

                }
            }
        }


        macro_rules! draw_circle {
            ($center: expr, $radius: expr) => {
                {
                    debug_lines_component.add_rotated_circle(
                    // add_rotated_circle!(
                        $center,
                        $radius,
                        100,
                        UnitQuaternion::from_axis_angle(&Vector3::x_axis(), FRAC_PI_2),
                        palette::Srgba::new(200.0, 200.0, 200.23, 1.0)
                    );
                }
            }
        }


        let mat_defaults = w.read_resource::<renderer::MaterialDefaults>().0.clone();

        let roughness = 1.0f32 * (3.0 / 4.0f32);
        let metallic = 1.0f32 * (2.0 / 4.0f32);

        let mtl = w.exec(
            |(mtl_loader, tex_loader): (
                assets::AssetLoaderSystemData<'_, renderer::Material>,
                assets::AssetLoaderSystemData<'_, renderer::Texture>,
            )| {
                let metallic_roughness = tex_loader.load_from_data(
                    rendy::texture::palette::load_from_linear_rgba(palette::LinSrgba::new(0.0, roughness, metallic, 0.0))
                        .into(),
                    (),
                );

                mtl_loader.load_from_data(
                    renderer::Material {
                        albedo: albedo.clone(),
                        metallic_roughness,
                        ..mat_defaults.clone()
                    },
                    (),
                )
            },
        );

        macro_rules! pos {
            ($x: expr, $y: expr, $z: expr) => {
                {
                    let mut pos = core::Transform::default();
                    pos.set_translation_xyz($x, $y, $z);
                    pos
                }
            }
        }

        macro_rules! create_node {
            ($node: expr, $pos: expr, $id: expr, $name: expr) => {
                {
                    // Create godswood node
                    let parent = w.create_entity()
                        .with($pos.clone())
                        .with(mesh.clone())
                        .with(mtl.clone())
                        .with(GodsNode { node: $node })
                        .build();

                    // Create UI display
                    w.create_entity()
                        .with(core::Parent { entity: parent })
                        .with($pos)
                        .with(UiTransform::new(format!("node{}", $id), Anchor::Middle, Anchor::Middle, 0., 10., 0., 200., 50.))
                        .with(UiText::new(self.font.clone(), format!("node{}", $name), [255., 10., 10., 1.], 50.0))
                        .build();
                }
            }
        }

        let woods = self.woods.woods.read().unwrap();
        let mut nodes = VecDeque::new();
        for wood in woods.values() {
            let wood = wood.read().unwrap();
            let gap = wood.base_gap * -1.0f32;
            let direction = Vector3::new(0.0, gap, 0.0);

            nodes.push_back(((0.0, 0.0, 0.0), wood.wood.get_root(), 1));

            loop {
                let node = nodes.pop_front();
                if node.is_none() {
                    break;
                }

                let ((x, y, z), node, depth) = node.unwrap();
                let node_arc = node.upgrade().unwrap();
                let scale = wood.scales.get(&depth).unwrap() * wood.base_scale;
                let node = node_arc.read().unwrap();
                create_node!(node_arc.clone(), pos!(x, y, z), node.id, node.name.clone());

                let children = node.get_children();
                let size = children.len();
                if size == 0 {
                    continue;
                } else if size == 1 {
                    draw_line!(Point3::new(x, y, z), Vector3::new(0.0, -wood.base_gap, 0.0));
                    nodes.push_back(((x, y - wood.base_gap, z), children[0].clone(), depth + 1));
                    continue;
                }

                // draw_line!(Point3::new(x, y, z), direction);
                draw_circle!(Point3::new(x, y - wood.base_gap, z), scale);
                break;

                let mut points = Vec::new();

                let angle = 2f32 * PI / size as f32;
                for i in 0..size {
                    let angle = angle * i as f32;
                    let kid_x = x - scale * angle.cos();
                    let kid_y = y - wood.base_gap;
                    let kid_z = z - scale * angle.sin();

                    draw_line!(Point3::new(x, y, z), Vector3::new(kid_x - x, kid_y - y, kid_z - z));
                    points.push((kid_x, kid_y, kid_z));
                }

                for node in children.iter() {
                    nodes.push_back((points.pop().unwrap(), node.clone(), depth + 1));
                }
            }

            /*
            let nodes = wood.wood.get_nodes_by_depths();
            let depth = wood.wood.get_depth();
            let nodes = nodes.read().unwrap();
            for i in 1..depth + 1 {
                let items = nodes.get(&i).unwrap();
                let mut index = 0.0f32;
                for item in items.iter() {
                    create_node!(item.upgrade().unwrap().clone(), pos!(3.0 * i as f32, 3.0 * index, 0.0));
                    index += 1.0;
                }
            }
            */
        }

        // let node = self.woods.store.new_node();
        // create_node!(node);
        
        w.create_entity()
            .with(debug_lines_component)
            .build();

    }

    fn handle_event(&mut self, data: StateData<'_, GameData<'_, '_>>, event: StateEvent) -> SimpleTrans {
        Trans::None
    }
}


