#![allow(unused)]

use std::{sync::LazyLock, path::{Path, PathBuf}};

use wpilib_nativeutils::{Artifact, ArtifactType, MavenRepo, ReleaseTrain};

static VERSION: LazyLock<String> = LazyLock::new(|| std::env::var("CARGO_PKG_VERSION").unwrap());
static YEAR: LazyLock<String> = LazyLock::new(|| std::env::var("CARGO_PKG_VERSION_MAJOR").unwrap());
static TARGET: LazyLock<String> = LazyLock::new(|| std::env::var("TARGET").unwrap());
const SHARED: bool = cfg!(feature = "shared");
static DEBUG: LazyLock<bool> = LazyLock::new(|| std::env::var("PROFILE").unwrap() == "debug");
static OUT_DIR: LazyLock<PathBuf> = LazyLock::new(|| PathBuf::from(std::env::var("OUT_DIR").unwrap()));

pub fn main() {

    let local_maven = wpilib_nativeutils::get_local_maven(ReleaseTrain::Release);
    let wpilib_maven = wpilib_nativeutils::get_wpilib_maven(&YEAR.as_str());
    let remote_maven = wpilib_nativeutils::get_remote_maven(ReleaseTrain::Release);
    let repos = [local_maven, wpilib_maven, remote_maven];

    let buildlibs = OUT_DIR.join("buildlibs");
    let headers = buildlibs.join("headers");
    std::fs::create_dir_all(&headers).unwrap();

    download_artifacts(&repos, "edu.wpi.first.hal", "hal-cpp");
    download_artifacts(&repos, "edu.wpi.first.wpiutil", "wpiutil-cpp");

    println!("cargo:rustc-link-search={}", wpilib_nativeutils::lib_search_path(&buildlibs, &TARGET, SHARED).canonicalize().unwrap().to_str().unwrap());
    println!("cargo:rerun-if-changed=HALInclude.h");
    wpilib_nativeutils::rustc_debug_switch(&["wpiHal", "wpiutil"], *DEBUG);
    generate_bindings_for_header(
        bindgen::Builder::default(),
        "HALInclude.h", r"(HAL_|WPI_)\w+", "hal_bindings.rs");
    //generate_bindings_for_header(bindgen::Builder::default(), "HALSIMInclude.h", r"(HALSIM_)\w+", "sim_bindings.rs");
    generate_bindings_for_header(bindgen::Builder::default(), headers.join("hal/Errors.h").as_os_str().to_str().unwrap(), ".*", "error_bindings.rs");
    //generate_bindings_for_header("WPIInclude.h", r"(WPI_)\w+", "wpi_bindings.rs");
}

fn download_artifacts(repos: &[MavenRepo], group_id: &str, artifact_id: &str) {

    let buildlibs = OUT_DIR.join("buildlibs");
    let cache_marker = buildlibs.join(format!(".nativeutils_downloaded_{group_id}.{artifact_id}-{}", VERSION.as_str()));
    if cache_marker.exists() { return; }

    let (link_shared, link_static) = if *DEBUG { 
        (ArtifactType::Shared, ArtifactType::Static)
    } else {
        (ArtifactType::SharedDebug, ArtifactType::StaticDebug)
    };

    wpilib_nativeutils::download_artifact_zip_to_dir(&TARGET, &buildlibs.join("headers"), repos, &Artifact {
        artifact_type: ArtifactType::Headers,
        group_id,
        artifact_id,
        version: &VERSION,
    }).unwrap();

    wpilib_nativeutils::download_artifact_zip_to_dir(&TARGET, &buildlibs, repos, &Artifact {
        artifact_type: link_shared,
        group_id,
        artifact_id,
        version: &VERSION,
    }).unwrap();

    wpilib_nativeutils::download_artifact_zip_to_dir(&TARGET, &buildlibs, repos, &Artifact {
        artifact_type: link_static,
        group_id,
        artifact_id,
        version: &VERSION
    }).unwrap();

    std::fs::OpenOptions::new().create(true).write(true).open(cache_marker).ok();

}

fn generate_bindings_for_header(builder: bindgen::Builder, header: &str, regex: &str, output: &str) {
  // Some config copied from first-rust-competition https://github.com/first-rust-competition/first-rust-competition/blob/master/hal-gen/src/main.rs
  //const SYMBOL_REGEX: &str = r"(HAL_|HALSIM_)\w+";
  let bindings = builder
    .header(header)
    .derive_default(true)
    .clang_arg(format!("-I{}", OUT_DIR.join("buildlibs/headers").as_os_str().to_str().unwrap()))
    .allowlist_type(regex)
    .allowlist_function(regex)
    .allowlist_var(regex)
    .default_enum_style(bindgen::EnumVariation::Rust { non_exhaustive: false })
    .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
    .clang_args(&[
      format!("--target={}", *TARGET),    // See: https://github.com/rust-lang/rust-bindgen/issues/1760
    ])
    .clang_arg("-xc++")
    .clang_arg("-std=c++20")
    .generate()
    .expect("Unable to generate bindings");

  bindings
    .write_to_file(OUT_DIR.join(output))
    .expect("Couldn't write bindings!");
}