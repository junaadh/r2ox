use crate::memory::traits::Unit;
use r2proc::{Range, Unit};
use x86_64::{PhysAddr, VirtAddr};

use super::index::PageIndex;

#[derive(Clone, Copy, Unit, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[unit_type(PhysAddr)]
#[repr(transparent)]
pub struct Frame {
    index: PageIndex,
}

#[derive(Clone, Copy, Unit, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[unit_type(VirtAddr)]
#[repr(transparent)]
pub struct Page {
    index: PageIndex,
}

pub struct FrameIterator {
    current: Frame,
    limit: Frame,
}

impl FrameIterator {
    pub fn new(init: &FrameRange) -> Self {
        Self {
            current: init.start,
            limit: init.end,
        }
    }
}

impl Iterator for FrameIterator {
    type Item = Frame;

    fn next(&mut self) -> Option<Self::Item> {
        match self.current > self.limit {
            true => None,
            false => {
                let cur = self.current;
                self.current += 1;
                Some(cur)
            }
        }
    }
}

pub struct PageIterator {
    current: Page,
    limit: Page,
}

impl PageIterator {
    pub fn new(init: &PageRange) -> Self {
        Self {
            current: init.start,
            limit: init.end,
        }
    }
}

impl Iterator for PageIterator {
    type Item = Page;

    fn next(&mut self) -> Option<Self::Item> {
        match self.current > self.limit {
            true => None,
            false => {
                let cur = self.current;
                self.current += 1;
                Some(cur)
            }
        }
    }
}

#[derive(Clone, Copy, Range)]
#[range_types(Frame, FrameIterator)]
pub struct FrameRange {
    start: Frame,
    end: Frame,
}

#[derive(Clone, Copy, Range)]
#[range_types(Page, PageIterator)]
pub struct PageRange {
    start: Page,
    end: Page,
}

#[cfg(test)]
mod tests {
    use alloc::format;

    use crate::memory::{pager::index::PageIndex, traits::Unit};

    use super::Frame;

    #[test]
    fn debug() {
        let frame = Frame::at(PageIndex::new(0));
        assert_eq!(format!("{:?}", frame), "Frame<0>");
    }
}
