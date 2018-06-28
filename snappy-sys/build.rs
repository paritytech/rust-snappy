extern crate cmake;

use std::env;

use cmake::Config;

fn main() {
	let out = Config::new("snappy")
		.define("CMAKE_VERBOSE_MAKEFILE", "ON")
		.build_target("snappy")
		.build();

	let mut build = out.join("build");

	if cfg!(target_os = "windows") {
		let profile = match &*env::var("PROFILE").unwrap_or("debug".to_owned()) {
			"bench" | "release" => "Release",
			_ => "Debug",
		};
		build = build.join(profile);
	}

	println!("cargo:rustc-link-search=native={}", build.display());
	println!("cargo:rustc-link-lib=static=snappy");

	// https://github.com/alexcrichton/cc-rs/blob/ca70fd32c10f8cea805700e944f3a8d1f97d96d4/src/lib.rs#L891
	if cfg!(any(target_os = "macos", target_os = "freebsd", target_os = "openbsd")) {
		println!("cargo:rustc-link-lib=c++");
	} else if cfg!(not(target_env = "msvc")) {
		println!("cargo:rustc-link-lib=stdc++");
	}
}
