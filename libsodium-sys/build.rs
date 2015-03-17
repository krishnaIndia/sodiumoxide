extern crate "pkg-config" as pkg_config;

use std::env;

fn find_static_lib(lib_name: &str) -> bool {
	match pkg_config::Config::new().statik(true).find(lib_name) {
			Ok(_) => true,
			Err(_) => false
		}
}

fn main () {
  if !find_static_lib("sodium") {
    println!("cargo:rustc-flags=-l sodium -L lib/");
  }
}
