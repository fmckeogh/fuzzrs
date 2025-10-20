use core::{arch::global_asm, slice};
pub fn instructions() -> &'static [u32] {
    let start_ptr = unsafe { &VECTOR_INSTRUCTION_START } as *const u32;
    let end_ptr = unsafe { &VECTOR_INSTRUCTION_END } as *const u32;
    let length_bytes = (end_ptr as usize) - (start_ptr as usize);
    let count = length_bytes / 4;
    unsafe { slice::from_raw_parts(start_ptr, count) }
}
unsafe extern "C" {
    static VECTOR_INSTRUCTION_START: u32;
    static VECTOR_INSTRUCTION_END: u32;
}
global_asm!(
    r#"
.data
.globl VECTOR_INSTRUCTION_START
VECTOR_INSTRUCTION_START:
add v7.2S, v9.2S, v19.2S
add v17.8H, v10.8H, v12.8H
add v9.8H, v26.8H, v11.8H
add v18.2S, v3.2S, v6.2S
add v28.16B, v20.16B, v24.16B
add v27.8B, v7.8B, v13.8B
add v22.4S, v14.4S, v3.4S
add v19.4S, v9.4S, v26.4S
add v6.4S, v13.4S, v23.4S
add v17.8B, v3.8B, v26.8B
addp v5.2S, v5.2S, v23.2S
addp v24.8H, v30.8H, v20.8H
addp v11.16B, v13.16B, v6.16B
addp v27.16B, v20.16B, v17.16B
addp v14.2S, v20.2S, v1.2S
addp v1.2D, v28.2D, v7.2D
addp v16.4S, v30.4S, v1.4S
addp v1.8B, v28.8B, v7.8B
addp v30.8H, v30.8H, v23.8H
addp v10.8B, v11.8B, v22.8B
and v15.8b, v24.8b, v28.8b
and v10.8b, v12.8b, v16.8b
and v27.8b, v27.8b, v18.8b
and v15.8b, v28.8b, v20.8b
and v12.8b, v24.8b, v27.8b
and v28.8b, v2.8b, v26.8b
and v30.8b, v3.8b, v27.8b
and v26.8b, v10.8b, v26.8b
and v27.8b, v9.8b, v17.8b
and v6.8b, v5.8b, v28.8b
and v28.16b, v20.16b, v21.16b
and v24.16b, v30.16b, v1.16b
and v9.16b, v25.16b, v28.16b
and v27.16b, v8.16b, v25.16b
and v15.16b, v21.16b, v17.16b
and v11.16b, v17.16b, v19.16b
and v5.16b, v15.16b, v23.16b
and v0.16b, v9.16b, v1.16b
and v26.16b, v5.16b, v10.16b
and v11.16b, v11.16b, v16.16b
cmeq d8, d7, d4
cmeq d29, d31, d3
cmeq d30, d8, d28
cmeq d27, d4, d5
cmeq d20, d19, d26
cmeq d5, d30, d11
cmeq d25, d3, d27
cmeq d16, d4, d4
cmeq d17, d30, d9
cmeq d28, d10, d20
cmeq v12.8H, v27.8H, v16.8H
cmeq v15.8B, v19.8B, v13.8B
cmeq v3.8B, v8.8B, v7.8B
cmeq v21.2D, v27.2D, v15.2D
cmeq v2.16B, v26.16B, v27.16B
cmeq v29.2S, v11.2S, v1.2S
cmeq v26.4S, v31.4S, v3.4S
cmeq v24.16B, v24.16B, v4.16B
cmeq v27.4H, v21.4H, v10.4H
cmeq v14.4H, v24.4H, v18.4H
cmeq d24, d0, #0
cmeq d25, d13, #0
cmeq d29, d20, #0
cmeq d28, d31, #0
cmeq d14, d23, #0
cmeq d26, d11, #0
cmeq d12, d5, #0
cmeq d12, d6, #0
cmeq d1, d11, #0
cmeq d10, d26, #0
cmeq v21.4S, v30.4S, #0
cmeq v16.4S, v19.4S, #0
cmeq v27.2D, v28.2D, #0
cmeq v19.16B, v20.16B, #0
cmeq v19.4S, v2.4S, #0
cmeq v16.4S, v30.4S, #0
cmeq v15.2S, v21.2S, #0
cmeq v9.16B, v2.16B, #0
cmeq v14.8B, v8.8B, #0
cmeq v16.4H, v18.4H, #0
cnt v10.8b, v9.8b
cnt v20.8b, v8.8b
cnt v26.8b, v10.8b
cnt v28.8b, v21.8b
cnt v14.8b, v24.8b
cnt v8.8b, v4.8b
cnt v12.8b, v27.8b
cnt v19.8b, v14.8b
cnt v12.8b, v23.8b
cnt v13.8b, v27.8b
cnt v9.16b, v2.16b
cnt v5.16b, v6.16b
cnt v4.16b, v19.16b
cnt v15.16b, v13.16b
cnt v3.16b, v4.16b
cnt v31.16b, v13.16b
cnt v13.16b, v20.16b
cnt v22.16b, v7.16b
cnt v30.16b, v18.16b
cnt v28.16b, v12.16b
dup v10.4H, w14
dup v22.8B, w2
dup v5.8B, w5
dup v18.4S, w27
dup v20.2S, w19
dup v25.4H, w9
dup v6.4S, w9
dup v5.16B, w26
dup v25.16B, w15
dup v23.8H, w11
dup v3.2D, x3
dup v19.2D, x1
dup v23.2D, x27
dup v18.2D, x26
dup v22.2D, x30
dup v1.2D, x8
dup v28.2D, x29
dup v26.2D, x14
dup v6.2D, x29
dup v26.2D, x16
orr v10.8b, v30.8b, v12.8b
orr v30.8b, v17.8b, v5.8b
orr v19.8b, v8.8b, v4.8b
orr v19.8b, v26.8b, v20.8b
orr v27.8b, v17.8b, v25.8b
orr v16.8b, v18.8b, v23.8b
orr v4.8b, v14.8b, v27.8b
orr v28.8b, v9.8b, v5.8b
orr v5.8b, v15.8b, v8.8b
orr v21.8b, v9.8b, v2.8b
orr v30.16b, v19.16b, v0.16b
orr v10.16b, v18.16b, v18.16b
orr v16.16b, v5.16b, v26.16b
orr v20.16b, v24.16b, v21.16b
orr v8.16b, v20.16b, v6.16b
orr v2.16b, v4.16b, v24.16b
orr v25.16b, v8.16b, v7.16b
orr v21.16b, v5.16b, v16.16b
orr v26.16b, v10.16b, v31.16b
orr v2.16b, v0.16b, v9.16b
umov w15, v30.b[0]
umov w20, v9.b[2]
umov w28, v25.b[3]
umov w5, v13.b[0]
umov w18, v21.b[0]
umov w28, v23.b[0]
umov w22, v17.b[0]
umov w24, v6.b[1]
umov w1, v0.b[0]
umov w13, v2.b[0]
umov w9, v6.h[2]
umov w8, v11.h[3]
umov w0, v2.h[0]
umov w11, v20.h[1]
umov w23, v26.h[3]
umov w19, v7.h[2]
umov w14, v12.h[3]
umov w23, v24.h[1]
umov w5, v20.h[1]
umov w15, v15.h[3]
umov w21, v20.s[2]
umov w0, v21.s[1]
umov w26, v4.s[0]
umov w12, v7.s[0]
umov w16, v22.s[2]
umov w11, v5.s[1]
umov w9, v24.s[1]
umov w19, v24.s[1]
umov w7, v9.s[1]
umov w16, v22.s[1]
umov x27, v31.d[1]
umov x28, v2.d[0]
umov x4, v9.d[0]
umov x13, v13.d[0]
umov x1, v26.d[0]
umov x13, v6.d[0]
umov x19, v8.d[0]
umov x6, v18.d[0]
umov x0, v10.d[1]
umov x29, v15.d[1]
movi d9, #0
movi d3, #0
movi d0, #0
movi d30, #0
movi d14, #0
movi d3, #0
movi d8, #0
movi d26, #0
movi d12, #0
movi d23, #0
.globl VECTOR_INSTRUCTION_END
VECTOR_INSTRUCTION_END:
"#
);
