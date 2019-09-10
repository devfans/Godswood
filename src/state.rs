
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
    renderer::{self, rendy::mesh::*},
    ui,
    utils::{self, scene},
};


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
                Trans::Switch(Box::new(Show { scene: self.prefab.as_ref().unwrap().clone() }))
            }
            assets::Completion::Loading => Trans::None
        }
    }
}

pub struct Show {
    scene: assets::Handle<assets::Prefab<GodsPrefabData>>,
}

impl SimpleState for Show {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        data.world.create_entity().with(self.scene.clone()).build();
        data.world.create_entity().with(self.scene.clone()).build();
    }

    fn handle_event(&mut self, data: StateData<'_, GameData<'_, '_>>, event: StateEvent) -> SimpleTrans {
        Trans::None
    }
}


