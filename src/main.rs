#![allow(unused)]

mod chunk;
mod mesh;

use bevy::prelude::*;
use bevy::reflect::TypeUuid;
use bevy::render::mesh::Indices;
use bevy::render::mesh::VertexAttributeValues;
use bevy::utils::HashMap;

use core::ops::Range;

use nalgebra::Matrix4;
use nalgebra::Rotation3;
use nalgebra::Vector2;
use nalgebra::Vector3;

fn setup(
	mut commands: Commands,
	mut meshes: ResMut<Assets<Mesh>>,
	mut materials: ResMut<Assets<StandardMaterial>>,
	mut globals: ResMut<chunk::block::Globals>,
) {
	globals.add(chunk::block::Global {
		mesh: chunk::block::RawMesh::load("assets/debug/cube.obj"),
	});

	commands.spawn_bundle(chunk::ChunkBundle {
		blocks: {
			let mut blocks = chunk::Blocks::default();

 			for i in 0..16 {
 				blocks.get_mut([i; 3]).unwrap().solid = true;
 			}

			blocks
		},
		pbr: PbrBundle {
			material: materials.add(Color::rgb(0.1, 2.0, 0.06).into()),
			..Default::default()
		},
	});

// 	commands.spawn_bundle(PbrBundle {
// 		mesh: meshes.add({
// 			let mut blocks = chunk::Blocks::default();
//
// // 			for i in 0..16 {
// // 				blocks.get_mut([i; 3]).unwrap().solid = true;
// // 			}
//
//  			blocks.get_mut([0, 0, 0]).unwrap().solid = true;
//
// 			blocks.mesh(&*globals)
// 		}),
// 		material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
// 		..Default::default()
// 	});

	// plane
	commands.spawn_bundle(PbrBundle {
		mesh: meshes.add(Mesh::from(shape::Plane { size: 5.0 })),
		material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
		..Default::default()
	});

	// light
	commands.spawn_bundle(LightBundle {
		transform: Transform::from_xyz(4.0, 8.0, 4.0),
		..Default::default()
	});
}

fn main() {
	App::build()
		.add_plugins(DefaultPlugins)
		.add_plugin(bevy_console::ConsolePlugin)
		.add_plugin(bevy_flycam::PlayerPlugin)
		.add_plugin(chunk::ChunkPlugin)
		.insert_resource(bevy_console::ConsoleConfiguration {
			keys: vec![bevy_console::ToggleConsoleKey::KeyCode(KeyCode::F1)],
			..Default::default()
		})
		.add_startup_system(setup.system())
		.run();
}
