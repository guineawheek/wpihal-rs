use std::{fmt::Display, io::Cursor, path::{Path, PathBuf}};


#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ArtifactType {
    //Pom,
    Jar,
    JarDebug,
    JarSources,
    Javadoc,
    Headers,
    Sources,
    Shared,
    SharedDebug,
    Static,
    StaticDebug,
}

impl ArtifactType {
    pub fn suffix(&self, platform: &str) -> String {
        match self {
            //ArtifactType::Pom => ".pom".to_string(),
            ArtifactType::Jar => ".jar".to_string(),
            ArtifactType::JarDebug => "debug.jar".to_string(),
            ArtifactType::JarSources => "sources.jar".to_string(),
            ArtifactType::Javadoc => "javadoc.jar".to_string(),
            ArtifactType::Headers => "headers.zip".to_string(),
            ArtifactType::Sources => "sources.zip".to_string(),
            ArtifactType::Shared => format!("{platform}.zip"),
            ArtifactType::SharedDebug => format!("{platform}debug.zip"),
            ArtifactType::Static => format!("{platform}static.zip"),
            ArtifactType::StaticDebug => format!("{platform}staticdebug.zip")
        }
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Artifact<'a> {
    pub artifact_type: ArtifactType,
    pub group_id: &'a str,
    pub artifact_id: &'a str,
    pub version: &'a str,
}

impl<'a> Artifact<'a> {
    pub fn construct_uri(&self, base: &str, platform: Platform) -> String {
        format!(
            "{base}/{group_id}/{artifact_id}/{version}/{artifact_id}-{version}-{suffix}", 
            group_id = self.group_id.replace(".", "/"),
            artifact_id = self.artifact_id,
            version = self.version,
            suffix = self.artifact_type.suffix(platform.platform_string())
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NativeUtilsError {
    UnsupportedURI,
    InvalidPlatform,
}

impl Display for NativeUtilsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("error ig")
    }
}
impl std::error::Error for NativeUtilsError {}

pub struct MavenRepo(pub String);

impl MavenRepo {
    pub fn fetch_artifact(&self, artifact: &Artifact, platform: Platform) -> anyhow::Result<Vec<u8>> {
        if self.0.starts_with("https://") {
            Ok(self.fetch_artifact_https(artifact, platform)?)
        } else if self.0.starts_with("file:") {
            Ok(self.fetch_artifact_fs(artifact, platform)?)
        } else {
            Err(NativeUtilsError::UnsupportedURI.into())
        }
    }

    fn fetch_artifact_https(&self, artifact: &Artifact, platform: Platform) -> anyhow::Result<Vec<u8>> {
        let uri = artifact.construct_uri(self.0.as_str(), platform);
        println!("uri: {}", uri);
        Ok(reqwest::blocking::get(uri)?.error_for_status()?.bytes()?.to_vec())
    }

    fn fetch_artifact_fs(&self, artifact: &Artifact, platform: Platform) -> anyhow::Result<Vec<u8>> {
        let uri = artifact.construct_uri(&self.0.as_str()[5..], platform);
        println!("uri: {}", uri);
        Ok(std::fs::read(uri)?)
    }

}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Platform {
    LinuxAthena, // roborio
    LinuxArm32, // old and stinky coprocs
    LinuxArm64, // new coprocs
    LinuxX86_64, // Intel
    OsxUniversal, // macs
    WindowsX86_64, // WIntel
    WindowsArm64, // Warm
}

impl Platform {
    pub fn platform_string(&self) -> &'static str {
        match self {
            Platform::LinuxAthena => "linuxathena",
            Platform::LinuxArm32 => "linuxarm32",
            Platform::LinuxArm64 => "linuxarm64",
            Platform::LinuxX86_64 => "linuxx86-64",
            Platform::OsxUniversal => "osxuniversal",
            Platform::WindowsX86_64 => "windowsx86-64",
            Platform::WindowsArm64 => "windowsarm64"
        }
    }

