use std::cell::RefCell;

use rand::Rng;

thread_local! {
    static RNG: RefCell<rand::rngs::ThreadRng> = RefCell::new(rand::thread_rng());
}

#[derive(Default, Debug, Clone, Copy, Hash, Eq, PartialEq)]
pub struct Id(usize);

impl Id {
    #[inline(always)]
    pub fn new() -> Id {
        let random_id = RNG.with(|rng| rng.borrow_mut().gen::<usize>());
        Id(random_id)
    }

    #[inline(always)]
    pub fn from(id: usize) -> Id {
        Id(id)
    }

    #[inline(always)]
    pub fn next(&mut self) {
        self.0 += 1;
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
    #[inline(always)]
    fn from(id: usize) -> Self {
        Self(id)
    }
}

impl From<Id> for usize {
    #[inline(always)]
    fn from(id: Id) -> Self {
        id.0
    }
}

impl From<u32> for Id {
    #[inline(always)]
    fn from(id: u32) -> Self {
        Self(id as usize)
    }
}

impl From<Id> for u32 {
    #[inline(always)]
    fn from(id: Id) -> Self {
        id.0 as u32
    }
}

impl From<u64> for Id {
    #[inline(always)]
    fn from(id: u64) -> Self {
        Self(id as usize)
    }
}

impl From<Id> for u64 {
    #[inline(always)]
    fn from(id: Id) -> Self {
        id.0 as u64
    }
}
