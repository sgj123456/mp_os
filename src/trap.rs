use riscv::{
    interrupt::{Exception, Interrupt, Trap},
    register::{scause, sepc},
    ExceptionNumber, InterruptNumber,
};

use crate::{println, timer};
#[unsafe(naked)]
#[no_mangle]
#[link_section = ".text.trap"]
pub extern "C" fn trap_entry() {
    core::arch::naked_asm!(
        // 保存全部寄存器到内核栈
        "sd ra, 0*8(sp)",
        "sd gp, 1*8(sp)",
        "sd tp, 2*8(sp)",
        "sd t0, 3*8(sp)",
        "sd t1, 4*8(sp)",
        "sd t2, 5*8(sp)",
        // "sd s0, 6*8(sp)",
        "sd s1, 7*8(sp)",
        "sd a0, 8*8(sp)",
        "sd a1, 9*8(sp)",
        "sd a2, 10*8(sp)",
        "sd a3, 11*8(sp)",
        "sd a4, 12*8(sp)",
        "sd a5, 13*8(sp)",
        "sd a6, 14*8(sp)",
        "sd a7, 15*8(sp)",
        "sd s2, 16*8(sp)",
        "sd s3, 17*8(sp)",
        "sd s4, 18*8(sp)",
        "sd s5, 19*8(sp)",
        "sd s6, 20*8(sp)",
        "sd s7, 21*8(sp)",
        "sd s8, 22*8(sp)",
        "sd s9, 23*8(sp)",
        "sd s10, 24*8(sp)",
        "sd s11, 25*8(sp)",
        "sd t3, 26*8(sp)",
        "sd t4, 27*8(sp)",
        "sd t5, 28*8(sp)",
        "sd t6, 29*8(sp)",
        // 调用 Rust 处理函数
        "call trap_handler",
        // 恢复寄存器
        "ld t6, 29*8(sp)",
        "ld t5, 28*8(sp)",
        "ld t4, 27*8(sp)",
        "ld t3, 26*8(sp)",
        "ld s11, 25*8(sp)",
        "ld s10, 24*8(sp)",
        "ld s9, 23*8(sp)",
        "ld s8, 22*8(sp)",
        "ld s7, 21*8(sp)",
        "ld s6, 20*8(sp)",
        "ld s5, 19*8(sp)",
        "ld s4, 18*8(sp)",
        "ld s3, 17*8(sp)",
        "ld s2, 16*8(sp)",
        "ld a7, 15*8(sp)",
        "ld a6, 14*8(sp)",
        "ld a5, 13*8(sp)",
        "ld a4, 12*8(sp)",
        "ld a3, 11*8(sp)",
        "ld a2, 10*8(sp)",
        "ld a1, 9*8(sp)",
        "ld a0, 8*8(sp)",
        "ld s1, 7*8(sp)",
        // "ld s0, 6*8(sp)",
        "ld t2, 5*8(sp)",
        "ld t1, 4*8(sp)",
        "ld t0, 3*8(sp)",
        "ld tp, 2*8(sp)",
        "ld gp, 1*8(sp)",
        "ld ra, 0*8(sp)",
        // 中断返回
        "sret",
    );
}

#[no_mangle]
#[inline(never)]
extern "C" fn trap_handler() {
    let cause = scause::read();
    let pc = sepc::read();
    match cause.cause() {
        Trap::Interrupt(interrupt) => {
            if let Ok(int) = Interrupt::from_number(interrupt) {
                match int {
                    Interrupt::SupervisorTimer => {
                        println!("timer interrupt");
                        timer::set_next_trigger()
                    }
                    Interrupt::SupervisorSoft => {}
                    Interrupt::SupervisorExternal => {}
                }
            }
        }

        Trap::Exception(exception) => {
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
                    _ => false,
                }
            } else {
                false
            };
            if !handled {
                loop {}
            }
        }
    }
}
