//! 提供物理页的 「`Box`」 [`FrameTracker`]

use crate::memory::{address::*, FRAME_ALLOCATOR};

/// 定义FrameTracker实现物理内存帧的引用
pub struct FrameTracker(pub(super) PhysicalPageNumber);

impl FrameTracker {
    /// 帧的物理地址
    pub fn address(&self) -> PhysicalAddress {
        self.0.into()
    }
    /// 帧的物理页号
    pub fn page_number(&self) -> PhysicalPageNumber {
        self.0
    }
}


/// 析构：帧在释放时放回
impl Drop for FrameTracker {
    fn drop(&mut self) {
        FRAME_ALLOCATOR.lock().dealloc(self);
    }
}