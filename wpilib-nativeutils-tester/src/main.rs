#![allow(dead_code)]
use std::{collections::BTreeMap, path::PathBuf};

use wpilib_nativeutils::{Artifact, ArtifactType, Platform, ReleaseTrain};

fn main() {
    let version = env!("CARGO_PKG_VERSION");
    let year = env!("CARGO_PKG_VERSION_MAJOR");
    //let target = "aarch64-apple-darwin";
    let platform = Platform::OsxUniversal;
    let local_maven = wpilib_nativeutils::get_local_maven(ReleaseTrain::Release);
    let wpilib_maven = wpilib_nativeutils::get_wpilib_maven(year);
    let remote_maven = wpilib_nativeutils::get_remote_maven(ReleaseTrain::Release);
    let repos = [local_maven, wpilib_maven, remote_maven];

    //let out_path = PathBuf::from(std::env::var("OUT_DIR").unwrap());
    let out_path = PathBuf::from("target");
    let buildlibs = out_path.join("buildlibs");
    std::fs::create_dir_all(&buildlibs).unwrap();

    wpilib_nativeutils::download_artifact_zip_to_dir(
        platform,
        &buildlibs,
        &repos,
        &Artifact {
            artifact_type: ArtifactType::Headers,
            group_id: "edu.wpi.first.hal",
            artifact_id: "hal-cpp",
            version,
        },
    )
    .unwrap();

    wpilib_nativeutils::download_artifact_zip_to_dir(
        platform,
        &buildlibs,
        &repos,
        &Artifact {
            artifact_type: ArtifactType::Shared,
            group_id: "edu.wpi.first.hal",
            artifact_id: "hal-cpp",
            version,
        },
    )
    .unwrap();
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

fn test_ur_parsing() {
    let file = include_str!("FRCUsageReporting.h");
    let re = regex::Regex::new(r"\s+k([a-zA-Z0-9]+)_([a-zA-Z0-9_]+) = ([0-9]+),").unwrap();
    let mut enum_ents: BTreeMap<&str, ResourceEnumBuilder> = Default::default();

    for (_, [enum_name, enum_var, value]) in re.captures_iter(file).map(|cap| cap.extract()) {
        if !enum_ents.contains_key(enum_name) {
            enum_ents.insert(enum_name, ResourceEnumBuilder::new(enum_name));
        }
        let ent = enum_ents.get_mut(enum_name).unwrap();

        ent.variants
            .insert(enum_var.to_string(), value.parse::<i32>().unwrap());
    }

    for ent in enum_ents.values() {
        println!("{}", ent.generate_enum());
    }
    //for line in file.split('\n') {
    //    println!("repr: '{}'", line);
    //    if line == "namespace HALUsageReporting {" {
    //        println!("Found start of usage reporting")
    //    }
    //}
}
