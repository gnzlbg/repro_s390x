#![feature(repr_simd, platform_intrinsics)]
#![allow(non_camel_case_types)]

#[derive(Copy,Clone)]
#[repr(simd)]
pub struct i128x1(i128);

extern "platform-intrinsic" {
    pub fn simd_shr<T>(x: T, y: T) -> T;
}

#[test]
pub fn vector_test() {
    unsafe {
        let z = i128x1(0_i128);
        let o = i128x1(1_i128);

        if simd_shr(o, z).0 !=  o.0 {
            panic!();
        }
    }
}

#[test]
pub fn scalar_test() {
    let z = 0_i128;
    let o = 1_i128;

    if o >> z != o {
        panic!();
    }
}
