use core::{arch::global_asm, slice};
pub fn instructions() -> &'static [u32] {
    let start_ptr = unsafe { &FLOAT_INSTRUCTION_START } as *const u32;
    let end_ptr = unsafe { &FLOAT_INSTRUCTION_END } as *const u32;
    let length_bytes = (end_ptr as usize) - (start_ptr as usize);
    let count = length_bytes / 4;
    unsafe { slice::from_raw_parts(start_ptr, count) }
}

unsafe extern "C" {
    static FLOAT_INSTRUCTION_START: u32;
    static FLOAT_INSTRUCTION_END: u32;
}

global_asm!(
    r#"
.data
.globl FLOAT_INSTRUCTION_START
FLOAT_INSTRUCTION_START:
ucvtf s7, w5
ucvtf s9, w7
ucvtf s17, w18
ucvtf s10, w12
ucvtf s9, w22
ucvtf s26, w13
ucvtf s18, w6
ucvtf s3, w5
ucvtf s28, w29
ucvtf s20, w7
ucvtf d27, w1
ucvtf d7, w28
ucvtf d22, w2
ucvtf d14, w19
ucvtf d19, w30
ucvtf d9, w22
ucvtf d6, w13
ucvtf d13, w22
ucvtf d17, w4
ucvtf d3, w17
ucvtf s5, x30
ucvtf s5, x16
ucvtf s24, x26
ucvtf s30, x1
ucvtf s11, x24
ucvtf s13, x2
ucvtf s27, x20
ucvtf s20, x21
ucvtf s14, x0
ucvtf s20, x1
ucvtf d1, x21
ucvtf d28, x4
ucvtf d16, x18
ucvtf d30, x2
ucvtf d1, x25
ucvtf d28, x12
ucvtf d30, x15
ucvtf d30, x26
ucvtf d10, x15
ucvtf d11, x0
scvtf s15, w30
scvtf s28, w0
scvtf s12, w0
scvtf s27, w9
scvtf s18, w28
scvtf s28, w0
scvtf s12, w1
scvtf s27, w18
scvtf s2, w27
scvtf s30, w7
scvtf d27, w4
scvtf d10, w11
scvtf d27, w15
scvtf d17, w26
scvtf d5, w27
scvtf d28, w2
scvtf d21, w26
scvtf d30, w13
scvtf d9, w19
scvtf d28, w27
scvtf s8, x29
scvtf s15, x0
scvtf s17, x28
scvtf s17, x15
scvtf s5, x2
scvtf s23, x8
scvtf s9, x7
scvtf s26, x24
scvtf s10, x21
scvtf s11, x18
scvtf d8, x12
scvtf d4, x12
scvtf d31, x19
scvtf d30, x1
scvtf d28, x15
scvtf d4, x22
scvtf d20, x19
scvtf d26, x8
scvtf d30, x3
scvtf d25, x5
fcvtzs w9, s1
fcvtzs w9, s1
fcvtzs w9, s1
fcvtzs w9, s1
fcvtzs w9, s1
fcvtzs w9, s1
fcvtzs w9, s1
fcvtzs w9, s1
fcvtzs w9, s1
fcvtzs w9, s1
fcvtzs x9, s1
fcvtzs x9, s1
fcvtzs x9, s1
fcvtzs x9, s1
fcvtzs x9, s1
fcvtzs x9, s1
fcvtzs x9, s1
fcvtzs x9, s1
fcvtzs x9, s1
fcvtzs x9, s1
fcvtzu w9, s1
fcvtzu w9, s1
fcvtzu w9, s1
fcvtzu w9, s1
fcvtzu w9, s1
fcvtzu w9, s1
fcvtzu w9, s1
fcvtzu w9, s1
fcvtzu w9, s1
fcvtzu w9, s1
fcvtzu x9, s1
fcvtzu x9, s1
fcvtzu x9, s1
fcvtzu x9, s1
fcvtzu x9, s1
fcvtzu x9, s1
fcvtzu x9, s1
fcvtzu x9, s1
fcvtzu x9, s1
fcvtzu x9, s1
fcvtzs w9, d1
fcvtzs w9, d1
fcvtzs w9, d1
fcvtzs w9, d1
fcvtzs w9, d1
fcvtzs w9, d1
fcvtzs w9, d1
fcvtzs w9, d1
fcvtzs w9, d1
fcvtzs w9, d1
fcvtzs x9, d1
fcvtzs x9, d1
fcvtzs x9, d1
fcvtzs x9, d1
fcvtzs x9, d1
fcvtzs x9, d1
fcvtzs x9, d1
fcvtzs x9, d1
fcvtzs x9, d1
fcvtzs x9, d1
fcvtzu w9, d1
fcvtzu w9, d1
fcvtzu w9, d1
fcvtzu w9, d1
fcvtzu w9, d1
fcvtzu w9, d1
fcvtzu w9, d1
fcvtzu w9, d1
fcvtzu w9, d1
fcvtzu w9, d1
fcvtzu x9, d1
fcvtzu x9, d1
fcvtzu x9, d1
fcvtzu x9, d1
fcvtzu x9, d1
fcvtzu x9, d1
fcvtzu x9, d1
fcvtzu x9, d1
fcvtzu x9, d1
fcvtzu x9, d1
fabs s27, s16
fabs s4, s4
fabs s17, s30
fabs s9, s28
fabs s10, s20
fabs s12, s18
fabs s27, s16
fabs s15, s27
fabs s19, s13
fabs s3, s16
fabs d8, d7
fabs d21, d28
fabs d27, d15
fabs d2, d25
fabs d26, d27
fabs d29, d21
fabs d11, d1
fabs d26, d28
fabs d31, d3
fabs d24, d9
fadd s24, s4, s27
fadd s19, s21, s10
fadd s14, s8, s24
fadd s18, s24, s0
fadd s25, s13, s29
fadd s20, s28, s31
fadd s14, s23, s26
fadd s11, s12, s5
fadd s12, s6, s1
fadd s11, s10, s26
fadd d21, d2, d30
fadd d16, d21, d19
fadd d27, d4, d28
fadd d19, d22, d20
fadd d19, d15, d2
fadd d16, d4, d30
fadd d15, d18, d21
fadd d9, d29, d2
fadd d14, d9, d8
fadd d16, d20, d18
fccmp s10, s9, #4, eq
fccmp s26, s10, #12, ne
fccmp s14, s24, #8, eq
fccmp s12, s27, #3, eq
fccmp s12, s23, #13, ne
fccmp s9, s2, #5, eq
fccmp s4, s19, #15, ne
fccmp s3, s4, #15, ne
fccmp s13, s20, #6, ne
fccmp s30, s18, #12, eq
fccmpe s10, s4, #0, eq
fccmpe s0, s3, #5, eq
fccmpe s27, s18, #7, eq
fccmpe s20, s12, #11, ne
fccmpe s0, s26, #6, ne
fccmpe s30, s5, #1, eq
fccmpe s25, s7, #3, ne
fccmpe s25, s16, #3, ne
fccmpe s20, s19, #9, eq
fccmpe s23, s30, #0, eq
fccmp d16, d8, #6, ne
fccmp d20, d1, #14, eq
fccmp d28, d4, #8, eq
fccmp d9, d9, #6, ne
fccmp d16, d26, #10, ne
fccmp d10, d30, #12, eq
fccmp d17, d5, #3, eq
fccmp d4, d19, #10, eq
fccmp d27, d17, #9, eq
fccmp d18, d23, #4, eq
fccmpe d27, d28, #9, ne
fccmpe d5, d15, #8, ne
fccmpe d9, d2, #14, ne
fccmpe d0, d10, #2, eq
fccmpe d16, d5, #10, eq
fccmpe d24, d21, #8, eq
fccmpe d6, d2, #4, eq
fccmpe d25, d8, #7, ne
fccmpe d5, d16, #10, eq
fccmpe d31, d2, #0, ne
fcmp s4, s30
fcmp s28, s5
fcmp s9, s14
fcmp s23, s25
fcmp s19, s17
fcmp s13, s12
fcmp s6, s21
fcmp s0, s12
fcmp s23, s4
fcmp s5, s17
fcmp s12, #0.0
fcmp s12, #0.0
fcmp s6, #0.0
fcmp s17, #0.0
fcmp s28, #0.0
fcmp s0, #0.0
fcmp s28, #0.0
fcmp s27, #0.0
fcmp s2, #0.0
fcmp s28, #0.0
fcmp d4, d6
fcmp d26, d1
fcmp d11, d3
fcmp d15, d2
fcmp d28, d3
fcmp d20, d9
fcmp d15, d26
fcmp d31, d15
fcmp d7, d22
fcmp d19, d12
fcmp d7, #0.0
fcmp d31, #0.0
fcmp d24, #0.0
fcmp d13, #0.0
fcmp d17, #0.0
fcmp d20, #0.0
fcmp d13, #0.0
fcmp d13, #0.0
fcmp d15, #0.0
fcmp d15, #0.0
fcmpe s9, s20
fcmpe s22, s3
fcmpe s21, s1
fcmpe s7, s4
fcmpe s4, s3
fcmpe s7, s24
fcmpe s13, s22
fcmpe s18, s12
fcmpe s5, s25
fcmpe s2, s24
fcmpe s5, #0.0
fcmpe s10, #0.0
fcmpe s24, #0.0
fcmpe s29, #0.0
fcmpe s23, #0.0
fcmpe s9, #0.0
fcmpe s17, #0.0
fcmpe s5, #0.0
fcmpe s22, #0.0
fcmpe s1, #0.0
fcmpe d20, d31
fcmpe d21, d10
fcmpe d2, d10
fcmpe d12, d9
fcmpe d14, d16
fcmpe d13, d22
fcmpe d8, d26
fcmpe d12, d26
fcmpe d6, d18
fcmpe d20, d8
fcmpe d10, #0.0
fcmpe d25, #0.0
fcmpe d18, #0.0
fcmpe d2, #0.0
fcmpe d23, #0.0
fcmpe d10, #0.0
fcmpe d11, #0.0
fcmpe d8, #0.0
fcmpe d15, #0.0
fcmpe d1, #0.0
fcsel s9, s3, s0, eq
fcsel s14, s3, s8, eq
fcsel s12, s23, s10, ne
fcsel s13, s18, s19, ne
fcsel s12, s25, s11, eq
fcsel s2, s22, s26, eq
fcsel s24, s17, s30, eq
fcsel s25, s13, s5, ne
fcsel s17, s6, s1, ne
fcsel s9, s10, s25, ne
fcsel d1, d3, d15, eq
fcsel d21, d2, d7, ne
fcsel d28, d19, d2, eq
fcsel d9, d28, d18, ne
fcsel d13, d17, d5, eq
fcsel d30, d11, d9, ne
fcsel d17, d11, d14, eq
fcsel d21, d7, d15, eq
fcsel d10, d30, d4, ne
fcsel d1, d11, d1, ne
fcvtzs w11, s3
fcvtzs w5, s7
fcvtzs w5, s13
fcvtzs w10, s12
fcvtzs w4, s14
fcvtzs w13, s29
fcvtzs w22, s28
fcvtzs w3, s10
fcvtzs w0, s27
fcvtzs w17, s28
fcvtzs x18, s20
fcvtzs x5, s13
fcvtzs x8, s22
fcvtzs x12, s19
fcvtzs x10, s13
fcvtzs x7, s0
fcvtzs x26, s11
fcvtzs x5, s15
fcvtzs x21, s16
fcvtzs x26, s23
fcvtzu w25, s14
fcvtzu w3, s24
fcvtzu w11, s1
fcvtzu w4, s17
fcvtzu w9, s7
fcvtzu w20, s30
fcvtzu w6, s31
fcvtzu w8, s14
fcvtzu w4, s24
fcvtzu w3, s22
fcvtzu x6, s18
fcvtzu x1, s22
fcvtzu x19, s30
fcvtzu x29, s22
fcvtzu x20, s1
fcvtzu x4, s13
fcvtzu x1, s2
fcvtzu x29, s25
fcvtzu x10, s9
fcvtzu x21, s31
fcvtzs w21, d23
fcvtzs w12, d12
fcvtzs w0, d9
fcvtzs w8, d11
fcvtzs w6, d27
fcvtzs w14, d7
fcvtzs w18, d24
fcvtzs w15, d11
fcvtzs w19, d2
fcvtzs w14, d4
fcvtzs x5, d15
fcvtzs x23, d18
fcvtzs x22, d24
fcvtzs x14, d21
fcvtzs x21, d21
fcvtzs x15, d17
fcvtzs x10, d19
fcvtzs x4, d2
fcvtzs x12, d23
fcvtzs x10, d30
fcvtzu w20, d15
fcvtzu w1, d0
fcvtzu w8, d2
fcvtzu w10, d29
fcvtzu w25, d13
fcvtzu w2, d8
fcvtzu w10, d19
fcvtzu w2, d14
fcvtzu w13, d6
fcvtzu w28, d12
fcvtzu x17, d25
fcvtzu x11, d8
fcvtzu x6, d27
fcvtzu x5, d8
fcvtzu x22, d24
fcvtzu x4, d6
fcvtzu x6, d11
fcvtzu x14, d14
fcvtzu x19, d18
fcvtzu x8, d18
fdiv s31, s12, s26
fdiv s27, s0, s20
fdiv s10, s11, s26
fdiv s14, s24, s22
fdiv s7, s22, s30
fdiv s19, s17, s8
fdiv s27, s4, s0
fdiv s2, s10, s7
fdiv s13, s17, s21
fdiv s27, s3, s0
fdiv d13, d2, d13
fdiv d7, d29, d13
fdiv d27, d7, d24
fdiv d22, d21, d16
fdiv d12, d28, d6
fdiv d10, d15, d24
fdiv d18, d11, d28
fdiv d19, d13, d7
fdiv d26, d26, d24
fdiv d16, d21, d27
fmov s16, s2
fmov s29, s29
fmov s9, s26
fmov s10, s5
fmov s2, s3
fmov s27, s23
fmov s19, s7
fmov s20, s26
fmov s17, s3
fmov s18, s3
fmov w6, s14
fmov w16, s27
fmov w22, s17
fmov w30, s13
fmov w7, s11
fmov w24, s17
fmov w27, s5
fmov w20, s23
fmov w27, s25
fmov w29, s2
fmov d28, d23
fmov d25, d16
fmov d30, d13
fmov d10, d15
fmov d17, d28
fmov d19, d31
fmov d10, d9
fmov d27, d0
fmov d26, d17
fmov d13, d27
fmov x9, d22
fmov x21, d10
fmov x12, d28
fmov x20, d27
fmov x21, d30
fmov x14, d18
fmov x13, d23
fmov x20, d20
fmov x0, d12
fmov x5, d21
fmov s8, w2
fmov s21, w27
fmov s0, w19
fmov s18, w1
fmov s1, w27
fmov s22, w24
fmov s22, w10
fmov s8, w21
fmov s31, w17
fmov s13, w7
fmov d8, x11
fmov d7, x13
fmov d1, x10
fmov d19, x20
fmov d21, x21
fmov d27, x24
fmov d14, x1
fmov d15, x5
fmov d0, x22
fmov d9, x4
fmov v2.D[1], x27
fmov v31.D[1], x25
fmov v3.D[1], x26
fmov v9.D[1], x23
fmov v17.D[1], x10
fmov v24.D[1], x12
fmov v1.D[1], x20
fmov v24.D[1], x10
fmov v8.D[1], x13
fmov v8.D[1], x11
fmov x3, v4.D[1]
fmov x4, v16.D[1]
fmov x9, v9.D[1]
fmov x27, v21.D[1]
fmov x13, v8.D[1]
fmov x3, v13.D[1]
fmov x26, v21.D[1]
fmov x23, v11.D[1]
fmov x25, v15.D[1]
fmov x22, v13.D[1]
fmov s5, #-5.000000000000000000e-01
fmov s6, #-5.000000000000000000e-01
fmov s7, #-5.000000000000000000e-01
fmov s6, #-5.000000000000000000e-01
fmov s5, #-5.000000000000000000e-01
fmov s31, #-5.000000000000000000e-01
fmov s9, #-5.000000000000000000e-01
fmov s13, #-5.000000000000000000e-01
fmov s10, #-5.000000000000000000e-01
fmov s17, #-5.000000000000000000e-01
fmov d10, #-5.000000000000000000e-01
fmov d12, #-5.000000000000000000e-01
fmov d21, #-5.000000000000000000e-01
fmov d4, #-5.000000000000000000e-01
fmov d28, #-5.000000000000000000e-01
fmov d10, #-5.000000000000000000e-01
fmov d14, #-5.000000000000000000e-01
fmov d26, #-5.000000000000000000e-01
fmov d31, #-5.000000000000000000e-01
fmov d25, #-5.000000000000000000e-01
fmul s2, s21, s7
fmul s10, s11, s12
fmul s22, s7, s27
fmul s4, s21, s1
fmul s11, s28, s7
fmul s16, s27, s16
fmul s30, s5, s2
fmul s8, s17, s23
fmul s13, s13, s1
fmul s27, s7, s0
fmul d20, d10, d22
fmul d27, d20, d1
fmul d8, d10, d8
fmul d3, d15, d29
fmul d4, d26, d25
fmul d12, d10, d20
fmul d28, d8, d25
fmul d30, d17, d10
fmul d22, d30, d23
fmul d23, d25, d31
fneg s24, s13
fneg s9, s14
fneg s9, s29
fneg s15, s17
fneg s8, s23
fneg s20, s23
fneg s21, s25
fneg s17, s14
fneg s5, s27
fneg s3, s1
fneg d4, d28
fneg d0, d21
fneg d7, d22
fneg d19, d30
fneg d13, d12
fneg d29, d5
fneg d25, d6
fneg d19, d2
fneg d4, d2
fneg d19, d12
fsqrt s26, s8
fsqrt s3, s15
fsqrt s1, s20
fsqrt s29, s6
fsqrt s15, s0
fsqrt s7, s19
fsqrt s29, s7
fsqrt s8, s4
fsqrt s29, s27
fsqrt s2, s11
fsqrt d7, d0
fsqrt d16, d1
fsqrt d6, d4
fsqrt d3, d10
fsqrt d6, d23
fsqrt d22, d0
fsqrt d31, d25
fsqrt d15, d0
fsqrt d13, d13
fsqrt d6, d29
fsub s13, s13, s16
fsub s10, s21, s25
fsub s14, s18, s20
fsub s17, s29, s28
fsub s17, s14, s29
fsub s23, s18, s0
fsub s2, s24, s23
fsub s24, s25, s22
fsub s18, s8, s22
fsub s31, s21, s28
fsub d28, d3, d10
fsub d13, d13, d31
fsub d6, d28, d17
fsub d26, d13, d15
fsub d22, d30, d29
fsub d19, d21, d15
fsub d20, d23, d7
fsub d11, d16, d0
fsub d2, d2, d9
fsub d24, d1, d30
fmadd s21, s30, s1, s31
fmadd s11, s15, s30, s17
fmadd s11, s15, s11, s24
fmadd s30, s2, s22, s27
fmadd s21, s11, s10, s9
fmadd s3, s18, s21, s19
fmadd s18, s23, s21, s27
fmadd s15, s22, s26, s4
fmadd s20, s27, s3, s31
fmadd s10, s1, s16, s21
fmsub s17, s28, s13, s15
fmsub s30, s3, s11, s19
fmsub s15, s21, s29, s18
fmsub s7, s18, s5, s26
fmsub s9, s26, s21, s24
fmsub s16, s15, s29, s5
fmsub s11, s0, s4, s21
fmsub s2, s21, s11, s19
fmsub s17, s24, s2, s15
fmsub s28, s13, s2, s11
fnmadd s3, s31, s29, s10
fnmadd s17, s2, s4, s26
fnmadd s28, s26, s19, s12
fnmadd s9, s16, s17, s20
fnmadd s16, s22, s10, s18
fnmadd s11, s21, s5, s28
fnmadd s13, s8, s11, s9
fnmadd s21, s13, s20, s24
fnmadd s13, s17, s3, s30
fnmadd s19, s7, s25, s15
fnmsub s1, s12, s28, s11
fnmsub s28, s13, s31, s12
fnmsub s3, s9, s31, s14
fnmsub s30, s4, s10, s12
fnmsub s12, s21, s21, s2
fnmsub s3, s10, s26, s16
fnmsub s27, s29, s14, s15
fnmsub s5, s7, s30, s6
fnmsub s19, s26, s17, s15
fnmsub s8, s17, s28, s11
fmadd d26, d27, d26, d25
fmadd d31, d4, d5, d12
fmadd d26, d26, d14, d29
fmadd d4, d8, d13, d0
fmadd d6, d27, d15, d11
fmadd d3, d13, d17, d22
fmadd d8, d3, d6, d16
fmadd d20, d2, d27, d14
fmadd d29, d21, d7, d28
fmadd d26, d12, d8, d20
fmsub d7, d22, d17, d11
fmsub d31, d30, d11, d5
fmsub d25, d26, d16, d28
fmsub d8, d1, d19, d16
fmsub d4, d25, d0, d24
fmsub d27, d27, d7, d24
fmsub d17, d14, d20, d11
fmsub d27, d29, d31, d2
fmsub d19, d16, d13, d18
fmsub d14, d25, d23, d7
fnmadd d19, d7, d4, d27
fnmadd d9, d23, d11, d13
fnmadd d16, d11, d6, d11
fnmadd d7, d13, d3, d24
fnmadd d27, d23, d3, d22
fnmadd d20, d2, d24, d8
fnmadd d18, d6, d26, d0
fnmadd d31, d18, d7, d18
fnmadd d25, d11, d14, d2
fnmadd d2, d25, d16, d18
fnmsub d5, d22, d29, d12
fnmsub d3, d0, d4, d30
fnmsub d24, d7, d21, d12
fnmsub d9, d13, d20, d27
fnmsub d19, d15, d27, d18
fnmsub d1, d2, d5, d26
fnmsub d14, d19, d29, d16
fnmsub d12, d13, d3, d17
fnmsub d3, d0, d29, d6
fnmsub d1, d1, d4, d25
.globl FLOAT_INSTRUCTION_END
FLOAT_INSTRUCTION_END:
"#
);
