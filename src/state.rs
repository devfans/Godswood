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
    renderer::{self, rendy::{self, mesh::*}, light, palette, camera, shape, debug_drawing::{DebugLines, DebugLinesComponent, DebugLinesParams} },
    ui,
    utils::{self, scene},
    window,
};

use crate::component::*;


pub type GodsPrefabData = scene::BasicScenePrefab<(Vec<Position>, Vec<Normal>, Vec<TexCoord>)>;


#[derive(Default)]
pub struct Loading {
    progress: assets::ProgressCounter,
    prefab: Option<assets::Handle<assets::Prefab<GodsPrefabData>>>,
    wood: Option<Value>,
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
                Trans::Switch(Box::new(Show { scene: self.prefab.as_ref().unwrap().clone(), woods, }))
            }
            assets::Completion::Loading => Trans::None
        }
    }
}

pub struct Show {
    scene: assets::Handle<assets::Prefab<GodsPrefabData>>,
    woods: Forest,
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
            transform.prepend_rotation_x_axis(PI * 0.03);
            // transform.prepend_rotation_z_axis(PI * 0.03);
            let (width, height) = {
                let dim = w.read_resource::<window::ScreenDimensions>();
                (dim.width(), dim.height())
            };
            w.create_entity()
                .with(camera::Camera::standard_3d(width, height))
                .with(transform)
                .build();
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

        macro_rules! draw_circle {
            ($center: expr, $radius: expr) => {
                {
                    debug_lines_component.add_rotated_circle(
                        $center,
                        $radius,
                        100,
                        UnitQuaternion::from_axis_angle(&Vector3::x_axis(), FRAC_PI_2),
                        palette::Srgba::new(200.0, 200.0, 200.23, 1.0),
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
            ($node: expr, $pos: expr) => {
                {
                    // Create godswood node
                                        
                    w.create_entity()
                        .with($pos)
                        .with(mesh.clone())
                        .with(mtl.clone())
                        .with(GodsNode { node: $node })
                        .build();
                }
            }
        }

        let woods = self.woods.woods.read().unwrap();
        for wood in woods.values() {
            let wood = wood.read().unwrap();
            let mut depth = 1;
            let gap = wood.base_gap * -1.0f32;
            let root = wood.wood.get_root().upgrade().unwrap();
            let node = root.read().unwrap();
            {
                create_node!(root.clone(), pos!(0.0, 0.0, 0.0));
                let point = Point3::new(0.0, 0.0, 0.0);
                let direction = Vector3::new(0.0, gap, 0.0);
                draw_line!(point, direction);
                draw_circle!(Point3::new(0.0, 0.0, -gap), wood.scales.get(&depth).unwrap().clone());
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


