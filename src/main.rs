#![no_std]
#![no_main]
mod config;
mod console;
mod lang_items;
mod timer;
mod trap;

use riscv::register::{
    sie, sscratch, sstatus,
    stvec::{self, Stvec, TrapMode},
};

use crate::trap::trap_entry;

extern "C" {
    static __bss_start: u8;
    static __bss_end: u8;
    static __stack_top: u8;
}

#[unsafe(naked)]
#[no_mangle]
#[link_section = ".text.entry"]
pub extern "C" fn _start() {
    core::arch::naked_asm!(
        "lla sp, {stack_top}",
        "lla a0, __bss_start",
        "lla a1, __bss_end",
        "bgeu a0, a1, 2f",
        "1:",
        "sd zero, 0(a0)",
        "addi a0, a0, 8",
        "bltu a0, a1, 1b",
        "2:",
        "j rust_main",
        stack_top = sym __stack_top,
    );
}

#[no_mangle]
pub extern "C" fn rust_main() {
    println!("========================================\n");
    println!("       RISC-V Kernel Booted!\n");
    println!("========================================\n");
    println!("Version: v1.0\n");
    println!("Architecture: RISC-V 64\n");
    println!("Mode: S-Mode\n");

    unsafe {
        sscratch::write(__stack_top as usize);
        stvec::write(Stvec::new(
            trap_entry as *const () as usize,
            TrapMode::Direct,
        ));
        sstatus::set_sie();
        sie::set_stimer();
        timer::set_next_trigger();
    }
    println!("timer init done");

    loop {
        riscv::asm::wfi();
    }
}
