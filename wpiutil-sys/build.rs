#![allow(unused)]

use std::{collections::BTreeMap, path::{Path, PathBuf}, sync::LazyLock};

use bindgen::callbacks::ParseCallbacks;
use wpilib_nativeutils::{Artifact, ArtifactType, MavenRepo, ReleaseTrain};

static VERSION: LazyLock<String> = LazyLock::new(|| std::env::var("CARGO_PKG_VERSION").unwrap());
static YEAR: LazyLock<String> = LazyLock::new(|| std::env::var("CARGO_PKG_VERSION_MAJOR").unwrap());
static TARGET: LazyLock<String> = LazyLock::new(|| std::env::var("TARGET").unwrap());
const SHARED: bool = cfg!(feature = "shared");
static DEBUG: LazyLock<bool> = LazyLock::new(|| std::env::var("PROFILE").unwrap() == "debug");
static OUT_DIR: LazyLock<PathBuf> = LazyLock::new(|| PathBuf::from(std::env::var("OUT_DIR").unwrap()).canonicalize().unwrap());

pub fn main() {

    let local_maven = wpilib_nativeutils::get_local_maven(ReleaseTrain::Release);
    let wpilib_maven = wpilib_nativeutils::get_wpilib_maven(&YEAR.as_str());
    let remote_maven = wpilib_nativeutils::get_remote_maven(ReleaseTrain::Release);
    let repos = [local_maven, wpilib_maven, remote_maven];

    let buildlibs = OUT_DIR.join("buildlibs");
    let headers = buildlibs.join("headers");
    std::fs::create_dir_all(&headers).unwrap();

    download_artifacts(&repos, "edu.wpi.first.wpiutil", "wpiutil-cpp");

    println!("cargo:rustc-link-search={}", wpilib_nativeutils::lib_search_path(&buildlibs, &TARGET, SHARED).canonicalize().unwrap().to_str().unwrap());
    //println!("cargo:rerun-if-changed=HALInclude.h");
    wpilib_nativeutils::rustc_debug_switch(&["wpiutil"], *DEBUG);
    generate_bindings_for_header(bindgen::Builder::default(), "bindings.rs");
}

fn download_artifacts(repos: &[MavenRepo], group_id: &str, artifact_id: &str) {

    let buildlibs = OUT_DIR.join("buildlibs");
    let cache_marker = buildlibs.join(format!(".nativeutils_downloaded_{group_id}.{artifact_id}-{}", VERSION.as_str()));
    if cache_marker.exists() { return; }

    wpilib_nativeutils::download_artifact_zip_to_dir(&TARGET, &buildlibs.join("headers"), repos, &Artifact {
        artifact_type: ArtifactType::Headers,
        group_id,
        artifact_id,
        version: &VERSION,
    }).unwrap();

    // this is fine for wpilib which disambiguates the artifacts pretty well, but other maven libraries might not 
    // like it if you stuff all their artifacts in the same place with the same names.
    // ideally you want a separate folder for your debug and release artifacts
    for artifact_type in [ArtifactType::Static, ArtifactType::StaticDebug, ArtifactType::Shared, ArtifactType::SharedDebug] {
        wpilib_nativeutils::download_artifact_zip_to_dir(&TARGET, &buildlibs, repos, &Artifact {
            artifact_type,
            group_id,
            artifact_id,
            version: &VERSION,
        }).unwrap();
    }

    std::fs::OpenOptions::new().create(true).write(true).open(cache_marker).ok();

}

fn generate_bindings_for_header(builder: bindgen::Builder, output: &str) {
  // Some config copied from first-rust-competition https://github.com/first-rust-competition/first-rust-competition/blob/master/hal-gen/src/main.rs
  //const SYMBOL_REGEX: &str = r"(HAL_|HALSIM_)\w+";

    let mut clang_args = vec![
        format!("--target={}", *TARGET),    // See: https://github.com/rust-lang/rust-bindgen/issues/1760
        "-xc++".to_string(),
        "-std=c++20".to_string(),
        "-v".to_string()
    ];


    if let Some(sysroot) = wpilib_nativeutils::locate_sysroot(TARGET.as_str(), YEAR.as_str()).unwrap() {
        const PLEASE_USE_UTF8: &str = "your file system path is not utf8 please fix your broken computer";
        clang_args.push(format!("--sysroot={}", sysroot.path().to_str().expect(PLEASE_USE_UTF8)));
        clang_args.push(format!("-I{}", sysroot.cpp_include().expect("can't find c++ headers in the sysroot").to_str().expect(PLEASE_USE_UTF8)));
        if let Some(bits_headers) = sysroot.cpp_bits_include() {
            // only the rio target has a separate bits header path for some reason
            clang_args.push(format!("-I{}", bits_headers.to_str().expect(PLEASE_USE_UTF8)));
        }
    }


  let bindings = builder
    .header("UtilInclude.h")
    .derive_default(true)
    .clang_arg(format!("-I{}", OUT_DIR.join("buildlibs/headers").as_os_str().to_str().unwrap()))
    .clang_args(&clang_args)
    .allowlist_item(r"WPI_\w+")
    //.allowlist_type(regex)
    //.allowlist_function(regex)
    //.allowlist_var(regex)
    .default_enum_style(bindgen::EnumVariation::Rust { non_exhaustive: false })
    .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
    .parse_callbacks(Box::new(WPIUtilCallbacks{}))
    .generate()
    .expect("Unable to generate bindings");

  bindings
    .write_to_file(OUT_DIR.join(output))
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