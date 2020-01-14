use std::env;
use std::path::Path;

fn main() {
    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let out = env::var("OUT_DIR").unwrap();
    let dest_dir = Path::new(&out);

    ::cbindgen::Builder::new()
      .with_language(::cbindgen::Language::C)
      .with_crate(crate_dir)
      .exclude_item("XTestFakeRelativeMotionEvent")
      .generate()
      .expect("Unable to generate bindings")
      .write_to_file(&dest_dir.join("../../../bindings.h"));
}
