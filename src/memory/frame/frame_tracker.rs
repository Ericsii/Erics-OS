//! 提供物理页的 「`Box`」 [`FrameTracker`]

use crate::memory::{address::*, FRAME_ALLOCATOR, PAGE_SIZE};

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

/// `FrameTracker` 可以 deref 得到对应的 `[u8; PAGE_SIZE]`
impl core::ops::Deref for FrameTracker {
    type Target = [u8; PAGE_SIZE];
    fn deref(&self) -> &Self::Target {
        self.page_number().deref_kernel()
    }
}

/// `FrameTracker` 可以 deref 得到对应的 `[u8; PAGE_SIZE]`
impl core::ops::DerefMut for FrameTracker {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.page_number().deref_kernel()
    }
}

/// 析构：帧在释放时放回
impl Drop for FrameTracker {
    fn drop(&mut self) {
        FRAME_ALLOCATOR.lock().dealloc(self);
    }
}