use wasm_bindgen::prelude::*;

extern crate blake2_rfc;

#[wasm_bindgen]
pub fn blake2b(length: usize, key: Option<Vec<u8>>, input: Vec<u8>) -> Vec<u8> {
     blake2_rfc::blake2b::blake2b(length, &key.unwrap_or(vec![]), &input).as_bytes().to_vec()
}

#[wasm_bindgen]
pub fn blake2s(length: usize, key: Option<Vec<u8>>, input: Vec<u8>) -> Vec<u8> {
     blake2_rfc::blake2s::blake2s(length, &key.unwrap_or(vec![]), &input).as_bytes().to_vec()
}
