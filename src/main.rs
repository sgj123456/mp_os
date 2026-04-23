#![no_std]
#![no_main]

use log::*;
use riscv::asm::wfi;

#[macro_use]
mod console;
pub mod config;
pub mod lang_items;
pub mod logging;
pub mod task;
pub mod timer;
pub mod trap;

/// clear BSS segment
fn clear_bss() {
    extern "C" {
        fn sbss();
        fn ebss();
    }
    unsafe {
        core::slice::from_raw_parts_mut(
            sbss as *const () as usize as *mut u8,
            ebss as *const () as usize - sbss as *const () as usize,
        )
        .fill(0);
    }
}
extern "C" {
    fn stext(); // begin addr of text segment
    fn etext(); // end addr of text segment
    fn srodata(); // start addr of Read-Only data segment
    fn erodata(); // end addr of Read-Only data ssegment
    fn sdata(); // start addr of data segment
    fn edata(); // end addr of data segment
    fn sbss(); // start addr of BSS segment
    fn ebss(); // end addr of BSS segment
    fn boot_stack_lower_bound(); // stack lower bound
    fn boot_stack_top(); // stack top
}
/// kernel log info
fn kernel_log_info() {
    logging::init();
    println!("[kernel] Hello, world!");
    trace!(
        "[kernel] .text [{:#x}, {:#x})",
        stext as *const () as usize,
        etext as *const () as usize
    );
    debug!(
        "[kernel] .rodata [{:#x}, {:#x})",
        srodata as *const () as usize, erodata as *const () as usize
    );
    info!(
        "[kernel] .data [{:#x}, {:#x})",
        sdata as *const () as usize, edata as *const () as usize
    );
    warn!(
        "[kernel] boot_stack top=bottom={:#x}, lower_bound={:#x}",
        boot_stack_top as *const () as usize, boot_stack_lower_bound as *const () as usize
    );
    error!(
        "[kernel] .bss [{:#x}, {:#x})",
        sbss as *const () as usize, ebss as *const () as usize
    );
}

#[no_mangle]
/// the rust entry-point of os
pub fn rust_main() -> ! {
    clear_bss();
    kernel_log_info();
    trap::init();
    timer::set_next_trigger();
    trap::enable_timer_interrupt();
    println!("[kernel] timer interrupt enabled");
    // task::run_first_task();
    loop {
        wfi();
        // println!("[kernel] wakeup from wfi");
    }
    panic!("Unreachable in rust_main!");
}

#[unsafe(naked)]
#[no_mangle]
#[link_section = ".text.entry"]
pub extern "C" fn _start() {
    core::arch::naked_asm!(
        "lla sp, {boot_stack_top}",
        "j rust_main",
        boot_stack_top = sym boot_stack_top,
    );
}
