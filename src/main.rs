#![no_std]
#![no_main]

use core::{
    arch::{asm, global_asm},
    fmt::Display,
    panic::PanicInfo,
};

use rand::{Rng, SeedableRng, rngs::SmallRng};

use crate::tests::instructions;

mod boot;
mod pl011;
mod print;
mod tests;

#[unsafe(no_mangle)]
extern "C" fn kmain() {
    let el = {
        let el: u64;
        unsafe { asm!("mrs {}, CurrentEL", out(reg) el) };
        u8::try_from(el >> 2).unwrap()
    };

    println!("starting (EL{el})");

    run_tests();

    println!("finished");
    loop {}
}

fn run_tests() {
    // //cb23c083 	sub	x3, x4, w3, sxtw
    // run_test(0xcb23c083);

    instructions().iter().for_each(|i| run_test(*i));
}

fn run_test(instruction: u32) {
    let input_ctx = MachineContext::random();
    let mut output_ctx = MachineContext::default();

    println!("running test: {instruction:#010x}");
    println!("input context:\n{input_ctx}");

    unsafe { test_slot = instruction };
    unsafe {
        asm!("ic ivau, x0");
    }
    unsafe { execute_test(&input_ctx, &mut output_ctx) };

    println!("output context:\n{output_ctx}");
}

#[repr(C)]
#[derive(Default, Clone, Copy)]
struct MachineContext {
    gprs: [u64; 31],
    flags: u64,
}

impl MachineContext {
    pub fn random() -> Self {
        let mut small_rng = SmallRng::seed_from_u64(0x1234);

        let mut ctx = Self::default();
        small_rng.fill(&mut ctx.gprs);
        ctx
    }
}

impl Display for MachineContext {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        for (i, value) in self.gprs.iter().enumerate() {
            writeln!(f, "X{i:02}: {value:#018x}")?;
        }

        writeln!(f, "flags: {:#018x}", self.flags)?;

        Ok(())
    }
}

#[panic_handler]
fn panic_handler(info: &PanicInfo) -> ! {
    println!("PANIC: {:?}", info.message());
    loop {}
}

unsafe extern "C" {
    fn execute_test(input_context: &MachineContext, output_context: &mut MachineContext);
    static mut test_slot: u32;
}

global_asm!(
    r#"
.globl execute_test
execute_test:
	// backup callee save regs
	stp x19, x20, [sp, #-16]! // 16 callee
	stp x21, x22, [sp, #-16]! // 32 callee
	stp x23, x24, [sp, #-16]! // 48 callee
	stp x25, x26, [sp, #-16]! // 64 callee
	stp x27, x28, [sp, #-16]! // 80 callee
	stp x29, x30, [sp, #-16]! // 96 callee

	stp x0, x1, [sp, #-16]! // 112 &input &output
	// load input state

	// G-REGS
	ldp x2, x3, [x0, 16 * 1]
	ldp x4, x5, [x0, 16 * 2]
	ldp x6, x7, [x0, 16 * 3]
	ldp x8, x9, [x0, 16 * 4]
	ldp x10, x11, [x0, 16 * 5]
	ldp x12, x13, [x0, 16 * 6]
	ldp x14, x15, [x0, 16 * 7]
	ldp x16, x17, [x0, 16 * 8]
	ldp x18, x19, [x0, 16 * 9]
	ldp x20, x21, [x0, 16 * 10]
	ldp x22, x23, [x0, 16 * 11]
	ldp x24, x25, [x0, 16 * 12]
	ldp x26, x27, [x0, 16 * 13]
	ldp x28, x29, [x0, 16 * 14]
	ldr x30, [x0, 16 * 15]

	// FLAGS
	ldr x1, [x0, 768]
	msr nzcv, x1

	ldp x0, x1, [x0]

// execute test slot
.globl test_slot
test_slot:
	.word 0

	// save state to output
	stp x0, x1, [sp, #-16]! // 128 out_x0 out_x1

	// G-REGS
	ldr x0, [sp, #24]
	stp x2, x3, [x0, 16 * 1]
	stp x4, x5, [x0, 16 * 2]
	stp x6, x7, [x0, 16 * 3]
	stp x8, x9, [x0, 16 * 4]
	stp x10, x11, [x0, 16 * 5]
	stp x12, x13, [x0, 16 * 6]
	stp x14, x15, [x0, 16 * 7]
	stp x16, x17, [x0, 16 * 8]
	stp x18, x19, [x0, 16 * 9]
	stp x20, x21, [x0, 16 * 10]
	stp x22, x23, [x0, 16 * 11]
	stp x24, x25, [x0, 16 * 12]
	stp x26, x27, [x0, 16 * 13]
	stp x28, x29, [x0, 16 * 14]
	str x30,     [x0, 16 * 15]


	mov x2, x0
	ldp x0, x1, [sp], #16 // 112 out_x0 out_x1
	stp x0, x1, [x2]

	// FLAGS
	mrs x0, nzcv
	str x0, [x2, 768]

	add sp, sp, #16

	// restore callee saved regs
	ldp x29, x30, [sp], #16
	ldp x27, x28, [sp], #16
	ldp x25, x26, [sp], #16
	ldp x23, x24, [sp], #16
	ldp x21, x22, [sp], #16
	ldp x19, x20, [sp], #16

	ret

.size execute_test, .-execute_test
    "#
);
