use std::time::Instant;

fn main() {
    better_panic::install();

    let first_arg = std::env::args().nth(1).unwrap_or_else(|| {
        std::path::PathBuf::from("testdata")
            .join("input.txt")
            .to_string_lossy()
            .into()
    });
    let input = std::fs::read(first_arg).unwrap();

    #[cfg(debug_assertions)]
    println!("{:>20} Running", "C");

    let mut sa_c = vec![0 as divsufsort::Idx; input.len()];
    let before_c = Instant::now();
    let c_duration;

    unsafe {
        cdivsufsort::divsufsort(input.as_ptr(), sa_c.as_mut_ptr(), input.len() as i32);
        c_duration = before_c.elapsed();
        cdivsufsort::dss_flush();
    }

    for i in 0..(sa_c.len() - 1) {
        assert!(
            input[sa_c[i] as usize..] < input[sa_c[i + 1] as usize..],
            "suffixes should be ordered"
        );
    }

    #[cfg(debug_assertions)]
    println!("{:>20} Running...", "Rust");

    {
        let res = std::panic::catch_unwind(|| {
            let mut sa_rust = vec![0 as divsufsort::Idx; input.len()];
            let before_rust = Instant::now();

            std::thread::spawn(|| loop {
                std::thread::sleep(std::time::Duration::from_millis(500));
                divsufsort::crosscheck::flush();
            });

            divsufsort::divsufsort(&input[..], &mut sa_rust[..]);
            let rust_duration = before_rust.elapsed();
            assert!(sa_c == sa_rust, "c & rust divsufsort SAs should be equal");

            #[cfg(debug_assertions)]
            println!("{:>20} Running...", "huc");

            let huc_duration = {
                let before_huc = Instant::now();
                let sa = suffix_array::SuffixArray::new(&input[..]);
                let (_, sa) = sa.into_parts();
                let sa = &sa[1..];

                for i in 0..sa_c.len() {
                    assert_eq!(sa_c[i], sa[i] as i32);
                }
                before_huc.elapsed()
            };

            let s1 = format!("c {:?}", c_duration);
            let s2 = format!("rust {:?}", rust_duration);
            let s3 = format!("rust-ref {:?}", huc_duration);
            println!("{:30} {:30} {:30}", s1, s2, s3);
        });
        divsufsort::crosscheck::flush();
        res.unwrap()
    };
}