    pub fn operating_system(&self) -> &'static str {
        match self {
            Platform::LinuxAthena => "linux",
            Platform::LinuxArm32 => "linux",
            Platform::LinuxArm64 => "linux",
            Platform::LinuxX86_64 => "linux",
            Platform::OsxUniversal => "osx",
            Platform::WindowsX86_64 => "windows",
            Platform::WindowsArm64 => "windows"
        }
    }

    pub fn architecture(&self) -> &'static str {
        match self {
            Platform::LinuxAthena => "athena",
            Platform::LinuxArm32 => "arm32",
            Platform::LinuxArm64 => "arm64",
            Platform::LinuxX86_64 => "x86-64",
            Platform::OsxUniversal => "universal",
            Platform::WindowsX86_64 => "x86-64",
            Platform::WindowsArm64 => "arm64"
        }
    }

    pub fn from_rust_target(rust_target: &str) -> Option<Self> {
        match rust_target {
            "arm-unknown-linux-gnueabi" => Some(Self::LinuxAthena), // the roborio
            "arm-unknown-linux-gnueabihf" => Some(Self::LinuxArm32), // old and stinky coprocessors
            "aarch64-unknown-linux-gnu" => Some(Self::LinuxArm64), // useful coprocessors
            "x86_64-unknown-linux-gnu" => Some(Self::LinuxX86_64), // the linux desktop. or a beelink
            "x86_64-apple-darwin" => Some(Self::OsxUniversal), // intel macs
            "aarch64-apple-darwin" => Some(Self::OsxUniversal), // actually good macs
            "x86_64-pc-windows-msvc" => Some(Self::WindowsX86_64), // the average programmer laptop
            "aarch64-pc-windows-msvc" => Some(Self::WindowsArm64), // the platform Nobody Uses
            _ => None // sorry we don't support risc-v
        }
    }
}



#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ReleaseTrain {
    Development,
    Release,
}

pub fn get_local_maven(release_train: ReleaseTrain) -> MavenRepo {
    let user_home = home::home_dir().unwrap_or_default().to_string_lossy().to_string();
    match release_train {
        ReleaseTrain::Development => MavenRepo(format!("file:{user_home}/releases/maven/development")),
        ReleaseTrain::Release => MavenRepo(format!("file:{user_home}/releases/maven/release")),
    }
}

pub fn get_remote_maven(release_train: ReleaseTrain) -> MavenRepo {
    const REMOTE_BASE: &str = "https://frcmaven.wpi.edu/artifactory";
    match release_train {
        ReleaseTrain::Development => MavenRepo(format!("{REMOTE_BASE}/development")),
        ReleaseTrain::Release => MavenRepo(format!("{REMOTE_BASE}/release")),
    }
}

pub fn get_wpilib_root(year: &str) -> PathBuf {
    #[cfg(target_os = "windows")]
    {
        let public_folder = std::env::var_os("PUBLIC").unwrap_or(std::ffi::OsString::from("C:\\Users\\Public"));
        Path::new(&public_folder).join("wpilib").join(year)
    }
    #[cfg(not(target_os = "windows"))]
    {
        let containing_dir = home::home_dir().unwrap_or_default();
        containing_dir.join("wpilib").join(year)
    }

}

pub fn get_wpilib_maven(year: &str) -> MavenRepo {
    let wpilib_maven_root = get_wpilib_root(year).join("maven");
    #[cfg(target_os = "windows")]
    let wpilib_root_string = wpilib_maven_root.to_string_lossy().replace("\\", "/");
    #[cfg(not(target_os = "windows"))]
    let wpilib_root_string = wpilib_maven_root.to_string_lossy().to_string();

    MavenRepo(format!("file:/{wpilib_root_string}"))
}

/*
Folder paths
buildlibs
|
[artifact-version]
|-[headers]
|-[libs]

Files to extract:
.pdb (windows debug)
.dll (windows sahred)
.lib (windows static)
.so (linux shared)
.a (unix static)
.dylib (osx)

*/


