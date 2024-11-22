use std::path::PathBuf;

use wpilib_nativeutils::{Artifact, ArtifactType, ReleaseTrain};

fn main() {
    let version = env!("CARGO_PKG_VERSION");
    let year = env!("CARGO_PKG_VERSION_MAJOR");
    let target = "aarch64-apple-darwin";
    let local_maven = wpilib_nativeutils::get_local_maven(ReleaseTrain::Release);
    let wpilib_maven = wpilib_nativeutils::get_wpilib_maven(year);
    let remote_maven = wpilib_nativeutils::get_remote_maven(ReleaseTrain::Release);
    let repos = [local_maven, wpilib_maven, remote_maven];

    //let out_path = PathBuf::from(std::env::var("OUT_DIR").unwrap());
    let out_path = PathBuf::from("target");
    let buildlibs = out_path.join("buildlibs");
    std::fs::create_dir_all(&buildlibs).unwrap();

    wpilib_nativeutils::download_artifact_zip_to_dir(target, &buildlibs, &repos, &Artifact {
        artifact_type: ArtifactType::Headers,
        group_id: "edu.wpi.first.hal",
        artifact_id: "hal-cpp",
        version,
    }).unwrap();

    wpilib_nativeutils::download_artifact_zip_to_dir(target, &buildlibs, &repos, &Artifact {
        artifact_type: ArtifactType::Shared,
        group_id: "edu.wpi.first.hal",
        artifact_id: "hal-cpp",
        version,
    }).unwrap();
}
