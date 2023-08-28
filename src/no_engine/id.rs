use rand::Rng;

#[derive(Default, Debug, Clone, Copy, Hash, Eq, PartialEq)]
pub struct Id(usize);

impl Id {
    pub fn new() -> Id {
        let random_id = rand::thread_rng().gen::<usize>();

        Id(random_id)
    }
}

impl std::fmt::Display for Id {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0.to_string())
    }
}

impl std::ops::Deref for Id {
    type Target = usize;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<usize> for Id {
    fn from(id: usize) -> Self {
        Self(id)
    }
}

impl From<Id> for usize {
    fn from(id: Id) -> Self {
        id.0
    }
}

impl From<u32> for Id {
    fn from(id: u32) -> Self {
        Self(id as usize)
    }
}

impl From<Id> for u32 {
    fn from(id: Id) -> Self {
        id.0 as u32
    }
}

impl From<u64> for Id {
    fn from(id: u64) -> Self {
        Self(id as usize)
    }
}

impl From<Id> for u64 {
    fn from(id: Id) -> Self {
        id.0 as u64
    }
}
