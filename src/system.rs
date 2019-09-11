use crate::resource::ShowState;
use crate::component::GodsNode;

use amethyst:: {
    Error,
    assets,
    core::{
        self,
        SystemDesc,
        math::{UnitQuaternion, Vector3}
    },
    derive::SystemDesc,
    ecs::prelude::{
        Entity, Join, Read, ReadStorage, System, Write, WriteStorage, SystemData,
    },
    input,
    prelude::*,
    renderer::{self, rendy::mesh::*, palette, debug_drawing::{DebugLines, DebugLinesComponent, DebugLinesParams} },
    ui,
    utils::{self, scene},
};


#[derive(Default, SystemDesc)]
pub struct ShowSystem {
    fps_display: Option<Entity>,
}

impl<'a> System<'a> for ShowSystem {
    type SystemData = (
        WriteStorage<'a, renderer::light::Light>,
        Read<'a, core::timing::Time>,
        ReadStorage<'a, renderer::Camera>,
        WriteStorage<'a, core::transform::Transform>,
        Write<'a, ShowState>,
        Write<'a, GodsNode>,
        WriteStorage<'a, ui::UiText>,
        Read<'a, utils::fps_counter::FpsCounter>,
        ui::UiFinder<'a>,
        Write<'a, DebugLines>,
    );
    fn run(&mut self, data: Self::SystemData) {
        let (mut lights, time, camera, mut transforms, mut state, castle, mut ui_text, fps_counter, finder, mut dl) = data;
        let t = (time.absolute_time_seconds() as f32).cos();
        /*
        dl.draw_direction(
            [t, 0.0, 0.5].into(),
            [10.0, 10.3, 100.0].into(),
            palette::Srgba::new(200.5, 200.05, 0.65, 1.0),
        );
        */


        /*
        let light_angular_velocity = -1.0;
        let light_orbit_radius = 15.0;
        let light_z = 6.0;
        let camera_angular_velocity = 1.0;

        state.light_angle += light_angular_velocity * time.delta_seconds();
        state.camera_angle += camera_angular_velocity * time.delta_seconds();

        let delta_rot: UnitQuaternion<f32> = UnitQuaternion::from_axis_angle(
            &Vector3::z_axis(),
            camera_angular_velocity * time.delta_seconds(),
        );

        for (_, transform) in (&camera, &mut transforms).join() {
            *transform.isometry_mut() = delta_rot * transform.isometry();
        }

        for (point_light, transform) in (&mut lights, &mut transforms).join().filter_map(|(light, transform)| {
            if let renderer::light::Light::Point(ref mut point_light) = *light {
                Some((point_light, transform))
            } else {
                None
            }
        }) {
            transform.set_translation_xyz(
                light_orbit_radius * state.light_angle.cos(),
                light_orbit_radius * state.light_angle.sin(),
                light_z,
            );
            point_light.color = state.light_color;
        }

        if self.fps_display.is_none() {
            self.fps_display = finder.find("fps_text");
        }
        if let Some(fps_entity) = self.fps_display {
            if let Some(fps_display) = ui_text.get_mut(fps_entity) {
                if time.frame_number() % 20 == 0 {
                    fps_display.text = format!("FPS: {:.*}", 2, fps_counter.sampled_fps());
                }
            }
        }
        */
    }
}




