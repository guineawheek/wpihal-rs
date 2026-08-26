#![allow(unused)]

use std::{
    collections::{BTreeMap, HashMap},
    fmt::format,
    path::{Path, PathBuf},
    sync::LazyLock,
};

use bindgen::{RustTarget, callbacks::ParseCallbacks};
use convert_case::Casing;
use wpilib_nativeutils::{Artifact, ArtifactType, MavenRepo, Platform, ReleaseTrain};

pub fn main() {
    let local_maven = wpilib_nativeutils::get_local_maven(ReleaseTrain::Release2027);
    let wpilib_maven = wpilib_nativeutils::get_wpilib_maven();
    let remote_maven = wpilib_nativeutils::get_remote_maven(ReleaseTrain::Release2027);
    let repos = [local_maven, wpilib_maven, remote_maven];
    let buildlibs = wpilib_nativeutils::out_dir().join("buildlibs");
    let headers = buildlibs.join("headers");

    let cache_marker = buildlibs.join(format!(
        ".nativeutils_downloaded_org.wpilib.hal.hal-cpp-{}",
        wpilib_nativeutils::VERSION,
    ));
    let generate_usage_reporting = !cache_marker.exists();

    wpilib_nativeutils::download_native_library_artifacts(
        &repos,
        wpilib_nativeutils::platform(),
        "org.wpilib.hal",
        "hal-cpp",
        wpilib_nativeutils::VERSION,
        &buildlibs,
        None,
    )
    .unwrap();
    wpilib_nativeutils::download_native_library_artifacts(
        &repos,
        wpilib_nativeutils::platform(),
        "org.wpilib.wpiutil",
        "wpiutil-cpp",
        wpilib_nativeutils::VERSION,
        &buildlibs,
        None,
    )
    .unwrap();
    wpilib_nativeutils::download_native_library_artifacts(
        &repos,
        wpilib_nativeutils::platform(),
        "org.wpilib.ntcore",
        "ntcore-cpp",
        wpilib_nativeutils::VERSION,
        &buildlibs,
        None,
    )
    .unwrap();
    wpilib_nativeutils::download_native_library_artifacts(
        &repos,
        wpilib_nativeutils::platform(),
        "org.wpilib.datalog",
        "datalog-cpp",
        wpilib_nativeutils::VERSION,
        &buildlibs,
        None,
    )
    .unwrap();
    wpilib_nativeutils::download_native_library_artifacts(
        &repos,
        wpilib_nativeutils::platform(),
        "org.wpilib.wpinet",
        "wpinet-cpp",
        wpilib_nativeutils::VERSION,
        &buildlibs,
        None,
    )
    .unwrap();
    println!("cargo:rerun-if-changed=shim");
    wpilib_nativeutils::rustc_link_search(
        &buildlibs,
        wpilib_nativeutils::platform(),
        std::env::var("CARGO_FEATURE_SHARED").is_ok(),
        wpilib_nativeutils::is_debug(),
    );
    wpilib_nativeutils::rustc_debug_switch(
        &["wpiHal", "wpiutil", "ntcore", "datalog", "wpinet"],
        wpilib_nativeutils::is_debug(),
    );
    generate_bindings_for_header(
        bindgen::Builder::default(),
        "shim/HALInclude.h",
        r"(HAL_|HALSIM_|_HALShim_)\w+",
        "hal_bindings.rs",
    );
    cc::Build::new()
        .cpp(true)
        .file("shim/HALShim.cpp")
        .std("c++20")
        .include(wpilib_nativeutils::fix_paths(&headers))
        .compile("HALShim");
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
    wpilib_nativeutils::add_sysroot_to_clang_args(&mut clang_args, wpilib_nativeutils::platform())
        .unwrap();

    let bindings = builder
        .rust_target(RustTarget::stable(85, 0).unwrap())
        .header(header)
        .derive_default(true)
        .derive_partialeq(true)
        .clang_arg(format!(
            "-I{}",
            wpilib_nativeutils::stringify_path(
                &wpilib_nativeutils::out_dir().join("buildlibs/headers")
            )
        ))
        .clang_args(&clang_args)
        .allowlist_type(regex)
        .allowlist_function(regex)
        .allowlist_var(regex)
        .blocklist_type(r"WPI_\w+")
        .opaque_type("std::.*")
        .default_enum_style(bindgen::EnumVariation::Rust {
            non_exhaustive: false,
        })
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .parse_callbacks(Box::new(WPIHalCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings");

    bindings
        .write_to_file(&wpilib_nativeutils::out_dir().join(output))
        .expect("Couldn't write bindings!");
}

#[derive(Debug, Copy, Clone)]
struct HalEnum {
    prefix: &'static str,
    name: &'static str,
    variant_prefix: &'static str,
}

impl HalEnum {
    pub const fn hal(name: &'static str, variant_prefix: &'static str) -> Self {
        Self {
            prefix: "HAL",
            name,
            variant_prefix,
        }
    }
}

const ENUMS_TO_FROBNICATE: &[HalEnum] = &[
    HalEnum::hal("AddressableLEDColorOrder", "HAL_ALED_"),
    HalEnum::hal("AlertLevel", "HAL_ALERT_"),
    HalEnum::hal("CANDeviceType", "HAL_CAN_DEV_"),
    HalEnum::hal("CANManufacturer", "HAL_CAN_MAN_"),
    HalEnum::hal("CANFlags", "HAL_CAN_"),
    HalEnum::hal("CANBusMap", "HAL_CAN_BUS_"),
    HalEnum::hal("AllianceStationID", "HAL_ALLIANCE_STATION_"),
    HalEnum::hal("MatchType", "HAL_MATCH_TYPE_"),
    HalEnum::hal("RobotMode", "HAL_ROBOT_MODE_"),
    HalEnum::hal("JoystickPOV", "HAL_JOYSTICK_POV_"),
    HalEnum::hal("EncoderIndexingType", "HAL_ENCODER_INDEX_"),
    HalEnum::hal("EncoderEncodingType", "HAL_ENCODER_"),
    HalEnum::hal("RuntimeType", "HAL_RUNTIME_"),
    HalEnum::hal("I2CPort", "HAL_I2C_"),
    HalEnum::hal("PowerDistributionType", "HAL_POWER_DISTRIBUTION_"),
    HalEnum::hal("REVPHCompressorConfigType", "HAL_REVPH_COMPRESSOR_CONFIG_"),
    HalEnum::hal("SerialPort", "HAL_SERIAL_PORT_"),
    HalEnum::hal("SimValueDirection", "HAL_SIM_VALUE_"),
];

#[derive(Debug)]
pub struct WPIHalCallbacks {
    enum_map: HashMap<String, HalEnum>,
}

impl WPIHalCallbacks {
    pub fn new() -> Self {
        Self {
            enum_map: HashMap::from_iter(
                ENUMS_TO_FROBNICATE
                    .iter()
                    .map(|he| (format!("{}_{}", he.prefix, he.name), *he)),
            ),
        }
    }
}

impl ParseCallbacks for WPIHalCallbacks {
    fn enum_variant_name(
        &self,
        enum_name: Option<&str>,
        original_variant_name: &str,
        _variant_value: bindgen::callbacks::EnumVariantValue,
    ) -> Option<String> {
        let enum_name = enum_name?;
        //let hal_enum_name = format!("HAL_{enum_name}");
        let mut proposed_rename = if let Some(hal_enum) = self.enum_map.get(enum_name).copied() {
            // rewrite enums to not have prefixes
            // search `HAL_ENUM` in codebase for instances

            original_variant_name
                .strip_prefix(hal_enum.variant_prefix)
                .unwrap()
                .to_case(convert_case::Case::Pascal)
        } else {
            original_variant_name.to_case(convert_case::Case::Pascal)
        };

        if proposed_rename.chars().nth(0)?.is_ascii_digit() {
            proposed_rename = format!("k{proposed_rename}");
        }

        Some(proposed_rename)
    }
}
