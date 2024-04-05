#[allow(non_camel_case_types)]
pub type int = i64;

pub mod win_game_capture {
  pub const ID: &str = "game-capture";
  /// TODO enum
  pub const TYPE: &str = "OBS_SOURCE_TYPE_INPUT";

  /// ```c
  /// #define SETTING_MODE_ANY         "any_fullscreen"
  /// #define SETTING_MODE_WINDOW      "window"
  /// #define SETTING_MODE_HOTKEY      "hotkey"
  /// ```
  #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize)]
  pub enum CaptureMode {
    /// ```c
    /// #define SETTING_MODE_ANY         "any_fullscreen"
    /// ```
    #[default]
    #[serde(rename = "any_fullscreen")]
    Any,
    /// #define SETTING_MODE_WINDOW      "window"
    #[serde(rename = "window")]
    Window,
    /// #define SETTING_MODE_HOTKEY      "hotkey"
    #[serde(rename = "hotkey")]
    Hotkey,
  }

  /// ```c
  /// enum window_priority {
  ///   WINDOW_PRIORITY_CLASS,
  ///   WINDOW_PRIORITY_TITLE,
  ///   WINDOW_PRIORITY_EXE,
  /// };
  /// ```
  #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, serde_repr::Serialize_repr, serde_repr::Deserialize_repr)]
  #[repr(u8)]
  pub enum WindowPriority {
    /// window_priority.WINDOW_PRIORITY_CLASS
    Class = 0,
    /// window_priority.WINDOW_PRIORITY_TITLE
    Title,
    /// window_priority.WINDOW_PRIORITY_EXE
    #[default]
    Exe,
  }

  /// ```c
  /// enum hook_rate {
  ///   HOOK_RATE_SLOW,
  ///   HOOK_RATE_NORMAL,
  ///   HOOK_RATE_FAST,
  ///   HOOK_RATE_FASTEST
  /// };
  /// ```
  #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, serde_repr::Serialize_repr, serde_repr::Deserialize_repr)]
  #[repr(u8)]
  pub enum HookRate {
    /// hook_rate.HOOK_RATE_SLOW
    Slow = 0,
    /// hook_rate.HOOK_RATE_NORMAL
    #[default]
    Normal,
    /// hook_rate.HOOK_RATE_FAST
    Fast,
    /// hook_rate.HOOK_RATE_FASTEST
    Fastest,
  }

  /// ```c
  /// #define RGBA10A2_SPACE_SRGB "srgb"
  /// #define RGBA10A2_SPACE_2100PQ "2100pq"
  /// ```
  #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize)]
  pub enum Rgba10a2Space {
    /// #define RGBA10A2_SPACE_SRGB "srgb"
    #[default]
    #[serde(rename = "srgb")]
    Srgb,
    /// #define RGBA10A2_SPACE_2100PQ "2100pq"
    #[serde(rename = "2100pq")]
    _2100PQ,
  }


