use alloc::vec::Vec;
use arrayvec::ArrayVec;
use buddy_system_allocator::LockedHeap;

#[global_allocator]
pub static GLOBAL_ALLOCATOR: LockedHeap<32> = LockedHeap::new();

#[derive(Clone)]
pub enum Lists<T: Clone, const MAX: usize> {
    StaticList(ArrayVec<T, MAX>),
    Vec(Vec<T>),
}

impl<T: Clone, const MAX: usize> Lists<T, MAX> {
    pub const fn new_static() -> Self {
        Self::StaticList(ArrayVec::<T, MAX>::new_const())
    }

    pub const fn new_vec() -> Self {
        Self::Vec(Vec::<T>::new())
    }

    pub fn push(&mut self, item: T) {
        match self {
            Self::StaticList(a) => a.push(item),
            Self::Vec(a) => a.push(item),
        }
    }

    pub fn remove(&mut self, index: usize) -> T {
        match self {
            Self::StaticList(a) => a.remove(index),
            Self::Vec(a) => a.remove(index),
        }
    }

    pub fn to_vec(&mut self) {
        match self {
            Self::StaticList(l) => *self = Self::Vec(l.to_vec()),
            Self::Vec(_v) => {}
        }
    }
}
