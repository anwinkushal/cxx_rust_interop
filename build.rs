fn main() {
    let bridges = ["src/e1_addition.rs", "src/e2_palindrome.rs"];
    let cpp_files = ["src/cxx_src/e1_addition.cpp", "src/cxx_src/e2_palindrome.cpp"];
    
    // Create a single builder for all bridges and C++ files
    let mut builder = cxx_build::bridges(&bridges);
    
    for cpp in &cpp_files {
        builder.file(cpp);
    }
    
    builder
        .flag_if_supported("-std=c++14")
        .compile("rust_cxx_class"); // Single compile call

    // Rerun if any source files change
    println!("cargo:rerun-if-changed=src/lib.rs");
    for op in &["e1_addition", "e2_palindrome"] {
        println!("cargo:rerun-if-changed=src/{}.rs", op);
        println!("cargo:rerun-if-changed=src/cxx_src/{}.cpp", op);
        println!("cargo:rerun-if-changed=include/{}.h", op);
    }
}