  #[derive(Debug, derivative::Derivative, serde::Serialize, serde::Deserialize, PartialEq)]
  #[derivative(Default)]
  pub struct Setting {
    /// ```c
    /// #define SETTING_MODE                 "capture_mode"
    /// obs_data_set_default_string(settings, SETTING_MODE, SETTING_MODE_ANY);
    /// ```
    pub capture_mode: CaptureMode,
    /// ```c
    /// #define SETTING_CAPTURE_WINDOW       "window"
    /// obs_data_get_string(settings, SETTING_CAPTURE_WINDOW);
    /// ```
    pub window: Option<String>,
    /// ```c
    /// #define SETTING_WINDOW_PRIORITY      "priority"
    /// obs_data_set_default_int(settings, SETTING_WINDOW_PRIORITY, (int)WINDOW_PRIORITY_EXE);
    /// ```
    pub priority: WindowPriority,
    /// ```c
    /// #define SETTING_COMPATIBILITY        "sli_compatibility"
    /// obs_data_set_default_bool(settings, SETTING_COMPATIBILITY, false);
    /// ```
    #[derivative(Default(value="false"))]
    pub sli_compatibility: bool,
    /// ```c
    /// #define SETTING_CURSOR               "capture_cursor"
    /// obs_data_set_default_bool(settings, SETTING_CURSOR, true);
    /// ```
    #[derivative(Default(value="true"))]
    pub capture_cursor: bool,
    /// ```c
    /// #define SETTING_TRANSPARENCY         "allow_transparency"
    /// obs_data_set_default_bool(settings, SETTING_TRANSPARENCY, false);
    /// ```
    pub allow_transparency: bool,
    /// ```c
    /// #define SETTING_PREMULTIPLIED_ALPHA  "premultiplied_alpha"
    /// obs_data_set_default_bool(settings, SETTING_PREMULTIPLIED_ALPHA, false);
    /// ```
    pub premultiplied_alpha: bool,
    /// ```c
    /// #define SETTING_LIMIT_FRAMERATE      "limit_framerate"
    /// obs_data_set_default_bool(settings, SETTING_LIMIT_FRAMERATE, false);
    /// ```
    pub limit_framerate: bool,
    /// ```c
    /// #define SETTING_CAPTURE_OVERLAYS     "capture_overlays"
    /// obs_data_set_default_bool(settings, SETTING_CAPTURE_OVERLAYS, false);
    /// ```
    pub capture_overlays: bool,
    /// ```c
    /// #define SETTING_ANTI_CHEAT_HOOK      "anti_cheat_hook"
    /// obs_data_set_default_bool(settings, SETTING_ANTI_CHEAT_HOOK, true);
    /// ```
    #[derivative(Default(value="true"))]
    pub anti_cheat_hook: bool,
    /// ```c
    /// #define SETTING_HOOK_RATE            "hook_rate"
    /// obs_data_set_default_int(settings, SETTING_HOOK_RATE, (int)HOOK_RATE_NORMAL);
    /// ```
    pub hook_rate: HookRate,
    /// ```c
    /// #define SETTING_RGBA10A2_SPACE       "rgb10a2_space"
    /// obs_data_set_default_string(settings, SETTING_RGBA10A2_SPACE, RGBA10A2_SPACE_SRGB);
    /// ```
    pub rgb10a2_space: Rgba10a2Space,
    /// ```c
    /// #define SETTINGS_COMPAT_INFO         "compat_info"
    /// ```
    /// TODO: might be property
    pub compat_info: Option<String>,
  }

  #[test]
  fn test_serde() {
    let setting = Setting::default();
    let json = serde_json::to_string(&setting).unwrap();
    println!("{json}");
    assert_eq!(json, r#"{"capture_mode":"any_fullscreen","window":null,"priority":2,"sli_compatibility":false,"capture_cursor":true,"allow_transparency":false,"premultiplied_alpha":false,"limit_framerate":false,"capture_overlays":false,"anti_cheat_hook":true,"hook_rate":1,"rgb10a2_space":"srgb","compat_info":null}"#);
    let setting2 = serde_json::from_str(&json).unwrap();
    assert_eq!(setting, setting2);
  }
}

/// Shown as "Display Capture" (macOS)
/// plugins/mac-capture/mac-display-capture.m
/// ```c
/// static void display_capture_defaults(obs_data_t *settings)
/// void window_defaults(obs_data_t *settings)
/// static void *sck_video_capture_create(obs_data_t *settings, obs_source_t *source)
/// ```
#[allow(deprecated)]
pub mod mac_display_capture {
  use uuid::Uuid;

  pub const ID: &str = "display_capture";
  /// TODO enum
  pub const TYPE: &str = "OBS_SOURCE_TYPE_INPUT";

  /// ```c
  /// enum crop_mode {
  ///   CROP_NONE,
  ///   CROP_MANUAL,
  ///   CROP_TO_WINDOW,
  ///   CROP_TO_WINDOW_AND_MANUAL,
  ///   CROP_INVALID
  /// };
  /// ```
  #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, serde_repr::Serialize_repr, serde_repr::Deserialize_repr)]
  #[repr(u8)]
  pub enum CropMode {
    /// crop_mode.CROP_NONE
    #[default]
    None = 0,
    /// crop_mode.CROP_MANUAL
    Manual,
    /// crop_mode.CROP_TO_WINDOW
    ToWindow,
    /// crop_mode.CROP_TO_WINDOW_AND_MANUAL
    ToWindowAndManual,
    /// crop_mode.CROP_INVALID
    Invalid,
  }

