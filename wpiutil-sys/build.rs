#![allow(unused)]

use std::{
    collections::BTreeMap,
    path::{Path, PathBuf},
    sync::LazyLock,
};

use bindgen::{RustTarget, callbacks::ParseCallbacks};
use wpilib_nativeutils::{
    Artifact, ArtifactType, MavenRepo, Platform, ReleaseTrain, is_debug, stringify_path,
};

pub fn main() {
    let local_maven = wpilib_nativeutils::get_local_maven(ReleaseTrain::Release2027);
    let wpilib_maven = wpilib_nativeutils::get_wpilib_maven();
    let remote_maven = wpilib_nativeutils::get_remote_maven(ReleaseTrain::Release2027);
    let repos = [local_maven, wpilib_maven, remote_maven];
    let buildlibs = wpilib_nativeutils::out_dir().join("buildlibs");
    let headers = buildlibs.join("headers");

    wpilib_nativeutils::download_native_library_artifacts(
        &repos,
        wpilib_nativeutils::platform(),
        "org.wpilib.wpiutil",
        "wpiutil-cpp",
        wpilib_nativeutils::version(),
        &buildlibs,
        None,
    )
    .unwrap();

    println!("cargo:rerun-if-changed=UtilsInclude.h");
    wpilib_nativeutils::rustc_link_search(
        &buildlibs,
        wpilib_nativeutils::platform(),
        std::env::var("CARGO_FEATURE_SHARED").is_ok(),
        wpilib_nativeutils::is_debug(),
    );
    wpilib_nativeutils::rustc_debug_switch(&["wpiutil"], wpilib_nativeutils::is_debug());
    generate_bindings_for_header(bindgen::Builder::default(), "bindings.rs");
}

fn generate_bindings_for_header(builder: bindgen::Builder, output: &str) {
    // Some config copied from first-rust-competition https://github.com/first-rust-competition/first-rust-competition/blob/master/hal-gen/src/main.rs
    //const SYMBOL_REGEX: &str = r"(HAL_|HALSIM_)\w+";

    let mut clang_args = vec![
        format!("--target={}", std::env::var("TARGET").unwrap()), // See: https://github.com/rust-lang/rust-bindgen/issues/1760
        "-xc++".to_string(),
        "-std=c++20".to_string(),
        "-v".to_string(),
        "-D_ALLOW_COMPILER_AND_STL_VERSION_MISMATCH".to_string(),
    ];
    wpilib_nativeutils::add_sysroot_to_clang_args(&mut clang_args, wpilib_nativeutils::platform())
        .unwrap();

    let bindings = builder
        .rust_target(RustTarget::stable(85, 0).unwrap())
        .header("UtilInclude.h")
        .derive_default(true)
        .derive_copy(false)
        .clang_arg(format!(
            "-I{}",
            wpilib_nativeutils::stringify_path(
                &wpilib_nativeutils::out_dir().join("buildlibs/headers")
            )
        ))
        .clang_args(&clang_args)
        .allowlist_item(r"WPI_\w+")
        //.allowlist_type(regex)
        //.allowlist_function(regex)
        //.allowlist_var(regex)
        .default_enum_style(bindgen::EnumVariation::Rust {
            non_exhaustive: false,
        })
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .parse_callbacks(Box::new(WPIUtilCallbacks {}))
        .generate()
        .expect("Unable to generate bindings");

    bindings
        .write_to_file(wpilib_nativeutils::out_dir().join(output))
        .expect("Couldn't write bindings!");
}

#[derive(Debug)]
pub struct WPIUtilCallbacks {}

impl ParseCallbacks for WPIUtilCallbacks {
    fn enum_variant_name(
        &self,
        enum_name: Option<&str>,
        original_variant_name: &str,
        _variant_value: bindgen::callbacks::EnumVariantValue,
    ) -> Option<String> {
        None
    }
}
