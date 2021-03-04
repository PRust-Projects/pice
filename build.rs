use std::process::Command;
use std::env;
use std::path::Path;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    Command::new("x86_64-w64-mingw32-windres")
        .args(&["src/icon.rc"])
        .arg(&format!("{}/icon.o", out_dir))
        .status().unwrap();
    
    Command::new("x86_64-w64-mingw32-gcc-ar")
        .args(&["crus", "libicon.a", "icon.o"])
        .current_dir(&Path::new(&out_dir))
        .status().unwrap();

    println!("cargo:rustc-link-search=native={}", out_dir);
    println!("cargo:rustc-link-lib=static=icon");
}
