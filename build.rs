pub fn main() {
    println!("cargo::rerun-if-changed=src/start.s");
    println!("cargo::rerun-if-changed=linker.ld");
}
