mod gltf;

pub use self::gltf::load_gltf;
use std::{
    fmt::Debug,
    hash::{Hash, Hasher},
};

use std::{any::TypeId, collections::HashMap, marker::PhantomData};

pub type HandleId = usize;

pub struct Handle<T> {
    pub id: HandleId,
    marker: PhantomData<T>,
}

impl<T> Handle<T> {
    pub fn new(id: HandleId) -> Self {
        Handle {
            id,
            marker: PhantomData,
        }
    }
}

impl<T> Hash for Handle<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl<T> PartialEq for Handle<T> {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl<T> Eq for Handle<T> {}

impl<T> Debug for Handle<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        let name = std::any::type_name::<T>().split("::").last().unwrap();
        write!(f, "Handle<{}>({})", name, self.id)
    }
}

// TODO: somehow handle this gracefully in asset managers. or alternatively remove Default
impl<T> Default for Handle<T> {
    fn default() -> Self {
        Handle {
            id: std::usize::MAX,
            marker: PhantomData,
        }
    }
}

impl<T> Copy for Handle<T> {}
impl<T> Clone for Handle<T> {
    fn clone(&self) -> Self {
        Handle {
            id: self.id.clone(),
            marker: PhantomData,
        }
    }
}

#[derive(Hash, Copy, Clone, Eq, PartialEq, Debug)]
pub struct HandleUntyped {
    pub id: HandleId,
    pub type_id: TypeId,
}

impl<T> From<Handle<T>> for HandleUntyped
where
    T: 'static,
{
    fn from(handle: Handle<T>) -> Self {
        HandleUntyped {
            id: handle.id,
            type_id: TypeId::of::<T>(),
        }
    }
}

impl<T> From<HandleUntyped> for Handle<T>
where
    T: 'static,
{
    fn from(handle: HandleUntyped) -> Self {
        if TypeId::of::<T>() != handle.type_id {
            panic!("attempted to convert untyped handle to incorrect typed handle");
        }

        Handle::new(handle.id)
    }
}

pub trait Asset<D> {
    fn load(descriptor: D) -> Self;
}

pub struct AssetStorage<T> {
    assets: HashMap<HandleId, T>,
    names: HashMap<String, Handle<T>>,
    current_index: HandleId,
}

impl<T> AssetStorage<T> {
    pub fn new() -> AssetStorage<T> {
        AssetStorage {
            assets: HashMap::new(),
            names: HashMap::new(),
            current_index: 0,
        }
    }

    pub fn get_named(&mut self, name: &str) -> Option<Handle<T>> {
        self.names.get(name).map(|handle| *handle)
    }

    pub fn add(&mut self, asset: T) -> Handle<T> {
        let id = self.current_index;
        self.current_index += 1;
        self.assets.insert(id, asset);
        Handle {
            id,
            marker: PhantomData,
        }
    }

    pub fn set_name(&mut self, name: &str, handle: Handle<T>) {
        self.names.insert(name.to_string(), handle);
    }

    pub fn get_id(&self, id: HandleId) -> Option<&T> {
        self.assets.get(&id)
    }

    pub fn get_id_mut(&mut self, id: HandleId) -> Option<&mut T> {
        self.assets.get_mut(&id)
    }

    pub fn get(&self, handle: &Handle<T>) -> Option<&T> {
        self.assets.get(&handle.id)
    }

    pub fn get_mut(&mut self, handle: &Handle<T>) -> Option<&mut T> {
        self.assets.get_mut(&handle.id)
    }
}
