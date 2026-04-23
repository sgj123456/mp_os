use riscv::{
    interrupt::{Exception, Interrupt, Trap},
    register::{
        scause, sepc, sie, sscratch, sstatus,
        stvec::{self, Stvec, TrapMode},
    },
    ExceptionNumber, InterruptNumber,
};

extern "C" {
    fn boot_stack_top();
}

pub fn init() {
    unsafe {
        stvec::write(Stvec::new(alltraps as *const () as usize, TrapMode::Direct));
        sscratch::write(boot_stack_top as *const () as usize);
    }
}
pub fn enable_timer_interrupt() {
    unsafe {
        sstatus::set_sie();
        sie::set_stimer();
    }
}
use crate::timer;
use core::arch::{asm, naked_asm};

#[unsafe(naked)]
#[no_mangle]
pub unsafe extern "C" fn alltraps() {
    naked_asm!(
        "csrrw   sp, sscratch, sp",
        "addi    sp, sp, -35*8",
        "sd      x1,     1*8(sp)",
        "sd      x2,     2*8(sp)",
        "sd      x3,     3*8(sp)",
        "sd      x4,     4*8(sp)",
        "sd      x5,     5*8(sp)",
        "sd      x6,     6*8(sp)",
        "sd      x7,     7*8(sp)",
        "sd      x8,     8*8(sp)",
        "sd      x9,     9*8(sp)",
        "sd      x10,   10*8(sp)",
        "sd      x11,   11*8(sp)",
        "sd      x12,   12*8(sp)",
        "sd      x13,   13*8(sp)",
        "sd      x14,   14*8(sp)",
        "sd      x15,   15*8(sp)",
        "sd      x16,   16*8(sp)",
        "sd      x17,   17*8(sp)",
        "sd      x18,   18*8(sp)",
        "sd      x19,   19*8(sp)",
        "sd      x20,   20*8(sp)",
        "sd      x21,   21*8(sp)",
        "sd      x22,   22*8(sp)",
        "sd      x23,   23*8(sp)",
        "sd      x24,   24*8(sp)",
        "sd      x25,   25*8(sp)",
        "sd      x26,   26*8(sp)",
        "sd      x27,   27*8(sp)",
        "sd      x28,   28*8(sp)",
        "sd      x29,   29*8(sp)",
        "sd      x30,   30*8(sp)",
        "sd      x31,   31*8(sp)",
        "csrr    t0,     sstatus",
        "csrr    t1,     sepc",
        "sd      t0,    32*8(sp)",
        "sd      t1,    33*8(sp)",
        "csrr    t2,     sscratch",
        "sd      t2,    34*8(sp)",
        "mv      a0,     sp",
        "call    trap_handler",
        "j       restore",
    );
}

#[unsafe(naked)]
#[no_mangle]
pub unsafe extern "C" fn restore() {
    naked_asm!(
        "ld      t0,    32*8(sp)",
        "ld      t1,    33*8(sp)",
        "ld      t2,    34*8(sp)",
        "csrw    sstatus, t0",
        "csrw    sepc,    t1",
        "csrw    sscratch, t2",
        "ld      x1,     1*8(sp)",
        "ld      x2,     2*8(sp)",
        "ld      x3,     3*8(sp)",
        "ld      x4,     4*8(sp)",
        "ld      x5,     5*8(sp)",
        "ld      x6,     6*8(sp)",
        "ld      x7,     7*8(sp)",
        "ld      x8,     8*8(sp)",
        "ld      x9,     9*8(sp)",
        "ld      x10,   10*8(sp)",
        "ld      x11,   11*8(sp)",
        "ld      x12,   12*8(sp)",
        "ld      x13,   13*8(sp)",
        "ld      x14,   14*8(sp)",
        "ld      x15,   15*8(sp)",
        "ld      x16,   16*8(sp)",
        "ld      x17,   17*8(sp)",
        "ld      x18,   18*8(sp)",
        "ld      x19,   19*8(sp)",
        "ld      x20,   20*8(sp)",
        "ld      x21,   21*8(sp)",
        "ld      x22,   22*8(sp)",
        "ld      x23,   23*8(sp)",
        "ld      x24,   24*8(sp)",
        "ld      x25,   25*8(sp)",
        "ld      x26,   26*8(sp)",
        "ld      x27,   27*8(sp)",
        "ld      x28,   28*8(sp)",
        "ld      x29,   29*8(sp)",
        "ld      x30,   30*8(sp)",
        "ld      x31,   31*8(sp)",
        "addi    sp, sp, 35*8",
        "csrrw   sp, sscratch, sp",
        "sret",
    );
}
#[no_mangle]
#[inline(never)]
extern "C" fn trap_handler() {
    let cause = scause::read();
    let pc = sepc::read();
    println!("[trap] scause={:#x}, sepc={:#x}", cause.bits(), pc);
    match cause.cause() {
        Trap::Interrupt(interrupt) => {
            println!("[trap] interrupt: {:?}", interrupt);
            if let Ok(int) = Interrupt::from_number(interrupt) {
                match int {
                    Interrupt::SupervisorTimer => {
                        println!("[trap] timer interrupt");
                        timer::set_next_trigger()
                    }
                    Interrupt::SupervisorSoft => {}
                    Interrupt::SupervisorExternal => {}
                }
            }
        }

        Trap::Exception(exception) => {
            println!("[trap] exception: {:?}", exception);
            let handled = if let Ok(ex) = Exception::from_number(exception) {
                match ex {
                    Exception::IllegalInstruction => {
                        unsafe {
                            sepc::write(pc + 4);
                        }
                        true
                    }
                    Exception::Breakpoint => {
                        unsafe {
                            sepc::write(pc + 4);
                        }
                        true
                    }
                    Exception::UserEnvCall => {
                        unsafe {
                            sepc::write(pc + 4);
                        }
                        true
                    }
                    _ => {
                        panic!("{ex:?}");
                    }
                }
            } else {
                false
            };
            if !handled {
                panic!("handled!");
            }
        }
    }
}
