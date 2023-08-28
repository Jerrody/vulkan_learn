use super::{objects::mesh::Mesh, Id};

mod loader;

pub enum ObjectsQueue {
    Mesh(Id),
}

pub struct AssetManager {
    loader: loader::ObjectsLoader,
    meshes: Vec<Mesh>,
    assets_to_upload: Vec<ObjectsQueue>,
}

impl AssetManager {
    pub fn new() -> Self {
        Self {
            loader: loader::ObjectsLoader::new(),
            meshes: Default::default(),
            assets_to_upload: Default::default(),
        }
    }

    pub fn load_file(&mut self, path: std::path::PathBuf) {
        let file_extension = path
            .extension()
            .expect("File extension not found")
            .to_str()
            .expect("File extension is not a valid UTF-8 string");

        match file_extension {
            "obj" => {
                let id = Id::new();
                let mesh = self.loader.load_obj_mesh(path, id);
                if let Some(mesh) = mesh {
                    self.meshes.push(mesh);
                    self.assets_to_upload.push(ObjectsQueue::Mesh(id));
                }
            }
            _ => panic!("File extension not supported"),
        }
    }

    #[inline(always)]
    pub fn get_mesh(&self, id: Id) -> &Mesh {
        unsafe { self.meshes.get_unchecked::<usize>(id.into()) }
    }

    #[inline(always)]
    pub fn get_assets_to_upload(&mut self) -> Vec<ObjectsQueue> {
        std::mem::take(&mut self.assets_to_upload)
    }
}
