use crate::node::*;
use std::f32::consts::PI;
use amethyst:: {
    Error,
    assets,
    core,
    derive,
    ecs::prelude::{
        Entity, Join, Read, ReadStorage, System, Write, WriteStorage
    },
    input,
    prelude::*,
    renderer::{self, rendy::{self, mesh::*}, light, palette, camera, shape },
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

        data.world.register::<Castle>();

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
                Trans::Switch(Box::new(Show { scene: self.prefab.as_ref().unwrap().clone(), woods: Forest::new() }))
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
            transform.set_translation_xyz(0.0, 0.0, -12.0);
            transform.prepend_rotation_y_axis(PI);
            let (width, height) = {
                let dim = w.read_resource::<window::ScreenDimensions>();
                (dim.width(), dim.height())
            };
            w.create_entity()
                .with(camera::Camera::standard_3d(width, height))
                .with(transform)
                .build();
        }

        let mat_defaults = w.read_resource::<renderer::MaterialDefaults>().0.clone();


        macro_rules! create_node {
            ($node: expr) => {
                {
                    // Create godswood node
                    let roughness = 1.0f32 * (3.0 / 4.0f32);
                    let metallic = 1.0f32 * (2.0 / 4.0f32);

                    let mut pos = core::Transform::default();
                    pos.set_translation_xyz(2.0f32 * (1 - 2) as f32, 2.0f32 * (3 - 2) as f32, 0.0);

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

                    w.create_entity()
                        .with(pos)
                        .with(mesh.clone())
                        .with(mtl)
                        .with(Castle { node: $node })
                        .build();
                }
            }
        }

        let node = self.woods.store.new_node();

        create_node!(node);
    }

    fn handle_event(&mut self, data: StateData<'_, GameData<'_, '_>>, event: StateEvent) -> SimpleTrans {
        Trans::None
    }
}