pub fn download_artifact_zip_to_dir(platform: Platform, dir: &Path, repos: &[MavenRepo], artifact: &Artifact) -> anyhow::Result<()> {
    //let Some(platform) = Platform::from_rust_target(target) else { return Err(NativeUtilsError::InvalidPlatform.into()) };
    let dir = PathBuf::from(dir);
    let mut last_err: Option<anyhow::Error> = None;
    let mut artifact_data: Option<Vec<u8>> = None;
    for repo in repos {
        match repo.fetch_artifact(artifact, platform) {
            Ok(a) => {
                artifact_data = Some(a);
                break;
            }
            Err(e) => {
                last_err = Some(e);
            }
        }
    }
    let Some(artifact_data) = artifact_data else { return Err(last_err.expect("no maven repos specified!!!")); };
    let mut zipfile = zip::ZipArchive::new(Cursor::new(artifact_data))?;
    zipfile.extract(dir)?;

    Ok(())
}

pub fn download_native_library_artifacts(
    repos: &[MavenRepo],
    platform: Platform,
    group_id: &str,
    artifact_id: &str,
    version: &str,
    buildlibs: &Path, 
) -> anyhow::Result<()> {
    let cache_marker = buildlibs.join(format!(".nativeutils_downloaded_{group_id}.{artifact_id}-{version}"));
    if cache_marker.exists() { return Ok(()); }

    let headers_dir = buildlibs.join("headers");
    std::fs::create_dir_all(&headers_dir)?;

    download_artifact_zip_to_dir(platform, &headers_dir, repos, &Artifact {
        artifact_type: ArtifactType::Headers,
        group_id,
        artifact_id,
        version,
    }).unwrap();

    for (artifact_type, build_type) in [
        (ArtifactType::Shared, "release"),
        (ArtifactType::SharedDebug, "debug"),
        (ArtifactType::Static, "release"),
        (ArtifactType::StaticDebug, "debug"),
    ] {
        let output_dir = buildlibs.join(build_type);
        std::fs::create_dir_all(&output_dir)?;
        download_artifact_zip_to_dir(platform, &output_dir, repos, &Artifact {
            artifact_type,
            group_id,
            artifact_id,
            version,
        })?;
    }
    std::fs::OpenOptions::new().create(true).write(true).open(cache_marker).ok();
    Ok(())
}

pub fn lib_search_path(dir: &Path, platform: Platform, shared: bool, debug: bool) -> PathBuf {
    let mut path = PathBuf::from(dir);
    path.push(if debug { "debug" } else { "release" });

    path.push(platform.operating_system());
    path.push(platform.architecture());
    if shared {
        path.push("shared");
    } else {
        path.push("static");
    }
    path
}

pub fn rustc_link_search(dir: &Path, platform: Platform, shared: bool, debug: bool) {
    println!("cargo:rustc-link-search={}", stringify_path(&lib_search_path(dir, platform, shared, debug)))
}

pub fn header_search_path(dir: &Path) -> PathBuf {
    dir.join("headers")
}

pub fn rustc_debug_switch(libs: &[&str], debug: bool) {
    for lib in libs {
        if debug {
            println!("cargo:rustc-link-lib={lib}d");
        } else {
            println!("cargo:rustc-link-lib={lib}");
        }
    }
}


