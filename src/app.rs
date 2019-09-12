use crate::tree::*;
use crate::state;
use crate::system::ShowSystem;
use crate::state::GodsPrefabData;
use serde_json::Value;

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
    renderer::{self, rendy::mesh::*, debug_drawing, plugins },
    ui,
    utils::{self, scene},
};


pub fn run(raw: &Value) -> Result<(), Error> {
    amethyst::start_logger(Default::default());
    let app_root = utils::application_root_dir()?;

    let asset_dir = app_root.join("assets");
    let display_config_path = app_root.join("config/display.ron");
    let game_data = GameDataBuilder::default()
        .with_system_desc(assets::PrefabLoaderSystemDesc::<GodsPrefabData>::default(), "", &[])
        .with(ShowSystem::default(), "show_system", &[])
        .with_bundle(utils::fps_counter::FpsCounterBundle::default())?
        .with_bundle(
            input::InputBundle::<input::StringBindings>::new().with_bindings_from_file(app_root.join("config/input.ron"))?,
        )?
        .with_bundle(core::transform::TransformBundle::new())?
        .with_bundle(ui::UiBundle::<input::StringBindings>::new())?
        .with_bundle(renderer::RenderingBundle::<renderer::types::DefaultBackend>::new()
                     .with_plugin(renderer::plugins::RenderToWindow::from_config_path(display_config_path)
                                  .with_clear([0.01, 0.03, 0.03, 1.0]))
                     .with_plugin(renderer::plugins::RenderShaded3D::default())
                     .with_plugin(ui::RenderUi::default())
                     .with_plugin(plugins::RenderDebugLines::default())
                     .with_plugin(plugins::RenderSkybox::default())
        )?;

    let mut game = Application::build(asset_dir, state::Loading::new(raw))?.build(game_data)?;
    game.run();
    Ok(())
}

