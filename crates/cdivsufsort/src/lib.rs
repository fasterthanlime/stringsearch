extern "C" {
    pub fn divsufsort(T: *const u8, SA: *mut i32, n: i32) -> i32;
    pub fn dss_flush();
}
