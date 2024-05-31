use crate::memory::traits::Unit;
use r2proc::Unit;

use super::index::PageIndex;

#[derive(Clone, Copy, Unit, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct Frame {
    index: PageIndex,
}

#[derive(Clone, Copy, Unit, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct Page {
    index: PageIndex,
}
