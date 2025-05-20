fn main() {
    build::rustc_env("RUSTC_FLAGS", "-C linker-plugin-lto -C linker=clang");
    build::rustc_link_arg("-fuse-ld=lld");

    cc::Build::new()
        .file("job.c")
        .flag("-flto=thin")
        .compile("libjob.a");
}
