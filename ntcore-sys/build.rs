#![allow(unused)]

use std::{
    collections::BTreeMap,
    path::{Path, PathBuf},
    sync::LazyLock,
};

use bindgen::callbacks::ParseCallbacks;
use convert_case::Casing;
use wpilib_nativeutils::{
    stringify_path, Artifact, ArtifactType, MavenRepo, Platform, ReleaseTrain,
};

static VERSION: LazyLock<String> = LazyLock::new(|| std::env::var("CARGO_PKG_VERSION").unwrap());
static YEAR: LazyLock<String> = LazyLock::new(|| std::env::var("CARGO_PKG_VERSION_MAJOR").unwrap());
static PLATFORM: LazyLock<Platform> = LazyLock::new(|| {
    Platform::from_rust_target(&std::env::var("TARGET").unwrap()).expect("Invalid build target")
});
const SHARED: bool = cfg!(feature = "shared");
static DEBUG: LazyLock<bool> = LazyLock::new(|| std::env::var("PROFILE").unwrap() == "debug");
static OUT_DIR: LazyLock<PathBuf> = LazyLock::new(|| {
    PathBuf::from(std::env::var("OUT_DIR").unwrap())
        .canonicalize()
        .unwrap()
});
static TARGET_DIR: LazyLock<PathBuf> = LazyLock::new(|| OUT_DIR.join("../../.."));

//CARGO_TARGET_DIR

pub fn main() {
    let local_maven = wpilib_nativeutils::get_local_maven(ReleaseTrain::Release);
    let wpilib_maven = wpilib_nativeutils::get_wpilib_maven(&YEAR.as_str());
    let remote_maven = wpilib_nativeutils::get_remote_maven(ReleaseTrain::Release);
    let repos = [local_maven, wpilib_maven, remote_maven];
    let buildlibs = TARGET_DIR.join("buildlibs");
    let headers = buildlibs.join("headers");

    wpilib_nativeutils::download_native_library_artifacts(
        &repos,
        *PLATFORM,
        "edu.wpi.first.ntcore",
        "ntcore-cpp",
        &VERSION,
        &buildlibs,
        None,
    )
    .unwrap();

    wpilib_nativeutils::download_native_library_artifacts(
        &repos,
        *PLATFORM,
        "edu.wpi.first.wpiutil",
        "wpiutil-cpp",
        &VERSION,
        &buildlibs,
        None,
    )
    .unwrap();

    println!("cargo:rerun-if-changed=NTCoreInclude.h");
    println!("cargo:rerun-if-changed=NTCoreShim.h");
    println!("cargo:rerun-if-changed=ntcore_rs_shim.cpp");
    wpilib_nativeutils::rustc_link_search(&buildlibs, *PLATFORM, SHARED, *DEBUG);
    wpilib_nativeutils::rustc_debug_switch(&["ntcore", "wpiutil"], *DEBUG);
    generate_bindings_for_header(bindgen::Builder::default(), "bindings.rs");
    cc::Build::new()
        .cpp(true)
        .file("ntcore_rs_shim.cpp")
        .std("c++20")
        .include(headers)
        .compile("ntcore_rs_shim");
}

fn generate_bindings_for_header(builder: bindgen::Builder, output: &str) {
    // Some config copied from first-rust-competition https://github.com/first-rust-competition/first-rust-competition/blob/master/hal-gen/src/main.rs
    //const SYMBOL_REGEX: &str = r"(HAL_|HALSIM_)\w+";

    let mut clang_args = vec![
        format!("--target={}", std::env::var("TARGET").unwrap()), // See: https://github.com/rust-lang/rust-bindgen/issues/1760
        "-xc++".to_string(),
        "-std=c++20".to_string(),
        "-v".to_string(),
    ];
    wpilib_nativeutils::add_sysroot_to_clang_args(&mut clang_args, *PLATFORM, &YEAR).unwrap();

    let bindings = builder
        .header("NTCoreInclude.h")
        .derive_default(true)
        .clang_arg(format!(
            "-I{}",
            wpilib_nativeutils::stringify_path(&TARGET_DIR.join("buildlibs/headers"))
        ))
        .clang_args(&clang_args)
        .opaque_type("std::.*")
        .allowlist_item(r"WPI_\w+")
        .allowlist_item(r"NT_\w+")
        .allowlist_item(r"NTCoreRS_\w+")
        //.allowlist_file(r"^.*ntcore_c.h$")
        //.allowlist_type(regex)
        //.allowlist_function(regex)
        //.allowlist_var(regex)
        .default_enum_style(bindgen::EnumVariation::Rust {
            non_exhaustive: false,
        })
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .parse_callbacks(Box::new(NTCoreCallbacks {}))
        .generate()
        .expect("Unable to generate bindings");

    bindings
        .write_to_file(OUT_DIR.join(output))
        .expect("Couldn't write bindings!");
}

#[derive(Debug)]
pub struct NTCoreCallbacks {}

impl ParseCallbacks for NTCoreCallbacks {
    fn enum_variant_name(
        &self,
        enum_name: Option<&str>,
        original_variant_name: &str,
        _variant_value: bindgen::callbacks::EnumVariantValue,
    ) -> Option<String> {
        if let Some(nt_variant) = original_variant_name.strip_prefix("NT_") {
            return Some(nt_variant.to_case(convert_case::Case::Pascal));
        }

        None
    }

    fn item_name(&self, item_info: bindgen::callbacks::ItemInfo) -> Option<String> {
        match item_info.name {
            "NT_Type" | "NT_EntryFlags" | "NT_LogLevel" | "NT_NetworkMode" | "NT_EventFlags" => {
                Some(item_info.name.to_case(convert_case::Case::Pascal))
            }
            _ => None,
        }
    }
}
