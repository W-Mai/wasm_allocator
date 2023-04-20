use std::env;
use std::env::VarError;
use std::fs::File;
use std::io::Write;
use std::path::Path;

// 1KB
const DEFAULT_HEAP_SIZE: usize = 1024;
const WASM_ALLOCATOR_HEAP_SIZE: &str = "WASM_ALLOCATOR_HEAP_SIZE";

fn main() {
    let out_dir = env::var("OUT_DIR").expect("OUT_DIR environment variable not provided");
    let dest_path = Path::new(&out_dir).join(WASM_ALLOCATOR_HEAP_SIZE);
    let env_var = env::var(WASM_ALLOCATOR_HEAP_SIZE);
    println!("cargo:WASM_ALLOCATOR_HEAP_SIZE={}", env_var.unwrap_or("".to_string()));
    let size: usize = match env::var(WASM_ALLOCATOR_HEAP_SIZE) {
        Ok(s) => s.parse().expect("Could not interpret WASM_ALLOCATOR_HEAP_SIZE as a 32 bit unsigned integer"),
        Err(ve) => match ve {
            VarError::NotPresent => { DEFAULT_HEAP_SIZE }
            VarError::NotUnicode(_) => { panic!("Could not interpret WASM_ALLOCATOR_HEAP_SIZE as a string representing a 32 bit unsigned integer") }
        },
    };
    let mut f = File::create(&dest_path)
        .expect("Could not create file to store wasm allocator HEAP_SIZE metadata.");
    write!(f, "{}", size)
        .expect("Could not write file to store wasm allocator HEAP_SIZE metadata.");
    f.flush()
        .expect("Could not flush file to store wasm allocator HEAP_SIZE metadata.");
}
