use std::env;

fn main() {
  let _target_dir = env::var_os("OUT_DIR").unwrap();
  let home_dir = env::var("HOME").unwrap();
  #[cfg(target_os = "macos")]
  {
    println!("cargo:rustc-link-arg=-Wl,-rpath,{home_dir}/Applications/OBS-test.app/Contents/Frameworks");
  }
  tauri_build::build()
}
