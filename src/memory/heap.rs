use super::config::KERNEL_HEAP_SIZE;
use buddy_system_allocator::LockedHeap;

/// 动态内存分配堆空间
/// 
/// 内存大小为 ['KERNEL_HEAP_SIZE']
/// 在编译时会被放到.bss段
static mut HEAP_SPACE: [u8; KERNEL_HEAP_SIZE] = [0; KERNEL_HEAP_SIZE];


/// 堆内存分配器
/// 
/// ### `#[global_allocator]`
/// [`LockedHeap`] 实现了 [`alloc::alloc::GlobalAlloc`] trait，
/// 可以为全局需要用到堆的地方分配空间。例如 `Box` `Arc` 等
#[global_allocator]
static HEAP: LockedHeap = LockedHeap::empty();

/// 初始化堆空间
pub fn init() {
    unsafe {
        HEAP.lock().init(
            HEAP_SPACE.as_ptr() as usize, KERNEL_HEAP_SIZE
        )
    }
}

/// 空间分配错误回调函数
#[alloc_error_handler]
fn alloc_error_handler(_: alloc::alloc::Layout) -> ! {
    panic!("alloc error")
}