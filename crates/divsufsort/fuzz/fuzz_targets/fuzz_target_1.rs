#![no_main]
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    let mut SA = vec![0i32; data.len()];
    divsufsort::divsufsort(&data, &mut SA);
});
