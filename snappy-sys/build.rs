extern crate gcc;

fn main() {
	let mut snappy_config = gcc::Config::new();
	snappy_config.include("snappy/");
	snappy_config.include(".");

	snappy_config.define("NDEBUG", Some("1"));

	if !cfg!(target_env = "msvc") {
		snappy_config.flag("-std=c++11");
	} else {
		snappy_config.flag("-EHsc");
	}

	snappy_config.file("snappy/snappy.cc");
	snappy_config.file("snappy/snappy-sinksource.cc");
	snappy_config.file("snappy/snappy-c.cc");
	snappy_config.cpp(true);
	snappy_config.compile("libsnappy.a");
}

