use super::{allocator::Allocator, objects::mesh::Mesh};
use std::sync::Arc;

mod loader;

pub enum ObjectsQueue {
    Mesh(usize),
}

pub struct AssetManager {
    allocator: Allocator,
    loader: loader::ObjectsLoader,
    meshes: Vec<Mesh>,
    assets_to_load: Vec<ObjectsQueue>,
}

impl AssetManager {
    pub fn new(allocator: Allocator) -> Self {
        Self {
            allocator,
            loader: loader::ObjectsLoader::new(),
            meshes: Default::default(),
            assets_to_load: Default::default(),
        }
    }

    pub fn load_file(&mut self, path: &str) {
        let file_extension = path.split('.').last().unwrap();
        match file_extension {
            "obj" => {
                let next_id = self.meshes.len();
                let mesh = self.loader.load_obj_mesh(path, next_id);

                self.meshes.push(mesh);
                self.assets_to_load.push(ObjectsQueue::Mesh(next_id));
            }
            _ => panic!("File extension not supported"),
        }
    }

    pub fn check_upload_queue(&mut self) {
        self.assets_to_load
            .pop()
            .into_iter()
            .for_each(|object_to_load| match object_to_load {
                ObjectsQueue::Mesh(id) => {
                    let mesh = unsafe { self.meshes.get_unchecked_mut(id) };
                    self.allocator.upload_mesh(mesh);
                    mesh.is_loaded = true;
                }
            });
    }
}
