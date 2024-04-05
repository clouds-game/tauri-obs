use obs_wrapper::obs_sys::obs_data_t;

use super::settings;

#[allow(non_camel_case_types)]
pub enum SourceSettings {
  Win_GameCapture(settings::win_game_capture::Setting),
}

impl SourceSettings {
  pub fn build(self) -> obs_data_t {
    todo!()
  }
}