  #[derive(Debug, derivative::Derivative, serde::Serialize, serde::Deserialize, PartialEq)]
  #[derivative(Default)]
  #[deprecated(note = "use mac_screen_capture instead")]
  pub struct Setting {
    /// ```objc
    /// NSNumber *screen = [[NSScreen mainScreen] deviceDescription][@"NSScreenNumber"];
    /// CFUUIDRef display_uuid = CGDisplayCreateUUIDFromDisplayID((CGDirectDisplayID) screen.intValue);
    /// CFStringRef uuid_string = CFUUIDCreateString(kCFAllocatorDefault, display_uuid);
    /// obs_data_set_default_string(settings, "display_uuid", CFStringGetCStringPtr(uuid_string, kCFStringEncodingUTF8));
    /// ```
    #[derivative(Default(value="Self::default_display_uuid()"))]
    pub display_uuid: Uuid,
    /// ```c
    /// obs_data_set_default_bool(settings, "show_cursor", true);
    /// ```
    #[derivative(Default(value="true"))]
    pub show_cursor: bool,
    /// ```c
    /// obs_data_set_default_int(settings, "crop_mode", CROP_NONE);
    /// ```
    pub crop_mode: CropMode,
    /// ```c
    /// obs_data_set_default_int(settings, "window", kCGNullWindowID);
    /// ```
    /// https://developer.apple.com/documentation/coregraphics/kcgnullwindowid
    pub window: u32,
    /// ```c
    /// obs_data_set_default_bool(settings, "show_empty_names", false);
    /// ```
    pub show_empty_names: bool,
  }

  impl Setting {
    #[cfg(target_os = "macos")]
    pub fn default_display_uuid() -> Uuid {
      // TODO: get display_uuid
      Uuid::default()
    }

    #[cfg(not(target_os = "macos"))]
    pub fn default_display_uuid() -> Uuid {
      Uuid::default()
    }
  }

  #[test]
  fn test_serde() {
    let setting = Setting::default();
    let json = serde_json::to_string(&setting).unwrap();
    println!("{json}");
    assert_eq!(json, r#"{"display_uuid":"00000000-0000-0000-0000-000000000000","show_cursor":true,"crop_mode":0,"window":0,"show_empty_names":false}"#);
    let setting2 = serde_json::from_str(&json).unwrap();
    assert_eq!(setting, setting2);
  }
}

/// Shown as "Window Capture" (macOS)
/// plugins/mac-capture/mac-window-capture.m
/// ```c
/// static void window_capture_defaults(obs_data_t *settings)
/// void window_defaults(obs_data_t *settings)
/// ```
#[allow(deprecated)]
pub mod mac_window_capture {
  pub const ID: &str = "window_capture";
  /// TODO enum
  pub const TYPE: &str = "OBS_SOURCE_TYPE_INPUT";

  #[derive(Debug, derivative::Derivative, serde::Serialize, serde::Deserialize, PartialEq)]
  #[derivative(Default)]
  #[deprecated(note = "use mac_screen_capture instead")]
  pub struct Setting {
    /// ```c
    /// obs_data_set_default_bool(settings, "show_shadow", false);
    /// ```
    pub show_shadow: bool,
    /// ```c
    /// obs_data_set_default_int(settings, "window", kCGNullWindowID);
    /// ```
    /// https://developer.apple.com/documentation/coregraphics/kcgnullwindowid
    pub window: u32,
    /// ```c
    /// obs_data_set_default_bool(settings, "show_empty_names", false);
    /// ```
    pub show_empty_names: bool,
  }

  #[test]
  fn test_serde() {
    let setting = Setting::default();
    let json = serde_json::to_string(&setting).unwrap();
    println!("{json}");
    assert_eq!(json, r#"{"show_shadow":false,"window":0,"show_empty_names":false}"#);
    let setting2 = serde_json::from_str(&json).unwrap();
    assert_eq!(setting, setting2);
  }
}
