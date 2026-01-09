#![allow(unused)]

use std::{
    collections::BTreeMap,
    fmt::format,
    path::{Path, PathBuf},
    sync::LazyLock,
};

use bindgen::{RustTarget, callbacks::ParseCallbacks};
use wpilib_nativeutils::{Artifact, ArtifactType, MavenRepo, Platform, ReleaseTrain};

const NI_VERSION: &'static str = "2026.1.0";
static VERSION: LazyLock<String> = LazyLock::new(|| std::env::var("CARGO_PKG_VERSION").unwrap());
static YEAR: LazyLock<String> = LazyLock::new(|| "2027_alpha1".to_string());
static PLATFORM: LazyLock<Platform> = LazyLock::new(|| {
    Platform::from_rust_target(
        &std::env::var("TARGET").unwrap(),
        std::env::var("CARGO_FEATURE_ROBOT_CONTROLLER").is_ok(),
    )
    .expect("Invalid build target")
});
static DEBUG: LazyLock<bool> = LazyLock::new(|| std::env::var("PROFILE").unwrap() == "debug");
static OUT_DIR: LazyLock<PathBuf> = LazyLock::new(|| {
    PathBuf::from(std::env::var("OUT_DIR").unwrap())
        .canonicalize()
        .unwrap()
});

pub fn main() {
    let local_maven = wpilib_nativeutils::get_local_maven(ReleaseTrain::Release2027);
    let wpilib_maven = wpilib_nativeutils::get_wpilib_maven(&YEAR.as_str());
    let remote_maven = wpilib_nativeutils::get_remote_maven(ReleaseTrain::Release2027);
    let repos = [local_maven, wpilib_maven, remote_maven];
    let buildlibs = OUT_DIR.join("buildlibs");
    let headers = buildlibs.join("headers");

    let cache_marker = buildlibs.join(format!(
        ".nativeutils_downloaded_edu.wpi.first.hal.hal-cpp-{}",
        VERSION.as_str()
    ));
    let generate_usage_reporting = !cache_marker.exists();

    wpilib_nativeutils::download_native_library_artifacts(
        &repos,
        *PLATFORM,
        "edu.wpi.first.hal",
        "hal-cpp",
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
        "edu.wpi.first.datalog",
        "datalog-cpp",
        &VERSION,
        &buildlibs,
        None,
    )
    .unwrap();
    wpilib_nativeutils::download_native_library_artifacts(
        &repos,
        *PLATFORM,
        "edu.wpi.first.wpinet",
        "wpinet-cpp",
        &VERSION,
        &buildlibs,
        None,
    )
    .unwrap();
    println!("cargo:rerun-if-changed=HALInclude.h");
    wpilib_nativeutils::rustc_link_search(
        &buildlibs,
        *PLATFORM,
        std::env::var("CARGO_FEATURE_SHARED").is_ok(),
        *DEBUG,
    );
    wpilib_nativeutils::rustc_debug_switch(
        &["wpiHal", "wpiutil", "ntcore", "datalog", "wpinet"],
        *DEBUG,
    );
    generate_bindings_for_header(
        bindgen::Builder::default(),
        "HALInclude.h",
        r"(HAL_|WPI_|HALSIM_)\w+",
        "hal_bindings.rs",
    );
    generate_bindings_for_header(
        bindgen::Builder::default(),
        headers.join("hal/Errors.h").as_os_str().to_str().unwrap(),
        ".*",
        "error_bindings.rs",
    );
}

fn generate_bindings_for_header(
    builder: bindgen::Builder,
    header: &str,
    regex: &str,
    output: &str,
) {
    // Some config copied from first-rust-competition https://github.com/first-rust-competition/first-rust-competition/blob/master/hal-gen/src/main.rs
    //const SYMBOL_REGEX: &str = r"(HAL_|HALSIM_)\w+";
    let mut clang_args = vec![
        format!("--target={}", std::env::var("TARGET").unwrap()), // See: https://github.com/rust-lang/rust-bindgen/issues/1760
        "-xc++".to_string(),
        "-std=c++20".to_string(),
        "-D_ALLOW_COMPILER_AND_STL_VERSION_MISMATCH".to_string(),
    ];
    wpilib_nativeutils::add_sysroot_to_clang_args(&mut clang_args, *PLATFORM, &YEAR).unwrap();

    let bindings = builder
        .rust_target(RustTarget::stable(85, 0).unwrap())
        .header(header)
        .derive_default(true)
        .clang_arg(format!(
            "-I{}",
            wpilib_nativeutils::stringify_path(&OUT_DIR.join("buildlibs/headers"))
        ))
        .clang_args(&clang_args)
        .allowlist_type(regex)
        .allowlist_function(regex)
        .allowlist_var(regex)
        .opaque_type("std::.*")
        .default_enum_style(bindgen::EnumVariation::Rust {
            non_exhaustive: false,
        })
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .parse_callbacks(Box::new(WPIHalCallbacks {}))
        .generate()
        .expect("Unable to generate bindings");

    bindings
        .write_to_file(OUT_DIR.join(output))
        .expect("Couldn't write bindings!");
}

#[derive(Debug)]
pub struct WPIHalCallbacks {}

impl ParseCallbacks for WPIHalCallbacks {
    fn enum_variant_name(
        &self,
        enum_name: Option<&str>,
        original_variant_name: &str,
        _variant_value: bindgen::callbacks::EnumVariantValue,
    ) -> Option<String> {
        let enum_name = enum_name?;
        let name = format!("{}_", enum_name);
        if original_variant_name.starts_with(name.as_str()) {
            let ov_name = original_variant_name.strip_prefix(name.as_str()).unwrap();
            Some(ov_name.to_string())
        } else {
            // rewrite enums to not have prefixes
            let prefix = match enum_name {
                "HAL_AnalogTriggerType" => "HAL_Trigger_",
                "HAL_CANManufacturer" => "HAL_CAN_Man_",
                "HAL_CANDeviceType" => "HAL_CAN_Dev_",
                "HAL_Counter_Mode" => "HAL_Counter_",
                "HAL_MatchType" => "HAL_",
                "HAL_EncoderIndexingType" => "HAL_",
                "HAL_EncoderEncodingType" => "HAL_Encoder_",
                "HAL_I2CPort" => "HAL_I2C_",
                "HAL_RadioLEDState" => "HAL_RadioLED_",
                "HAL_SPIPort" => "HAL_SPI_",
                "HAL_SPIMode" => "HAL_SPI_",
                _ => {
                    return None;
                }
            };

            Some(
                original_variant_name
                    .strip_prefix(prefix)
                    .unwrap()
                    .to_string(),
            )
        }
    }
}
pub struct ResourceEnumBuilder {
    name: String,
    variants: BTreeMap<String, i32>,
}

impl ResourceEnumBuilder {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            variants: Default::default(),
        }
    }
    pub fn generate_enum(&self) -> String {
        let mut s = format!(
            "#[derive(Debug, Copy, Clone, PartialEq, Eq)]\n#[repr(i32)]\npub enum {} {{\n",
            self.name
        );
        let mut variants: Vec<(&String, &i32)> = self.variants.iter().collect();
        variants.sort_by(|(_, v1), (_, v2)| v1.cmp(v2));
        for (k, v) in variants {
            s.push_str(format!("    k{k} = {v},\n").as_str());
        }
        s.push_str("}\n");
        s
    }
}
