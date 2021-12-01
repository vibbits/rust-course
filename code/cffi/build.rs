fn main() {
    println!("cargo:rerun-if-changed=src/resource.c");

    cc::Build::new()
        .file("src/resource.c")
        .compile("resource");
}