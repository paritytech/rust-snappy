extern crate cmake;

use std::env;
use std::fs;

use cmake::Config;

fn main() {
	let src = env::current_dir().unwrap().join("snappy");

	let out = Config::new("snappy")
		.define("CMAKE_VERBOSE_MAKEFILE", "ON")
		.build_target("snappy")
		.build();

	let mut build = out.join("build");

	if cfg!(target_os = "windows") {
		let stub = build.join("snappy-stubs-public.h");

		let profile = match &*env::var("PROFILE").unwrap_or("debug".to_owned()) {
			"bench" | "release" => "Release",
			_ => "Debug",
		};
		build = build.join(profile);

		fs::copy(stub, build.join("snappy-stubs-public.h")).unwrap();
	}

	fs::copy(src.join("snappy.h"), build.join("snappy.h")).unwrap();

	println!("cargo:rustc-link-search=native={}", build.display());
	println!("cargo:rustc-link-lib=static=snappy");
	println!("cargo:include={}", build.display());

    // NOTE: the cfg! macro doesn't work when cross-compiling, it would return values for the host
    let target_os = env::var("CARGO_CFG_TARGET_OS").expect("CARGO_CFG_TARGET_OS is set by cargo.");
    let target_android = target_os.contains("android");

	// https://github.com/alexcrichton/cc-rs/blob/ca70fd32c10f8cea805700e944f3a8d1f97d96d4/src/lib.rs#L891
	if cfg!(any(target_os = "macos", target_os = "freebsd", target_os = "openbsd")) {
		println!("cargo:rustc-link-lib=c++");
	} else if cfg!(not(target_env = "msvc")) && !target_android {
		println!("cargo:rustc-link-lib=stdc++");
	}
}
