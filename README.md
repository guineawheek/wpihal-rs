# wpihal-rs

Safe-enough™️ WPILib HAL bindings in Rust

## Installation

This is not a particularly stable package, no warranty if it blows up in your face, makes you lose matches, or makes your code not compile mid-competition.

As such, you can add `wpihal` as a git dependency to your project toml:

```toml
[dependencies]
wpihal = { git = "https://github.com/guineawheek/wpihal.git" }
```

## Why just the HAL?

Past attempts at writing a WPILib port for Rust have proven to be...a bit much to reliably maintain.

The HAL generally doesn't change that much and almost entirely an easy-to-wrap C API.

Besides the obvious "run your robot off of Rust", HAL bindings also have additional applications such as:
* HALSim plugins
* Vendor drivers
* Other pieces of native code run in FRC contexts that would benefit from not having to compile under 3 different C++ compilers of varying versions

This library does _not_ wrap ChipObject/HMB or FRCNetComm directly; if you want that you'll need to do that yourself.


## Overall goals

This crate in general aims for correct, safe behavior.

Actual achievements of these aims is not certain and there are almost certainly subtle soundness holes, but given this is a wrapper crate for a WPILib component and not a Rust RFC those holes may have a chance of being fixed this century if found.

* RAII wrappers for all the session handles that automatically close handles on `Drop`
 * Also implement `Drop` for things that typically require a manual free
* `HALResult<T>` wrappers over status fields
* Vague sense of maintaining aliasing xor mutability
 * Lifetime abuse to ensure that super-peripheral handles don't get `Drop`ed before their child peripherals do (e.g. DIOs don't get dropped before LED handles)
* Slightly better user experience than just using raw bindgen vomit
* Terrible build times (already achieved)

## Things that could use some work
* Testing on real hardware
* Better guards against WPILib's overuse of `i32`s for things that are NOT i32 sized
* macro-ized halsim wrapers