pub trait Unit {
    type Addr;

    fn new(addr: Self::Addr) -> Self;
    fn at(index: crate::memory::pager::index::PageIndex) -> Self;
    fn addr(self) -> Self::Addr;
    fn inclusive_addr(self) -> Self::Addr;
    fn index(self) -> crate::memory::pager::index::PageIndex;
}

pub trait Range {}
