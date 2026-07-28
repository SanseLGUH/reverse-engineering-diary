fn main() {
    let java_home = std::env::var("JAVA_HOME").expect("JAVA_HOME not set");
    println!("cargo:rustc-link-search=native={}/lib", java_home);
    println!("cargo:rustc-link-lib=dylib=jvm");
}