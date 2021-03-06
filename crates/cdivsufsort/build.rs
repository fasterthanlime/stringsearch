fn main() {
    let mut build = cc::Build::new();

    build.flag("-DHAVE_CONFIG_H=1").warnings(false);

    let profile = std::env::var("PROFILE").unwrap();
    match profile.as_str() {
        "debug" => {
            // muffin
        }
        "release" => {
            if build.get_compiler().is_like_msvc() {
                build.flag("/Oi").flag("/Ot").flag("/Ox").flag("/Oy");
            }
        }
        _ => {}
    };
    if std::env::var("CARGO_FEATURE_CROSSCHECK").is_ok() {
        build.flag("-DENABLE_CROSSCHECK=1");
    }

    build
        .file("c-sources/divsufsort.c")
        .file("c-sources/sssort.c")
        .file("c-sources/trsort.c")
        .file("c-sources/utils.c");

    build.compile("libdivsufsort.a");
}
