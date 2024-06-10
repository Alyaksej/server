fn main() {
    println!("cargo:rustc-link-lib=static=array_processing");
    println!("cargo:rustc-link-search=native=/home/user/RustroverProjects/server");
}