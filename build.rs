fn main() {
    // TODO: Only compile+link C library in test

    cc::Build::new()
        .flag("-DHAVE_CONFIG_H")
        .file("original/divsufsort.c")
        .file("original/sssort.c")
        .file("original/trsort.c")
        .file("original/utils.c")
        .compile("libdivsufsort.a");
}
