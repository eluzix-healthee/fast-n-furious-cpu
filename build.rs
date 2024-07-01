use std::env;

fn main() {

    match env::var("OBJ_FILE") {
        Ok(object_file) => {
            println!("cargo:rustc-link-arg={}", object_file);
        },
        Err(_) => {}
    }
}
