#![feature(path)]
#![feature(path_ext)]
extern crate "pkg-config" as pkg_config;

use std::path::Path;
use std::fs::PathExt;

fn find_static_lib(lib_name: &str) -> bool {
  match pkg_config::Config::new().statik(true).find(lib_name) {
    Ok(_) => true,
    Err(_) => false
  }
}

fn main () {
  if find_static_lib("sodium") {
    return;
  }
  let path = Path::new(".").join("..").join("third_party_libs").join("libsodium.a");
  if path.exists() {
    println!("cargo:rustc-flags=-l sodium -L third_party_libs/");
  } else {
    panic!("Sodium library could not be found.\n\
    Linux or OSX: Install sodium from https://github.com/jedisct1/libsodium \n\
    Windows:\n\tDownload the library from, https://download.libsodium.org/libsodium/releases/libsodium-1.0.1-mingw.tar.gz\n\
    \tExtract and add the libsodium.a file to the third_party_libs folder in the project root directory");
  }
}
