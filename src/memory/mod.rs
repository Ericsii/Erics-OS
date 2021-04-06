//! 内存模块

pub mod config;
pub mod heap;

pub use {
    config::*,
};

pub fn init() {
    heap::init();
    println!("Memory initialized.")
}