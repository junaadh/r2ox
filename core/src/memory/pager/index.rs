use core::ops::{Add, AddAssign, Sub, SubAssign};

use x86_64::{PhysAddr, VirtAddr};

use crate::memory::PAGE_SIZE;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct PageIndex(pub usize);

impl PageIndex {
    #[inline]
    pub fn new(idx: usize) -> Self {
        Self(idx)
    }
}

impl From<PhysAddr> for PageIndex {
    #[inline]
    fn from(value: PhysAddr) -> Self {
        Self(value.as_u64() as usize / PAGE_SIZE)
    }
}

impl From<PageIndex> for PhysAddr {
    #[inline]
    fn from(value: PageIndex) -> Self {
        PhysAddr::new((value.0 * PAGE_SIZE) as u64)
    }
}

impl From<VirtAddr> for PageIndex {
    #[inline]
    fn from(value: VirtAddr) -> Self {
        Self(value.as_u64() as usize / PAGE_SIZE)
    }
}

impl From<PageIndex> for VirtAddr {
    #[inline]
    fn from(value: PageIndex) -> Self {
        VirtAddr::new((value.0 * PAGE_SIZE) as u64)
    }
}

impl Add<usize> for PageIndex {
    type Output = Self;
    fn add(self, rhs: usize) -> Self::Output {
        Self(self.0 + rhs)
    }
}

impl Sub<usize> for PageIndex {
    type Output = Self;
    fn sub(self, rhs: usize) -> Self::Output {
        Self(self.0 - rhs)
    }
}

impl AddAssign<usize> for PageIndex {
    fn add_assign(&mut self, rhs: usize) {
        self.0 += rhs;
    }
}

impl SubAssign<usize> for PageIndex {
    fn sub_assign(&mut self, rhs: usize) {
        self.0 -= rhs;
    }
}
