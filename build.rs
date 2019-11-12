fn main() {
    // TODO: Only compile+link C library in test

    let mut build = cc::Build::new();

    build.flag("-DHAVE_CONFIG_H=1").warnings(false);

    let profile = std::env::var("PROFILE").unwrap();
    match profile.as_str() {
        "debug" => {
            build.flag("-DENABLE_CROSSCHECK=1");
        }
        "release" => {
            build.flag("/Oi").flag("/Ot").flag("/Ox").flag("/Oy");
        }
        _ => {}
    };

    build
        .file("original/divsufsort.c")
        .file("original/sssort.c")
        .file("original/trsort.c")
        .file("original/utils.c");

    build.compile("libdivsufsort.a");
}
