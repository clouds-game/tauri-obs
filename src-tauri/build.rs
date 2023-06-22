use std::env;

fn main() {
  let _target_dir = env::var_os("OUT_DIR").unwrap();
  #[cfg(target_os = "macos")]
  {
    println!("cargo:rustc-link-arg=-Wl,-rpath,/Applications/OBS.app/Contents/Frameworks");
  }
  tauri_build::build()
}
