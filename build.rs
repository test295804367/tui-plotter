
fn main() {
    cc::Build::new()
        .cpp(true)
        .file("src_cpp/libqalculate_wrapper.cpp")
        .include("include")
        .flag_if_supported("-std=c++11")
        .compile("libqalculate_wrapper.a");
}
