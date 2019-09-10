
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


pub struct ShowState {
    pub light_angle: f32,
    pub light_color: renderer::palette::Srgb,
    pub ambient_light: bool,
    pub point_light: bool,
    pub directional_light: bool,
    pub camera_angle: f32,
}

impl Default for ShowState {
    fn default() -> Self {
        ShowState {
            light_angle: 0.0,
            light_color: renderer::palette::Srgb::new(1.0, 1.0, 1.0),
            ambient_light: true,
            point_light: true,
            directional_light: true,
            camera_angle: 0.0
        }
    }
}
