extern crate wasm_bindgen;

use base64::{engine::general_purpose, Engine as _};
use wasm_bindgen::prelude::*;

const DELTA: u32 = 0x9e3779b9;
struct TEA {}
impl TEA {
    pub fn encrypt(v: &[u32], k: &[u32], iter: u32) -> [u32; 2] {
        let mut v0 = v[0];
        let mut v1 = v[1];
        let mut sum = 0u32;
        for _ in 0..iter {
            sum = sum.wrapping_add(DELTA);
            v0 = v0.wrapping_add(
                v1.wrapping_shl(4).wrapping_add(k[0])
                    ^ (v1.wrapping_add(sum))
                    ^ v1.wrapping_shr(5).wrapping_add(k[1]),
            );
            v1 = v1.wrapping_add(
                v0.wrapping_shl(4).wrapping_add(k[2])
                    ^ (v0.wrapping_add(sum))
                    ^ v0.wrapping_shr(5).wrapping_add(k[3]),
            );
        }
        return [v0, v1];
    }
    pub fn decrypt(v: &[u32], k: &[u32], iter: u32) -> [u32; 2] {
        let mut v0 = v[0];
        let mut v1 = v[1];
        let mut sum = DELTA.wrapping_mul(iter);
        for _ in 0..iter {
            v1 = v1.wrapping_sub(
                v0.wrapping_shl(4).wrapping_add(k[2])
                    ^ (v0.wrapping_add(sum))
                    ^ v0.wrapping_shr(5).wrapping_add(k[3]),
            );
            v0 = v0.wrapping_sub(
                v1.wrapping_shl(4).wrapping_add(k[0])
                    ^ (v1.wrapping_add(sum))
                    ^ v1.wrapping_shr(5).wrapping_add(k[1]),
            );
            sum = sum.wrapping_sub(DELTA);
        }
        return [v0, v1];
    }
}

#[wasm_bindgen]
extern "C" {
    pub fn alert(s: &str);

    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);

    #[wasm_bindgen(js_namespace = console, js_name = log)]
    pub fn log_u8(u: &[u8]);
}

#[wasm_bindgen]
pub fn tea_encrypt(str: &[u8], key_base64: &str, iter: u32) -> Vec<u8> {
    let (_, str_byte, suffix) = unsafe { str.align_to::<u32>() };
    let key = general_purpose::STANDARD.decode(key_base64).unwrap();
    let (_, key_byte, _) = unsafe { key.align_to::<u32>() };
    let mut i = 0;
    let mut arry = Vec::new();
    while i < str_byte.len() {
        if i + 1 < str_byte.len() {
            let result = TEA::encrypt(str_byte.get(i..(i + 2)).unwrap(), key_byte, iter);
            arry.extend(result);
        } else {
            arry.extend([str_byte.get(i).unwrap()]);
        }
        i += 2;
    }
    let mut return_val = Vec::new();
    let (_, res, _) = unsafe { arry.align_to::<u8>() };
    return_val.extend(res);
    return_val.extend(suffix);
    return_val
}

#[wasm_bindgen]
pub fn tea_decrypt(str: &[u8], key_base64: &str, iter: u32) -> Vec<u8> {
    let (_, str_byte, suffix) = unsafe { str.align_to::<u32>() };
    let key = general_purpose::STANDARD.decode(key_base64).unwrap();
    let (_, key_byte, _) = unsafe { key.align_to::<u32>() };
    let mut i = 0;
    let mut arry = Vec::new();
    while i < str_byte.len() {
        if i + 1 < str_byte.len() {
            let result = TEA::decrypt(str_byte.get(i..(i + 2)).unwrap(), key_byte, iter);
            arry.extend(result);
        } else {
            arry.extend([str_byte.get(i).unwrap()]);
        }
        i += 2;
    }
    let mut return_val = Vec::new();
    let (_, res, _) = unsafe { arry.align_to::<u8>() };
    return_val.extend(res);
    return_val.extend(suffix);
    return_val
}

#[wasm_bindgen]
pub fn base64_encrypt(str: &str) -> String {
    general_purpose::STANDARD.encode(str)
}

#[wasm_bindgen]
pub fn base64_decrypt(str: &str) -> Vec<u8> {
    general_purpose::STANDARD.decode(str).unwrap()
}
