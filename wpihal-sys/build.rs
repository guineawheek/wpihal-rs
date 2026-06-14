#![allow(unused)]

use std::{
    collections::BTreeMap,
    fmt::format,
    path::{Path, PathBuf},
    sync::LazyLock,
};

use bindgen::{RustTarget, callbacks::ParseCallbacks};
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
        wpilib_nativeutils::version()
    ));
    let generate_usage_reporting = !cache_marker.exists();

    wpilib_nativeutils::download_native_library_artifacts(
        &repos,
        wpilib_nativeutils::platform(),
        "org.wpilib.hal",
        "hal-cpp",
        wpilib_nativeutils::version(),
        &buildlibs,
        None,
    )
    .unwrap();
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
    wpilib_nativeutils::download_native_library_artifacts(
        &repos,
        wpilib_nativeutils::platform(),
        "org.wpilib.ntcore",
        "ntcore-cpp",
        wpilib_nativeutils::version(),
        &buildlibs,
        None,
    )
    .unwrap();
    wpilib_nativeutils::download_native_library_artifacts(
        &repos,
        wpilib_nativeutils::platform(),
        "org.wpilib.datalog",
        "datalog-cpp",
        wpilib_nativeutils::version(),
        &buildlibs,
        None,
    )
    .unwrap();
    wpilib_nativeutils::download_native_library_artifacts(
        &repos,
        wpilib_nativeutils::platform(),
        "org.wpilib.wpinet",
        "wpinet-cpp",
        wpilib_nativeutils::version(),
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
        r"(HAL_|WPI_|HALSIM_|_HALShim_)\w+",
        "hal_bindings.rs",
    );
    cc::Build::new()
        .cpp(true)
        .file("shim/HALShim.cpp")
        .std("c++20")
        .include(headers)
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
        .opaque_type("std::.*")
        .default_enum_style(bindgen::EnumVariation::Rust {
            non_exhaustive: false,
        })
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .parse_callbacks(Box::new(WPIHalCallbacks {}))
        .generate()
        .expect("Unable to generate bindings");

    bindings
        .write_to_file(&wpilib_nativeutils::out_dir().join(output))
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
            // search `HAL_ENUM` in codebase for instances
            let prefix = match enum_name {
                "HAL_AddressableLEDColorOrder" => "HAL_ALED_",
                "HAL_AlertLevel" => "HAL_ALERT_",
                "HAL_CANDeviceType" => "HAL_CAN_DEV_",
                "HAL_CANManufacturer" => "HAL_CAN_MAN_",
                "HAL_CANFlags" => "HAL_CAN_",
                "HAL_CANBusMap" => "HAL_CAN_BUS_",
                "HAL_AllianceStationID" => "HAL_ALLIANCE_STATION_",
                "HAL_MatchType" => "HAL_MATCH_TYPE_",
                "HAL_RobotMode" => "HAL_ROBOT_MODE_",
                "HAL_JoystickPOV" => "HAL_JOYSTICK_POV_",
                "HAL_EncoderIndexingType" => "HAL_ENCODER_INDEX_",
                "HAL_EncoderEncodingType" => "HAL_",
                "HAL_RuntimeType" => "HAL_RUNTIME_",
                "HAL_I2CPort" => "HAL_I2C_",
                "HAL_PowerDistributionType" => "HAL_POWER_DISTRIBUTION_",
                "HAL_REVPHCompressorConfigType" => "HAL_REVPH_COMPRESSOR_CONFIG_",
                "HAL_SerialPort" => "HAL_SERIAL_PORT_",
                "HAL_SimValueDirection" => "HAL_SIM_VALUE_",
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
