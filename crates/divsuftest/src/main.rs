use size_format::SizeFormatterBinary;
use std::{env, io::Write, process, time::Instant};

fn main() {
    better_panic::install();

    let all_args: Vec<String> = env::args().collect();
    let args = &all_args[1..];

    if args.len() == 0 {
        println!("Usage: divsuftest bench|crosscheck INPUT [LENGTH]");
        process::exit(1);
    }

    let (cmd, args) = (&args[0], &args[1..]);

    let input_path = args.get(0).expect("INPUT argument expected");
    let input_full = std::fs::read(input_path).unwrap();
    let len = args
        .get(1)
        .map(parse_size)
        .unwrap_or_else(|| input_full.len());
    let input = &input_full[..len];
    println!(
        "Input is size {}B",
        SizeFormatterBinary::new(input.len() as u64)
    );

    match cmd.as_ref() {
        "crosscheck" => {
            #[cfg(not(feature = "crosscheck"))]
            {
                println!(
                    "Error: This version of divsuftest wasn't built with crosscheck enabled :("
                );
                println!("Bailing out.");
                process::exit(1);
            }

            #[cfg(feature = "crosscheck")]
            command_crosscheck(input);
        }
        "bench" => command_bench(input),
        "run" => command_run(input),
        x => panic!("unknown command {:?}", x),
    }
}

#[cfg(feature = "crosscheck")]
fn command_crosscheck(input: &[u8]) {
    println!("Cross-checking!");
    std::fs::create_dir_all("crosscheck").unwrap();

    {
        println!("Running C version...");
        let sa = cdivsufsort::sort(input);
        unsafe {
            cdivsufsort::dss_flush();
        }
        println!("Verifying C result...");
        sa.verify().expect("cdivsufsort should sort all suffixes");
    }

    {
        let res = std::panic::catch_unwind(|| {
            println!("Running Rust version...");
            std::thread::spawn(|| loop {
                std::thread::sleep(std::time::Duration::from_millis(500));
                divsufsort::crosscheck::flush();
            });

            let sa = divsufsort::sort(input);

            println!("Verifying Rust result...");
            sa.verify().expect("cdivsufsort should sort all suffixes");
        });
        divsufsort::crosscheck::flush();
        res.unwrap()
    };
}

fn command_run(input: &[u8]) {
    let before = Instant::now();
    divsufsort::sort(input);
    println!("Done in {:?}", before.elapsed());
}

fn command_bench(input: &[u8]) {
    #[cfg(debug_assertions)]
    {
        println!("==========================================");
        println!("Warning: benchmarking with a debug build.");
        println!("This will be slow..");
        println!("==========================================");
    }

    #[cfg(feature = "crosscheck")]
    {
        println!("==========================================");
        println!("Warning: benchmarking with crosscheck enabled.");
        println!("This will be slow..");
        println!("==========================================");
    }

    let flush = || {
        std::io::stdout().lock().flush().unwrap();
    };

    let mut datapoints = Vec::new();
    let mut measure = |name: &'static str, f: &dyn Fn()| {
        print!(".");
        flush();
        let before = Instant::now();
        f();
        datapoints.push((name, before.elapsed()))
    };

    print!("measuring");
    flush();

    measure("c-divsufsort", &|| {
        cdivsufsort::sort(input);
    });
    measure("divsufsort", &|| {
        divsufsort::sort(input);
    });
    measure("saca-k", &|| {
        suffix_array::SuffixArray::new(input);
    });

    println!("done!");

    {
        use cli_table::{format::CellFormat, Cell, Row, Table};
        let bold = CellFormat::builder().bold(true).build();
        let regular = CellFormat::builder().build();

        let mut rows = vec![Row::new(vec![
            Cell::new("Algorithm", bold),
            Cell::new("Time", bold),
            Cell::new("Average speed", bold),
        ])];
        for dp in datapoints {
            let bps = (input.len() as f64 / dp.1.as_secs_f64()) as u64;
            rows.push(Row::new(vec![
                Cell::new(dp.0, regular),
                Cell::new(&format!("{:?}", dp.1), regular),
                Cell::new(&format!("{}B/s", SizeFormatterBinary::new(bps)), regular),
            ]));
        }

        Table::new(rows, Default::default()).print_stdout().unwrap();
    }
}

fn parse_size<I: AsRef<str>>(input: I) -> usize {
    let mut factor = 1_usize;

    let input = input.as_ref().to_lowercase();
    let input = if input.ends_with("k") {
        factor = 1024;
        input.trim_end_matches("k")
    } else if input.ends_with("m") {
        factor = 1024 * 1024;
        input.trim_end_matches("m")
    } else {
        &input[..]
    };

    let size: usize = input.parse().unwrap();
    size * factor
}
