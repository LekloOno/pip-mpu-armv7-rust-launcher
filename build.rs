use std::path::Path;

fn main() {
    let local_path = Path::new(env!("CARGO_MANIFEST_DIR"));
    println!(
        "cargo:rerun-if-changed={}",
        local_path.join("link.ld").display()
    )
}
