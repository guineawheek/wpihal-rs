use bindgen::{RustTarget, callbacks::ParseCallbacks};
use wpilib_native_utils::{Artifact, ArtifactType, ReleaseTrain, WPILibVersion};

pub fn main() {
    let wpilib_version = wpilib_native_utils::bind_version();
    let repos = wpilib_version.get_mavens(ReleaseTrain::Release);
    let buildlibs = wpilib_native_utils::out_dir().join("buildlibs");

    let version = wpilib_version.to_string();
    let shared = std::env::var("CARGO_FEATURE_SHARED").is_ok();
    let debug = wpilib_native_utils::is_debug();
    let platform = wpilib_native_utils::platform();

    let artifacts = [Artifact::new(
        "org.wpilib.wpiutil",
        "wpiutil-cpp",
        &version,
        ArtifactType::native("wpiutil", shared, debug),
    )
    .with_headers()];

    wpilib_native_utils::download_artifacts(
        platform,
        &repos,
        artifacts.into_iter().flatten(),
        &buildlibs,
    )
    .unwrap();

    println!("cargo:rerun-if-changed=UtilsInclude.h");
    wpilib_native_utils::rustc_link_search(&buildlibs, platform, shared);
    generate_bindings_for_header(&wpilib_version, bindgen::Builder::default(), "bindings.rs");
}

fn generate_bindings_for_header(
    wpilib_version: &WPILibVersion,
    builder: bindgen::Builder,
    output: &str,
) {
    // Some config copied from first-rust-competition https://github.com/first-rust-competition/first-rust-competition/blob/master/hal-gen/src/main.rs
    //const SYMBOL_REGEX: &str = r"(HAL_|HALSIM_)\w+";

    let mut clang_args = vec![
        format!("--target={}", std::env::var("TARGET").unwrap()), // See: https://github.com/rust-lang/rust-bindgen/issues/1760
        "-xc++".to_string(),
        "-std=c++20".to_string(),
        "-v".to_string(),
        "-D_ALLOW_COMPILER_AND_STL_VERSION_MISMATCH".to_string(),
    ];
    wpilib_native_utils::add_sysroot_to_clang_args(
        &mut clang_args,
        wpilib_native_utils::platform(),
        wpilib_version,
    )
    .unwrap();

    let bindings = builder
        .rust_target(RustTarget::stable(85, 0).unwrap())
        .header("UtilInclude.h")
        .derive_default(true)
        .derive_copy(false)
        .clang_arg(format!(
            "-I{}",
            wpilib_native_utils::stringify_path(
                &wpilib_native_utils::out_dir().join("buildlibs/headers")
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
        .write_to_file(wpilib_native_utils::out_dir().join(output))
        .expect("Couldn't write bindings!");
}

#[derive(Debug)]
pub struct WPIUtilCallbacks {}

impl ParseCallbacks for WPIUtilCallbacks {
    fn enum_variant_name(
        &self,
        _enum_name: Option<&str>,
        _original_variant_name: &str,
        _variant_value: bindgen::callbacks::EnumVariantValue,
    ) -> Option<String> {
        None
    }
}
