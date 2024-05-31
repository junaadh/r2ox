use core::error::Error;

use alloc::boxed::Box;

pub trait Unit {
    type Addr;

    fn new(addr: Self::Addr) -> Self;
    fn at(index: crate::memory::pager::index::PageIndex) -> Self;
    fn addr(self) -> Self::Addr;
    fn inclusive_addr(self) -> Self::Addr;
    fn index(self) -> crate::memory::pager::index::PageIndex;
}

pub trait Range: Clone + Copy {
    type Basic: Unit;
    type Iter;

    fn new(start: Self::Basic, end: Self::Basic) -> Self;
    fn empty() -> Self;
    fn start(self) -> Self::Basic;
    fn end(self) -> Self::Basic;
    fn as_page_size(self) -> usize;
    fn as_bytes(self) -> usize;
    fn is_empty(self) -> bool;
    fn start_addr(self) -> <Self::Basic as Unit>::Addr;
    fn inclusive_addr(self) -> <Self::Basic as Unit>::Addr;
    fn merge(&mut self, other: Self) -> Result<(), Box<dyn Error>>;
    fn overlaps(self, other: Self) -> bool;
    fn consumes(self, other: Self) -> bool;
    fn contains(self, unit: Self::Basic) -> bool;
    fn iter(&self) -> Self::Iter;
}
