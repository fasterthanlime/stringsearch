#![no_main]
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    let mut SA = Vec::new(data.len());
    divsufsort::divsufsort(&data, &mut SA);
});
