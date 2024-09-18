use std::env;
use std::ffi::OsStr;
use std::path::PathBuf;
use std::process::Command;

const SUBMODULE: &str = "TurboPForOM";
const LIB: &str = "ic";

fn run_command<I, S>(prog: &str, args: I)
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
{
    let output = Command::new(prog)
        .args(args)
        .output()
        .expect("failed to start command; turbo_pfor_sys currently only supports Unix");
    assert!(output.status.success(), "{:?}", output);
}

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=src/lib.rs");

    let bindings = bindgen::Builder::default()
        .header(format!("{}/include/vp4.h", SUBMODULE))
        .header(format!("{}/include/fp.h", SUBMODULE))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    let out_path_str = out_path.to_str().unwrap();
    // We do the build in the out dir instead of in source, since it pollutes
    // the lib directory.
    for item in ["src", "include", "Makefile"] {
        run_command(
            "cp",
            vec![
                "-R".to_string(),
                format!("{}/{}", SUBMODULE, item),
                out_path_str.to_string(),
            ],
        );
    }
    run_command("make", vec!["-C", out_path_str, "libic.a"]);

    println!("cargo:rustc-link-search=native={}", out_path_str);
    println!("cargo:rustc-link-lib=static={}", LIB);
}
