#[cfg(windows)]
fn print_link_search_path() {
    use std::env;

    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    if cfg!(target_arch = "x86_64") {
        println!("cargo:rustc-link-search=native={}/lib/x64", manifest_dir);
    } else {
        println!("cargo:rustc-link-search=native={}/lib", manifest_dir);
    }
}

#[cfg(not(windows))]
fn print_link_search_path() {}

fn main() {
    print_link_search_path();
}