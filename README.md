# wpihal-rs

Safe-enough™ WPILib HAL bindings in Rust

## Installation

This is not a particularly stable package, no warranty if it blows up in your face, makes you lose matches, or makes your code not compile mid-competition.

As such, you can add `wpihal` as a git dependency to your project toml:

```toml
[dependencies]
wpihal = { git = "https://github.com/guineawheek/wpihal-rs.git", branch = "2027", features = ["robot-controller"] }
```

I recommend pinning to a specific commit or branch.
**The API may change at a moment's notice,** especially since this is tracking the SystemCore alpha/beta testing releases.

To accelerate build times, you should also install the version of WPILib corresponding to this package's version; in this case **v2027.0.0-alpha-6**;
as the build scripts first search your computer for the WPILib Maven artifacts in `~/wpilib` (or `%PUBLIC%\wpilib` on bad operating systems) before trying to download them externally.

## Compiling for coprocessors

Both arm64 coprocessors and SystemCore are, well, 64-bit ARM.
Set the `robot-controller` feature flag if you want to compile for SystemCore.

Otherwise, this library will be linked against the arm64 coprocessor version of WPILib.

## Why just the HAL?

Past attempts at writing a WPILib port for Rust have proven to be...a bit much to reliably maintain.

The HAL generally doesn't change that much and almost entirely an easy-to-wrap C API.

Besides the obvious "run your robot off of Rust", HAL bindings also have additional applications such as:
* HALSim plugins
* Vendor drivers
* Other pieces of native code run in FRC contexts that would benefit from not having to compile under 3 different C++ compilers of varying versions

This library does _not_ wrap higher-level layers directly; if you want that you'll need to do that yourself.

## Overall goals

This crate in general aims for correct, safe behavior.

Actual achievements of these aims is not certain and there are almost certainly subtle soundness holes, but given this is a wrapper crate for a WPILib component and not a Rust RFC those holes may have a chance of being fixed this century if found.

* RAII wrappers for all the session handles that automatically close handles on `Drop`
 * Also implement `Drop` for things that typically require a manual free
* `HALResult<T>` wrappers over status fields
* Vague sense of maintaining aliasing xor mutability
 * Lifetime abuse to ensure that super-peripheral handles don't get `Drop`ed before their child peripherals do (e.g. DIOs don't get dropped before LED handles)
* Slightly better user experience than just using raw bindgen vomit
* Deals with linking against WPILib for you on all relevant platforms
* Terrible build times (already achieved)
 * to be fair, it's for similar reasons as gradle as it's bottlenecked on unzipping artifacts

## Things that could use some work

* Better guards against WPILib's overuse of `i32`s for things that are NOT i32 sized
* Stability guarentees
* ~~NTCore support~~ now out of scope.
* Versioning this in both the semver sense and the wpilib sense they are inconsistent with each other
* Be more specific about usage of `&mut self` vs `&self` on wrapper types depending on if the underlying HAL impl is thread-safe (which implies `Send`/`Sync`)
* `WPI_EventHandle` support (and `HAL_ProvideNewDataEventHandle` and friends)
  * I want to plumb this in a way that tokio could use it if one wanted to
* Opmode support
  * ID generation not implemented
* Finish the halsim simdevice hooks (no motivation)
* Dealing with wpilib install locations moving
* Proc-macros for defining HAL enum impls and `HALSIM_InitExtension` 

## HALSIM plugins

if you make a cdylib that exports the following:

```rust
#[link_name = "HALSIM_InitExtension"]
unsafe extern "C" fn init_extension() -> i32 {
  // this is your extension's entry point
  0
}
```
then this should work out, and then you can do your `HAL_RegisterExtension/HAL_RegisterExtensionListener/HAL_SetMain` or whatever