fn main() {
    println!("cargo:rustc-link-arg=dependency-chain.o");
    println!("cargo:rustc-link-arg=load-store-ports.o");
}