pub fn stringify_path(path: &Path) -> String {
    const PLEASE_USE_UTF8: &str = "your file system paths are not utf8 please build this in a utf8 pathed directory";
    let canon = path.canonicalize().unwrap();
    let s = canon.to_str().expect(PLEASE_USE_UTF8);
    #[cfg(windows)]
    {
        // on windows, canonicalize() adds the _absolute_ absolute path with a \\?\ prefix, but this breaks downstream tooling.
        // downside, of course, is the 255 character limit.
        s.strip_prefix(r"\\?\").unwrap_or(s).to_string()
    }
    #[cfg(unix)]
    {
        s.to_string()
    }
}

pub fn add_sysroot_to_clang_args(clang_args: &mut Vec<String>, platform: Platform, year: &str) -> anyhow::Result<()> {
    if let Some(sysroot) = locate_sysroot(platform, year) {
        eprintln!("Located sysroot at {:?}", sysroot.path());
        eprintln!("Located sysroot c++ at {:?}", sysroot.cpp_include());
        clang_args.push(format!("--sysroot={}", stringify_path(sysroot.path())));
        clang_args.push(format!("-I{}", stringify_path(&sysroot.cpp_include().expect("can't find c++ headers in the sysroot"))));
        if let Some(bits_headers) = sysroot.cpp_bits_include() {
            // only the rio target has a separate bits header path for some reason
            clang_args.push(format!("-I{}", stringify_path(&bits_headers)));
        }
    }
    Ok(())
}


pub struct Sysroot {
    path: PathBuf,
    target: String,
}
impl Sysroot {
    pub fn new(path: &Path, target: &str) -> Self {
        Self {
            path: path.into(),
            target: target.into()
        }
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    pub fn cpp_include(&self) -> Option<PathBuf> {
        let cpp_base = self.path.join("usr").join("include").join("c++");
        latest_gcc_version(&cpp_base)
    }

    pub fn cpp_bits_include(&self) -> Option<PathBuf> {
        let path = self.cpp_include()?.join(&self.target);
        if path.exists() {
            Some(path)
        } else { None }
    }

}

/// Locates ths sysroot and relevant directories to be included in order for C++ bindgen to work
pub fn locate_sysroot(platform: Platform, year: &str) -> Option<Sysroot> {
    // Locates the sysroot.
    /*
    Sysroots are located at:
      roborio:
        /usr/local/arm-nilrt-linux-gnueabi/sysroot
        ~/wpilib/{YEAR}/roborio/arm-nilrt-linux-gnueabi/sysroot
      aarch64:
        /usr/local/aarch64-linux-gnu/sysroot
      armhf:
        /usr/local/arm-linux-gnueabihf/sysroot
      
      Everything else shouldn't need one because it's a native build.
     */
    match platform {
        Platform::LinuxAthena => {
            // first check the local location first and then try everything else
            const ATHENA_SYSROOT: &str = "/usr/local/arm-nilrt-linux-gnueabi/sysroot";
            if Path::new(ATHENA_SYSROOT).exists() {
                Some(Sysroot::new(Path::new(ATHENA_SYSROOT), "arm-nilrt-linux-gnueabi"))
            } else {
                let user_sysroot = get_wpilib_root(year).join("roborio").join("arm-nilrt-linux-gnueabi").join("sysroot");
                if user_sysroot.exists() {
                    Some(Sysroot::new(&user_sysroot, "arm-nilrt-linux-gnueabi"))
                } else { None }
            }
        }
        Platform::LinuxArm32 => {
            const ARM32_SYSROOT: &str = "/usr/local/arm-linux-gnueabihf/sysroot";
            if Path::new(ARM32_SYSROOT).exists() {
                Some(Sysroot::new(Path::new(ARM32_SYSROOT), "arm-linux-gnueabihf"))
            } else { None }
        }
        Platform::LinuxArm64 => {
            const ARM64_SYSROOT: &str = "/usr/local/aarch64-linux-gnu/sysroot";
            if Path::new(ARM64_SYSROOT).exists() {
                Some(Sysroot::new(Path::new(ARM64_SYSROOT), "aarch64-linux-gnu"))
            } else { None }
        }
        _ => None
    }
}

fn latest_gcc_version(p: &Path) -> Option<PathBuf> {
    Some(p.read_dir().ok()?.max_by(|a, b| {
        match (a, b) {
            (Ok(v1), Ok(v2)) => {
                let Ok(v1_str) = v1.file_name().into_string() else { return std::cmp::Ordering::Less };
                let Ok(v1_num) = v1_str.parse::<i64>() else { return std::cmp::Ordering::Less };
                let Ok(v2_str) = v2.file_name().into_string() else { return std::cmp::Ordering::Greater };
                let Ok(v2_num) = v2_str.parse::<i64>() else { return std::cmp::Ordering::Greater };
                v1_num.cmp(&v2_num)
            }
            (Ok(_), Err(_)) => std::cmp::Ordering::Greater,
            (Err(_), Ok(_)) => std::cmp::Ordering::Less,
            (Err(_), Err(_)) => std::cmp::Ordering::Equal,
        }
    })?.ok()?.path())
}

