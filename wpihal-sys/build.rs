#![allow(unused)]

use std::{collections::BTreeMap, fmt::format, path::{Path, PathBuf}, sync::LazyLock};

use bindgen::callbacks::ParseCallbacks;
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
        "HALInclude.h", r"(HAL_|WPI_|HALSIM_)\w+", "hal_bindings.rs");
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

    create_usage_reporting(
        &OUT_DIR.join("buildlibs/headers/hal/FRCUsageReporting.h"),
        &OUT_DIR.join("usage_reporting.rs")
    );

    // 

    std::fs::OpenOptions::new().create(true).write(true).open(cache_marker).ok();

}

fn generate_bindings_for_header(builder: bindgen::Builder, header: &str, regex: &str, output: &str) {
    // Some config copied from first-rust-competition https://github.com/first-rust-competition/first-rust-competition/blob/master/hal-gen/src/main.rs
    //const SYMBOL_REGEX: &str = r"(HAL_|HALSIM_)\w+";
    let mut clang_args = vec![
        format!("--target={}", *TARGET),    // See: https://github.com/rust-lang/rust-bindgen/issues/1760
        "-xc++".to_string(),
        "-std=c++20".to_string()
    ];
    if let Some(sysroot) = wpilib_nativeutils::locate_sysroot(TARGET.as_str(), YEAR.as_str()).unwrap() {
        const PLEASE_USE_UTF8: &str = "your file system path is not utf8 please fix your broken computer";
        eprintln!("Located sysroot at {:?}", sysroot.path());
        eprintln!("Located sysroot c++ at {:?}", sysroot.cpp_include());
        clang_args.push(format!("--sysroot={}", sysroot.path().to_str().expect(PLEASE_USE_UTF8)));
        clang_args.push(format!("-I{}", sysroot.cpp_include().expect("can't find c++ headers in the sysroot").to_str().expect(PLEASE_USE_UTF8)));
        if let Some(bits_headers) = sysroot.cpp_bits_include() {
            // only the rio target has a separate bits header path for some reason
            clang_args.push(format!("-I{}", bits_headers.to_str().expect(PLEASE_USE_UTF8)));
        }
    }

    let bindings = builder
      .header(header)
      .derive_default(true)
      .clang_arg(format!("-I{}", OUT_DIR.join("buildlibs/headers").as_os_str().to_str().unwrap()))
      .allowlist_type(regex)
      .allowlist_function(regex)
      .allowlist_var(regex)
      .default_enum_style(bindgen::EnumVariation::Rust { non_exhaustive: false })
      .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
      .parse_callbacks(Box::new(WPIHalCallbacks{}))
      .clang_args(&clang_args)
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
                _ => { return None; }
            };

            Some(original_variant_name.strip_prefix(prefix).unwrap().to_string())
        }
    }
}
pub struct ResourceEnumBuilder {
    name: String,
    variants: BTreeMap<String, i32>
}

impl ResourceEnumBuilder {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            variants: Default::default(),
        }
    }
    pub fn generate_enum(&self) -> String {
        let mut s = format!("#[derive(Debug, Copy, Clone, PartialEq, Eq)]\n#[repr(i32)]\npub enum {} {{\n", self.name);
        let mut variants: Vec<(&String, &i32)> = self.variants.iter().collect();
        variants.sort_by(|(_, v1), (_, v2)| { v1.cmp(v2) });
        for (k, v) in variants {
            s.push_str(format!("    k{k} = {v},\n").as_str());
        }
        s.push_str("}\n");
        s
    }
}

fn create_usage_reporting(header: &PathBuf, output: &PathBuf) {
    let file = std::fs::read_to_string(header).unwrap();
    let re = regex::Regex::new(r"\s+k([a-zA-Z0-9]+)_([a-zA-Z0-9_]+) = ([0-9]+),").unwrap();
    let mut enum_ents: BTreeMap<&str, ResourceEnumBuilder> = Default::default();

    for (_, [enum_name, enum_var, value]) in re.captures_iter(file.as_str()).map(|cap| cap.extract()) {
        if !enum_ents.contains_key(enum_name) {
            enum_ents.insert(enum_name, ResourceEnumBuilder::new(enum_name));
        }
        let ent = enum_ents.get_mut(enum_name).unwrap();

        ent.variants.insert(enum_var.to_string(), value.parse::<i32>().unwrap());
    }

    let mut usage_module = String::new();

    for ent in enum_ents.values() {
        usage_module.push_str(&ent.generate_enum());
    }

    std::fs::write(output, usage_module).unwrap();
}