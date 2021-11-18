use bevy::ecs::query::ChangedFetch;
use bevy::ecs::query::ReadFetch;
use bevy::prelude::*;
use bevy::reflect::TypeUuid;

use nalgebra::Vector2;
use nalgebra::Vector3;

macro_rules! new {
	() => {
		pub fn new() -> Self {
			Self::default()
		}
	};
}

#[derive(Debug, Default)]
pub struct MeshBuilder {
	positions: Vec<Vector3<f32>>,
	normals: Vec<Vector3<f32>>,
	uvs: Vec<Vector2<f32>>,
	indices: Vec<u32>,
}

impl MeshBuilder {
	fn new(
		positions: Vec<Vector3<f32>>,
		normals: Vec<Vector3<f32>>,
		uvs: Vec<Vector2<f32>>,
		indices: Vec<u32>,
	) -> Self {
		Self {
			positions,
			normals,
			uvs,
			indices,
		}
	}

	fn append(&mut self, other: &Self) {
		self.extend(
			other.positions.iter().copied(),
			other.normals.iter().copied(),
			other.uvs.iter().copied(),
			other.indices.iter().copied(),
		)
	}
	fn extend(
		&mut self,
		positions: impl IntoIterator<Item = Vector3<f32>>,
		normals: impl IntoIterator<Item = Vector3<f32>>,
		uvs: impl IntoIterator<Item = Vector2<f32>>,
		indices: impl IntoIterator<Item = u32>,
	) {
		let indices_count = self.indices.len() as u32;

		self.positions.extend(positions);
		self.normals.extend(normals);
		self.uvs.extend(uvs);
		self.indices
			.extend(indices.into_iter().map(|index| index + indices_count));
	}
	fn finish(&self) -> Mesh {
		let mut mesh = Mesh::new(Default::default());

		mesh.set_attribute(Mesh::ATTRIBUTE_POSITION, self.positions.iter().copied().map(Into::into).collect::<Vec<[f32; 3]>>());
		mesh.set_attribute(Mesh::ATTRIBUTE_NORMAL, self.normals.iter().copied().map(Into::into).collect::<Vec<[f32; 3]>>());
		mesh.set_attribute(Mesh::ATTRIBUTE_UV_0, self.uvs.iter().copied().map(Into::into).collect::<Vec<[f32; 2]>>());
		mesh.set_indices(Some(bevy::render::mesh::Indices::U32(self.indices.clone())));


		println!("{:?}", mesh);

		mesh
	}
}

pub mod block {
	use bevy::reflect::TypeUuid;

	use nalgebra::Vector2;
	use nalgebra::Vector3;

	use super::MeshBuilder;

	#[derive(Default)]
	pub struct RawMesh {
		up: MeshBuilder,
		down: MeshBuilder,
		left: MeshBuilder,
		right: MeshBuilder,
		forwards: MeshBuilder,
		backwards: MeshBuilder,
	}

	impl RawMesh {
		new! {}

		pub fn load(path: &str) -> Self {
			let file = std::fs::read_to_string(path).unwrap();
			let objects = wavefront_obj::obj::parse(file).unwrap().objects;

			fn convert(object: &wavefront_obj::obj::Object) -> MeshBuilder {
				let mut builder = MeshBuilder::default();

				builder
					.extend(
						object.vertices.iter().map(|v| Vector3::new(v.x, v.y, v.z).cast()),
						object.normals.iter().flat_map(|v| [Vector3::new(v.x, v.y, v.y).cast(); 4]),
						object.tex_vertices.iter().map(|v| Vector2::new(v.u, v.v).cast()),
						object.geometry.iter().flat_map(|geometry| &geometry.shapes).flat_map(|shape| match shape.primitive {
							wavefront_obj::obj::Primitive::Triangle(x, y, z) => [x.0 as u32, y.0  as u32, z.0 as u32],
							_ => panic!()
						})
					);

				builder
			}

			let up = convert(objects.iter().find(|object| object.name == "up").unwrap());
			let down = convert(objects.iter().find(|object| object.name == "down").unwrap());
			let left = convert(objects.iter().find(|object| object.name == "left").unwrap());
			let right = convert(objects.iter().find(|object| object.name == "right").unwrap());
			let forwards = convert(objects.iter().find(|object| object.name == "forwards").unwrap());
			let backwards = convert(objects.iter().find(|object| object.name == "backwards").unwrap());

			Self { up, down, left, right, forwards, backwards }
		}
	}

