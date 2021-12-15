extern crate cc;

fn main () {
    println!("cargo:rustc-flags=-l dylib=stdc++");
}