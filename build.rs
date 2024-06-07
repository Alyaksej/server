fn main() {
    println!("cargo:rustc-link-lib=static=arrayProcessing");
    println!("cargo:rustc-link-search=native=/home/user/RustroverProjects/proj/src");
    println!("cargo:rustc-link-lib=static=voidptr");
    println!("cargo:rustc-link-search=native=/home/user/RustroverProjects/proj/src");
}