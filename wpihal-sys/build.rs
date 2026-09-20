#![allow(unused)]

use std::{
    collections::{BTreeMap, HashMap},
    fmt::format,
    path::{Path, PathBuf},
    sync::LazyLock,
};

use bindgen::{RustTarget, callbacks::ParseCallbacks};
use convert_case::Casing;
use wpilib_native_utils::{
    Artifact, ArtifactType, MavenRepo, Platform, ReleaseTrain, WPILibVersion,
};

pub fn main() {
    let wpilib_version = wpilib_native_utils::bind_version();
    let repos = wpilib_version.get_mavens(ReleaseTrain::Release);
    let buildlibs = wpilib_native_utils::out_dir().join("buildlibs");

    let version = wpilib_version.to_string();
    let shared = std::env::var("CARGO_FEATURE_SHARED").is_ok();
    let debug = wpilib_native_utils::is_debug();
    let platform = wpilib_native_utils::platform();

    let artifacts = [
        Artifact::new(
            "org.wpilib.hal",
            "hal-cpp",
            &version,
            ArtifactType::native("wpiHal", shared, debug),
        ),
        Artifact::new(
            "org.wpilib.wpiutil",
            "wpiutil-cpp",
            &version,
            ArtifactType::native("wpiutil", shared, debug),
        ),
        Artifact::new(
            "org.wpilib.ntcore",
            "ntcore-cpp",
            &version,
            ArtifactType::native("ntcore", shared, debug),
        ),
        Artifact::new(
            "org.wpilib.datalog",
            "datalog-cpp",
            &version,
            ArtifactType::native("datalog", shared, debug),
        ),
        Artifact::new(
            "org.wpilib.wpinet",
            "wpinet-cpp",
            &version,
            ArtifactType::native("wpinet", shared, debug),
        ),
        Artifact::new(
            "org.wpilib.mrclib",
            "mrclib-cpp",
            "2027.1.0-alpha-1-116-g5288562",
            ArtifactType::SharedOnly("MrcLib".to_string()),
        ),
    ]
    .into_iter()
    .map(|a| a.with_headers())
    .flatten();

    wpilib_native_utils::download_artifacts(platform, &repos, artifacts, &buildlibs).unwrap();
    wpilib_native_utils::rustc_link_search(&buildlibs, platform, shared);
    generate_bindings_for_header(
        &wpilib_version,
        bindgen::Builder::default(),
        "shim/HALInclude.h",
        r"(HAL_|HALSIM_|_HALShim_)\w+",
        "hal_bindings.rs",
    );
    println!("cargo:rerun-if-changed=shim");
    cc::Build::new()
        .cpp(true)
        .file("shim/HALShim.cpp")
        .std("c++20")
        .include(wpilib_native_utils::fix_windows(&buildlibs.join("headers")))
        .compile("HALShim");
}

fn generate_bindings_for_header(
    wpilib_version: &WPILibVersion,
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
        "-std=c++23".to_string(),
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
        .header(header)
        .derive_default(true)
        .derive_partialeq(true)
        .clang_arg(format!(
            "-I{}",
            wpilib_native_utils::stringify_path(
                &wpilib_native_utils::out_dir().join("buildlibs/headers")
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
        .write_to_file(&wpilib_native_utils::out_dir().join(output))
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
