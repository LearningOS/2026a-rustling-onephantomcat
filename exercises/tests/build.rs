//! This is the build script for both tests7 and tests8.
//!
//! You should modify this file to make both exercises pass.

fn main() {
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();

    println!("cargo:rustc-env=TEST_FOO={}", timestamp);
    // 声明 pass 是允许使用的配置值
    println!("cargo:rustc-check-cfg=cfg(feature, values(\"pass\"))");

    // 启用这个编译条件
    println!("cargo:rustc-cfg=feature=\"pass\"");
}