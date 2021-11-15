mod mesh;

use bevy::prelude::*;
use bevy::render::mesh::Indices;

use nalgebra::Vector3;

#[derive(Debug, Default)]
struct MeshBuilder {
	positions: Vec<[f32; 3]>,
	normals: Vec<[f32; 3]>,
	uvs: Vec<[f32; 2]>,
	indices: Vec<u32>,
}

impl MeshBuilder {
	pub fn append<P, N, U, I>(&mut self, positions: P, normals: N, uvs: U, indices: I)
	where
		P: IntoIterator<Item = [f32; 3]>,
		N: IntoIterator<Item = [f32; 3]>,
		U: IntoIterator<Item = [f32; 2]>,
		I: IntoIterator<Item = u32>,
	{
		let vertices_count = self.positions.len() as u32;
		let indices = indices.into_iter().map(|index| vertices_count + index);

		self.positions.extend(positions);
		self.normals.extend(normals);
		self.uvs.extend(uvs);
		self.indices.extend(indices);
	}
}

struct Block {
	solid: bool,
}

impl Block {
	fn mesh_into(&self, builder: &mut MeshBuilder, block_position: Vector3<f32>) {
		let indices = [0, 1, 2, 1, 3, 2];

		#[rustfmt::skip]
		let faces = [
			(
				[0.0, 1.0, 0.0],
				[
					[0.0, 0.0],
					[1.0, 1.0],
					[0.0, 0.0],
					[1.0, 1.0],
				],
				[
					[1.0, 1.0, 1.0],
					[0.0, 1.0, 1.0],
					[1.0, 1.0, 0.0],
					[0.0, 1.0, 0.0],
				],
			),
			(
				[0.0, -1.0, 0.0],
				[
					[0.0, 0.0],
					[1.0, 1.0],
					[0.0, 0.0],
					[1.0, 1.0],
				],
				[
					[0.0, 0.0, 1.0],
					[1.0, 0.0, 1.0],
					[0.0, 0.0, 0.0],
					[1.0, 0.0, 0.0],
				]
			),
			(
				[0.0, 0.0, 1.0],
				[
					[0.0, 0.0],
					[1.0, 1.0],
					[0.0, 0.0],
					[1.0, 1.0],
				],
				[
					[1.0, 0.0, 1.0],
					[0.0, 0.0, 1.0],
					[1.0, 1.0, 1.0],
					[0.0, 1.0, 1.0],
				],
			),
			(
				[0.0, 0.0, -1.0],
				[
					[0.0, 0.0],
					[1.0, 1.0],
					[0.0, 0.0],
					[1.0, 1.0],
				],
				[
					[0.0, 0.0, 0.0],
					[1.0, 0.0, 0.0],
					[0.0, 1.0, 0.0],
					[1.0, 1.0, 0.0],
				],
			),
			(
				[1.0, 0.0, 0.0],
				[
					[0.0, 0.0],
					[1.0, 1.0],
					[0.0, 0.0],
					[1.0, 1.0],
				],
				[
					[1.0, 0.0, 1.0],
					[1.0, 1.0, 1.0],
					[1.0, 0.0, 0.0],
					[1.0, 1.0, 0.0],
				],
			),
			(
				[-1.0, 0.0, 0.0],
				[
					[0.0, 0.0],
					[1.0, 1.0],
					[0.0, 0.0],
					[1.0, 1.0],
				],
				[
					[0.0, 1.0, 1.0],
					[0.0, 0.0, 1.0],
					[0.0, 1.0, 0.0],
					[0.0, 0.0, 0.0],
				],
			),
		];

		for &(normals, uvs, vertices) in &faces {
			let vertices = vertices.iter().map(|vertex_position| (Vector3::from(*vertex_position) + block_position.cast()).into());
			let indices = indices.iter().copied();

			builder.append(vertices, [normals; 4], uvs, indices);
		}
	}
}

struct Chunk {
	data: [[[Block; 16]; 16]; 16]
}

fn add_debug_mesh(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<StandardMaterial>>) {
	let mut mesh_builder = MeshBuilder::default();

	for i in 0..16 {
		Block { solid: true }.mesh_into(&mut mesh_builder, Vector3::new(i as f32, i as f32, i as f32));
	}

	let mut mesh = Mesh::new(Default::default());

	mesh.set_attribute(Mesh::ATTRIBUTE_POSITION, mesh_builder.positions);
	mesh.set_attribute(Mesh::ATTRIBUTE_NORMAL, mesh_builder.normals);
	mesh.set_attribute(Mesh::ATTRIBUTE_UV_0, mesh_builder.uvs);

	mesh.set_indices(Some(Indices::U32(mesh_builder.indices)));
	// plane
	commands.spawn_bundle(PbrBundle {
		mesh: meshes.add(Mesh::from(shape::Plane { size: 5.0 })),
		material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
		..Default::default()
	});
	// cube
	commands.spawn_bundle(PbrBundle {
		mesh: meshes.add(mesh),
		material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
		transform: Transform::from_xyz(0.0, 0.5, 0.0),
		..Default::default()
	});
	// light
	commands.spawn_bundle(LightBundle {
		transform: Transform::from_xyz(4.0, 8.0, 4.0),
		..Default::default()
	});
// 	// camera
// 	commands.spawn_bundle(PerspectiveCameraBundle {
// 		transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
// 		..Default::default()
// 	});
}

fn main() {
	App::build()
		.add_plugins(DefaultPlugins)
		.add_plugin(bevy_console::ConsolePlugin)
		.add_plugin(bevy_flycam::PlayerPlugin)
		.insert_resource(bevy_console::ConsoleConfiguration {
			keys: vec![bevy_console::ToggleConsoleKey::KeyCode(KeyCode::F1)],
			..Default::default()
		})
		.add_startup_system(add_debug_mesh.system())
		.run();
}
