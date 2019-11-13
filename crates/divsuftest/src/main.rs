use size_format::SizeFormatterBinary;
use std::time::Instant;

fn main() {
    better_panic::install();
    std::fs::create_dir_all("crosscheck").unwrap();

    let first_arg = std::env::args().nth(1).unwrap_or_else(|| {
        std::path::PathBuf::from("testdata")
            .join("input")
            .to_string_lossy()
            .into()
    });
    let orig_input = std::fs::read(first_arg).unwrap();
    let maxlen: usize = std::env::args()
        .nth(2)
        .map(|x| x.parse().unwrap())
        .unwrap_or_else(|| orig_input.len());
    let input = &orig_input[..maxlen];
    println!(
        "{:>20} {}B",
        "Input size",
        SizeFormatterBinary::new(maxlen as u64)
    );

    println!("{:>20} Running", "c");

    let before_c = Instant::now();
    let c_duration;

    unsafe {
        let mut sa = vec![0_i32; input.len()];
        cdivsufsort::divsufsort(input.as_ptr(), sa.as_mut_ptr(), input.len() as i32);
        c_duration = before_c.elapsed();
        cdivsufsort::dss_flush();
        check_order(|i| sa[i], input);
    }

    let rust_duration = {
        let res = std::panic::catch_unwind(|| {
            println!("{:>20} Running...", "rust");
            let mut sa = vec![0 as divsufsort::Idx; input.len()];
            let before_rust = Instant::now();

            std::thread::spawn(|| loop {
                std::thread::sleep(std::time::Duration::from_millis(500));
                divsufsort::crosscheck::flush();
            });

            divsufsort::divsufsort(&input[..], &mut sa[..]);
            let rust_duration = before_rust.elapsed();
            check_order(|i| sa[i], input);
            rust_duration
        });
        divsufsort::crosscheck::flush();
        res.unwrap()
    };

    let huc_duration = {
        println!("{:>20} Running...", "rust-ref");
        let before_huc = Instant::now();
        let sa = suffix_array::SuffixArray::new(&input[..]);
        let (_, sa) = sa.into_parts();
        let sa = &sa[1..];
        check_order(|i| sa[i] as i32, input);
        before_huc.elapsed()
    };

    let s1 = format!("c {:?}", c_duration);
    let s2 = format!("rust {:?}", rust_duration);
    let s3 = format!("rust-ref {:?}", huc_duration);
    println!("{:30} {:30} {:30}", s1, s2, s3);
}

fn check_order<SA: Fn(usize) -> i32>(sa: SA, input: &[u8]) {
    for i in 0..(input.len() - 1) {
        assert!(
            input[sa(i) as usize..] < input[sa(i + 1) as usize..],
            "suffixes should be ordered"
        );
    }
}
