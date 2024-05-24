// rustc --print cfg
// debug_assertions
// panic="unwind"
// target_abi=""
// target_arch="x86_64"
// target_endian="little"
// target_env="msvc"
// target_family="windows"
// target_feature="cmpxchg16b"
// target_feature="fxsr"
// target_feature="sse"
// target_feature="sse2"
// target_feature="sse3"
// target_has_atomic="128"
// target_has_atomic="16"
// target_has_atomic="32"
// target_has_atomic="64"
// target_has_atomic="8"
// target_has_atomic="ptr"
// target_os="windows"
// target_pointer_width="64"
// target_vendor="pc"
// windows
// Summary

//     cfg! and #[cfg] use predefined configuration options set by the Rust compiler based on the target triple.
//     You can use these options to conditionally compile code for different platforms, architectures, and features.
//     The rustc --print cfg command lists all available configuration options for the current compilation target.
//     Simplify conditions by using configuration options directly (e.g., cfg!(windows) instead of cfg!(target_os = "windows")).
fn main() {
    let helpful_message = if cfg!(target_os = "windows") {
        "backslash"
    } else {
        "slash"
    };

    println!("...then type the directory name followed by a {helpful_message}. Then you...");

    os_specific_function();

    let os_message = if cfg!(target_os = "windows") {
        "Running on Windows"
    } else if cfg!(target_os = "macos") {
        "Running on macOS"
    } else if cfg!(target_os = "linux") {
        "Running on Linux"
    } else {
        "Running on an unknown OS"
    };

    println!("{}", os_message);

    let arch_message = if cfg!(target_arch = "x86_64") {
        "64-bit architecture"
    } else if cfg!(target_arch = "x86") {
        "32-bit architecture"
    } else {
        "Unknown architecture"
    };

    println!("{}", arch_message);
}

#[cfg(target_os = "windows")]
fn os_specific_function() {
    println!("This is Windows!");
}

#[cfg(not(target_os = "windows"))]
fn os_specific_function() {
    println!("This is not Windows!");
}
