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

	println!("cargo:root={}", build.display());
	println!("cargo:rustc-link-search=native={}", build.display());
	println!("cargo:rustc-link-lib=static=snappy");

	// https://github.com/alexcrichton/cc-rs/blob/ca70fd32c10f8cea805700e944f3a8d1f97d96d4/src/lib.rs#L891
	if cfg!(any(target_os = "macos", target_os = "freebsd", target_os = "openbsd")) {
		println!("cargo:rustc-link-lib=c++");
	} else if cfg!(not(target_env = "msvc")) {
		println!("cargo:rustc-link-lib=stdc++");
	}
}
