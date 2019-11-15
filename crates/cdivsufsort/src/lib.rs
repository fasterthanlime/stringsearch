extern "C" {
    fn divsufsort(T: *const u8, SA: *mut i32, n: i32) -> i32;
    pub fn dss_flush();
}

/// Sort suffixes of `text` and store their lexographic order
/// in the given suffix array `sa`.
/// Will panic if `sa.len()` != `text.len()`
pub fn sort_in_place(text: &[u8], sa: &mut [i32]) {
    assert_eq!(
        text.len(),
        sa.len(),
        "text and suffix array should have same len"
    );
    assert!(
        text.len() < i32::max_value() as usize,
        "text too large, should not exceed {} bytes",
        i32::max_value() - 1
    );

    let ret = unsafe { divsufsort(text.as_ptr(), sa.as_mut_ptr(), text.len() as i32) };
    assert_eq!(0, ret);
}

//// Sort suffixes
pub fn sort<'a>(text: &'a [u8]) -> sacabase::SuffixArray<i32> {
    let mut sa = vec![0; text.len()];
    sort_in_place(text, &mut sa);
    sacabase::SuffixArray::new(text, sa)
}
