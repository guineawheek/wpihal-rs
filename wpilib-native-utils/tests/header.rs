use std::path::PathBuf;

use wpilib_native_utils::{Artifact, ArtifactType, Platform, ReleaseTrain, WPILibVersion};

#[test]
#[ignore]
fn download_header() {
    let platform = Platform::LinuxSystemCore;
    let version = WPILibVersion::new("2027.0.0-alpha-6");
    let local_maven = wpilib_native_utils::get_local_maven(ReleaseTrain::Development);
    let wpilib_maven = version.get_wpilib_maven();
    let remote_maven = version.get_remote_maven(ReleaseTrain::Development);
    let repos = [local_maven, wpilib_maven, remote_maven];

    let target_path = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());
    let buildlibs = target_path.join("../target/test/buildlibs");
    std::fs::create_dir_all(&buildlibs).unwrap();

    wpilib_native_utils::download_artifact_zip_to_dir(
        platform,
        &buildlibs,
        &repos,
        &Artifact {
            artifact_type: ArtifactType::Headers,
            group_id: "org.wpilib.hal",
            artifact_id: "hal-cpp",
            version: &version.to_string(),
        },
    )
    .unwrap();
}
