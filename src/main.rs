mod animation_transform;
mod area_light;
mod button;
mod cubes;
mod cursor;
mod mesh;
mod pbr;
mod practice;
mod shadow;
mod wire;

use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::pbr::wireframe::WireframePlugin;
use bevy::render::render_resource::WgpuFeatures;
use bevy::render::settings::WgpuSettings;
use bevy::{prelude::*, utils::HashSet};
use rand::{prelude::SliceRandom, Rng};
use std::{
    env::VarError,
    io::{self, BufRead, BufReader},
    process::Stdio,
};

fn main() {
    practice::run();
    // button::run();
    // area_light::run();
    // pbr::run();
    // wire::run();
    // shadow::run();
    // mesh::run();
    // cubes::run();
    // animation_transform::run();
    // App::new()
    //     .add_plugins(DefaultPlugins)
    //     .insert_resource(ClearColor(Color::WHITE))
    //     .add_startup_system(cubes::set_up)
    //     .add_startup_system(area_light::setup)
    //     .add_startup_system(mesh::setup)
    //     .add_startup_system(pbr::setup)
    //     .insert_resource(Msaa { samples: 4 })
    //     .insert_resource(WgpuSettings {
    //         features: WgpuFeatures::POLYGON_MODE_LINE,
    //         ..Default::default()
    //     })
    //     .add_plugin(WireframePlugin)
    //     .add_startup_system(wire::setup)
    //     .run();
}
