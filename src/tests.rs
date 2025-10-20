use core::{arch::global_asm, slice};
pub fn instructions() -> &'static [u32] {
    let start_ptr = unsafe { &INSTRUCTION_START } as *const u32;
    let end_ptr = unsafe { &INSTRUCTION_END } as *const u32;
    let length_bytes = (end_ptr as usize) - (start_ptr as usize);
    let count = length_bytes / 4;
    unsafe { slice::from_raw_parts(start_ptr, count) }
}
unsafe extern "C" {
    static INSTRUCTION_START: u32;
    static INSTRUCTION_END: u32;
}
global_asm!(
    r#"
.data
.globl INSTRUCTION_START
INSTRUCTION_START:
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
cmeq v15.8H, v28.8H, v10.8H
cmeq v12.2S, v27.2S, v27.2S
cmeq v18.16B, v28.16B, v20.16B
cmeq v12.8B, v27.8B, v28.8B
cmeq v2.2D, v30.2D, v3.2D
cmeq v27.8B, v10.8B, v26.8B
cmeq v27.4S, v17.4S, v6.4S
cmeq v5.8B, v28.8B, v20.8B
cmeq v21.4H, v30.4H, v1.4H
cmeq v9.2D, v28.2D, v27.2D
cmeq v8.8H, v15.8H, #0
cmeq v21.8H, v11.8H, #0
cmeq v17.4S, v5.4S, #0
cmeq v15.2S, v0.2S, #0
cmeq v9.4S, v26.4S, #0
cmeq v5.16B, v11.16B, #0
cmeq v11.4S, v8.4S, #0
cmeq v7.2S, v29.2S, #0
cmeq v31.2S, v30.2S, #0
cmeq v8.8H, v27.8H, #0
cnt v4.8b, v5.8b
cnt v20.8b, v19.8b
cnt v26.8b, v5.8b
cnt v30.8b, v11.8b
cnt v25.8b, v3.8b
cnt v27.8b, v16.8b
cnt v4.8b, v4.8b
cnt v17.8b, v30.8b
cnt v9.8b, v28.8b
cnt v10.8b, v20.8b
cnt v12.16b, v18.16b
cnt v27.16b, v16.16b
cnt v15.16b, v27.16b
cnt v19.16b, v13.16b
cnt v3.16b, v16.16b
cnt v8.16b, v7.16b
cnt v21.16b, v28.16b
cnt v27.16b, v15.16b
cnt v2.16b, v25.16b
cnt v26.16b, v27.16b
dup v29.16B, w14
dup v1.4H, w2
dup v31.8H, w11
dup v9.4H, w5
dup v27.16B, w18
dup v10.4H, w18
dup v24.2S, w24
dup v0.8H, w19
dup v29.8B, w3
dup v31.8B, w22
dup v26.2D, x13
dup v5.2D, x15
dup v1.2D, x19
dup v26.2D, x10
dup v30.2D, x29
dup v19.2D, x21
dup v28.2D, x15
dup v20.2D, x5
dup v2.2D, x14
dup v30.2D, x22
orr v21.8b, v9.8b, v29.8b
orr v2.8b, v14.8b, v9.8b
orr v8.8b, v16.8b, v20.8b
orr v18.8b, v10.8b, v9.8b
orr v20.8b, v8.8b, v26.8b
orr v10.8b, v28.8b, v21.8b
orr v14.8b, v24.8b, v8.8b
orr v4.8b, v12.8b, v27.8b
orr v19.8b, v14.8b, v12.8b
orr v23.8b, v13.8b, v27.8b
orr v9.16b, v2.16b, v5.16b
orr v6.16b, v4.16b, v19.16b
orr v15.16b, v13.16b, v3.16b
orr v4.16b, v31.16b, v13.16b
orr v13.16b, v20.16b, v22.16b
orr v7.16b, v30.16b, v18.16b
orr v28.16b, v12.16b, v10.16b
orr v4.16b, v16.16b, v22.16b
orr v0.16b, v3.16b, v5.16b
orr v12.16b, v27.16b, v18.16b
umov w0, v4.b[0]
umov w4, v11.b[1]
umov w11, v26.b[2]
umov w20, v30.b[1]
umov w16, v12.b[1]
umov w15, v19.b[3]
umov w20, v16.b[3]
umov w19, v20.b[3]
umov w22, v20.b[3]
umov w0, v0.b[2]
umov w15, v8.h[2]
umov w1, v20.h[1]
umov w20, v20.h[0]
umov w2, v24.h[2]
umov w11, v9.h[2]
umov w3, v16.h[2]
umov w22, v9.h[2]
umov w7, v12.h[2]
umov w3, v5.h[3]
umov w2, v4.h[3]
umov w4, v20.s[3]
umov w30, v25.s[0]
umov w14, v23.s[0]
umov w8, v27.s[0]
umov w10, v5.s[1]
umov w4, v8.s[1]
umov w14, v2.s[2]
umov w4, v0.s[2]
umov w25, v18.s[0]
umov w18, v26.s[0]
umov x3, v21.d[0]
umov x17, v6.d[0]
umov x28, v24.d[1]
umov x16, v7.d[1]
umov x3, v16.d[0]
umov x1, v31.d[0]
umov x27, v9.d[0]
umov x4, v28.d[1]
umov x17, v14.d[1]
umov x23, v19.d[1]
.globl INSTRUCTION_END
INSTRUCTION_END:
"#
);
