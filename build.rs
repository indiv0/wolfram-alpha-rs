#[cfg(feature = "with-syntex")]
mod inner {
    extern crate serde_codegen;

    use std::env;
    use std::path::Path;

    pub fn main() {
        let out_dir = env::var_os("OUT_DIR").unwrap();

        for &(src, dst) in &[
            ("src/model.in.rs", "model.rs"),
        ] {
            let src = Path::new(src);
            let dst = Path::new(&out_dir).join(dst);

            serde_codegen::expand(&src, &dst).unwrap();
        }
    }
}

#[cfg(feature = "nightly")]
mod inner {
    pub fn main() {}
}

fn main() {
    inner::main();
}
