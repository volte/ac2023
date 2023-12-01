use copy_to_output::copy_to_output;
use std::env;

fn main() {
    println!("cargo:rerun-if-changed=inputs/*");
    copy_to_output("inputs", &env::var("PROFILE").unwrap())
        .expect("Could not copy inputs to target directory");
}