	#[derive(Default)]
	pub struct Global {
		pub mesh: RawMesh,
	}

	#[derive(Debug, Default)]
	pub struct Local {
		pub id: usize,
		pub solid: bool,
	}

	pub fn mesh_into(local: &Local, global: &Global, position: Vector3<f32>, builder: &mut MeshBuilder) {
		fn mesh_into(this: &MeshBuilder, local: &Local, position: Vector3<f32>, builder: &mut MeshBuilder) {
			builder.extend(
				this.positions.iter().copied().map(|relative| relative + position),
				this.normals.iter().copied(),
				this.uvs.iter().copied(),
				this.indices.iter().copied(),
			)
		}

		if local.solid {
			mesh_into(&global.mesh.up, local, position, builder);
			mesh_into(&global.mesh.down, local, position, builder);
			mesh_into(&global.mesh.left, local, position, builder);
			mesh_into(&global.mesh.right, local, position, builder);
			mesh_into(&global.mesh.forwards, local, position, builder);
			mesh_into(&global.mesh.backwards, local, position, builder);
		}
	}

	impl Local {
		new! {}
	}

	#[derive(Default, TypeUuid)]
	#[uuid = "1febd719-fb3f-4b88-bab8-3acea8e1970b"]
	pub struct Globals {
		pub globals: Vec<Global>,
	}

	impl Globals {
		new! {}

		pub fn add(&mut self, global: Global) {
			self.globals.push(global)
		}
	}
}

// #[derive(Default, TypeUuid)]
// #[uuid="bc56e797-692b-4412-80d0-133d2d6a212b"]
// struct ChunkIndex {
// 	chunks: HashMap<[i32; 3], Entity>,
// }

#[derive(Debug, Default)]
pub struct Blocks {
	blocks: [[[block::Local; 16]; 16]; 16],
}

impl Blocks {
	new! {}

	pub fn mesh(&self, globals: &block::Globals) -> Mesh {
		let mut builder = MeshBuilder::default();

		for z in 0..16 {
			for y in 0..16 {
				for x in 0..16 {
					let local = &self.blocks[x][y][z];

					block::mesh_into(&local, globals.globals.get(local.id).unwrap(), Vector3::new(x, y, z).cast(), &mut builder)
				}
			}
		}

		builder.finish()
	}
	pub fn update_meshes(
		mut meshes: ResMut<Assets<Mesh>>,
		global_block_data: Res<block::Globals>,
		query: Query<(&Handle<Mesh>, &Blocks), Changed<Blocks>>,
	) {
		for (mesh_handle, blocks) in query.iter() {
			meshes.set(mesh_handle, blocks.mesh(&*global_block_data));
		}
	}
	pub fn get_mut(&mut self, [x, y, z]: [usize; 3]) -> Option<&mut block::Local> {
		self.blocks.get_mut(z)?.get_mut(y)?.get_mut(x)
	}
}

#[derive(Default, Bundle)]
pub struct ChunkBundle {
	pub blocks: Blocks,

	#[bundle]
	pub pbr: PbrBundle,
}

impl ChunkBundle {
	new! {}
}

#[derive(Default)]
pub struct ChunkPlugin;

impl ChunkPlugin {
	new! {}
}

impl Plugin for ChunkPlugin {
	fn build(&self, app: &mut AppBuilder) {
		app.init_resource::<block::Globals>()
			.add_system(Blocks::update_meshes.system());
	}
}
