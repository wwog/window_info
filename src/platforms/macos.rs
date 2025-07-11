use objc2_core_foundation::CFRetained;
use objc2_core_graphics::{CGWindowListCopyWindowInfo, CGWindowListOption, kCGNullWindowID};
use plist::{Value, from_reader};
use serde::Deserialize;
use std::ffi::{CStr, c_void};
use std::io::Cursor;

unsafe extern "C" {
  fn CFArrayGetCount(array: *const c_void) -> isize;
  fn CFArrayGetValueAtIndex(array: *const c_void, index: isize) -> *const c_void;
  fn CFShow(obj: *const c_void);
  fn CFCopyDescription(obj: *const c_void) -> *const c_void;
  fn CFRelease(obj: *const c_void);
  fn CFStringGetCStringPtr(string: *const c_void, encoding: u32) -> *const std::os::raw::c_char;
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct WindowBounds {
  #[serde(rename = "Height")]
  height: i32,
  #[serde(rename = "Width")]
  width: i32,
  #[serde(rename = "X", deserialize_with = "de_x")]
  x: i32,
  #[serde(rename = "Y", deserialize_with = "de_y")]
  y: i32,
}
#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct CGWindow {
  #[serde(rename = "kCGWindowAlpha")]
  alpha: f32,
  #[serde(rename = "kCGWindowBounds")]
  bounds: WindowBounds,
  #[serde(rename = "kCGWindowIsOnscreen")]
  is_onscreen: i32,
  #[serde(rename = "kCGWindowLayer")]
  layer: i32,
  #[serde(rename = "kCGWindowMemoryUsage")]
  memory_usage: i32,
  #[serde(rename = "kCGWindowName")]
  name: String,
  #[serde(rename = "kCGWindowNumber")]
  number: i32,
  #[serde(rename = "kCGWindowOwnerName")]
  owner_name: String,
  #[serde(rename = "kCGWindowOwnerPID")]
  owner_pid: i32,
  #[serde(rename = "kCGWindowSharingState")]
  sharing_state: i32,
  #[serde(rename = "kCGWindowStoreType")]
  store_type: i32,
  #[serde(skip)]
  index: usize,
}

// 兼容 Y 字段为字符串或数字，始终转为 i32
fn de_y<'de, D>(deserializer: D) -> Result<i32, D::Error>
where
  D: serde::Deserializer<'de>,
{
  struct YVisitor;
  impl<'de> serde::de::Visitor<'de> for YVisitor {
    type Value = i32;
    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
      formatter.write_str("string or integer")
    }
    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
      E: serde::de::Error,
    {
      v.parse::<i32>().map_err(E::custom)
    }
    fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
    where
      E: serde::de::Error,
    {
      Ok(v as i32)
    }
    fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
    where
      E: serde::de::Error,
    {
      Ok(v as i32)
    }
  }
  deserializer.deserialize_any(YVisitor)
}

// 兼容 X 字段为字符串或数字，始终转为 i32
fn de_x<'de, D>(deserializer: D) -> Result<i32, D::Error>
where
  D: serde::Deserializer<'de>,
{
  struct XVisitor;
  impl<'de> serde::de::Visitor<'de> for XVisitor {
    type Value = i32;
    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
      formatter.write_str("string or integer")
    }
    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
      E: serde::de::Error,
    {
      v.parse::<i32>().map_err(E::custom)
    }
    fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
    where
      E: serde::de::Error,
    {
      Ok(v as i32)
    }
    fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
    where
      E: serde::de::Error,
    {
      Ok(v as i32)
    }
  }
  deserializer.deserialize_any(XVisitor)
}

unsafe fn cf_string_to_rust_string(cf_string: *const c_void) -> String {
  if cf_string.is_null() {
    return String::from("");
  }

  let c_string = unsafe { CFStringGetCStringPtr(cf_string, 0x08000100) }; // kCFStringEncodingUTF8
  if !c_string.is_null() {
    let rust_string = unsafe { CStr::from_ptr(c_string).to_string_lossy().into_owned() };
    // 处理Unicode转义序列
    return rust_string;
  }

  // 如果无法直接获取C字符串，返回默认值
  String::from("")
}

pub fn list_windows() -> Vec<String> {
  unsafe {
    /*
    选项                                     包含范围             是否需要窗口 ID 是否限于屏幕上   典型场景
    kCGWindowListOptionAll                   所有窗口（屏幕内外）      否             否      全局窗口信息收集
    kCGWindowListOptionOnScreenOnly          仅屏幕上窗口             否             是      截屏或可见窗口分析
    kCGWindowListOptionOnScreenAboveWindow   指定窗口上方的屏幕上窗口   是             是      窗口层级分析（上层）
    kCGWindowListOptionOnScreenBelowWindow   指定窗口下方的屏幕上窗口   是             是      窗口层级分析（下层）
    kCGWindowListOptionIncludingWindow       包含指定窗口             是          视其他选项    结合层级选项使用
    kCGWindowListExcludeDesktopElements      排除桌面元素             否          视其他选项    过滤桌面窗口
     */
    // 获取当前屏幕上所有窗口信息
    let windows = CGWindowListCopyWindowInfo(
      CGWindowListOption::OptionOnScreenOnly | CGWindowListOption::ExcludeDesktopElements,
      kCGNullWindowID,
    )
    .unwrap();
    // 将返回的窗口信息转换为CFArrayRef,使用CFRetained将其转换为指针
    let array_ptr = CFRetained::as_ptr(&windows).as_ptr() as *const c_void;
    // 通过CFArrayGetCount获取窗口数量
    let count = CFArrayGetCount(array_ptr);
    println!("当前屏幕上窗口数量: {}", count);
    let mut cg_windows: Vec<CGWindow> = Vec::new();
    let mut windows_strs: Vec<String> = Vec::new();
    for i in 0..count {
      // 使用CFArrayGetValueAtIndex获取指定索引的窗口信息
      // 这里的array_ptr是CFArrayRef类型，指向一个包含窗口信息
      // 的数组，每个元素都是一个字典（CFDictionaryRef）
      let window_dict = CFArrayGetValueAtIndex(array_ptr, i) as *const c_void;
      if window_dict.is_null() {
        println!("无法获取窗口{}信息", i);
        CFShow(window_dict);
        continue;
      }
      let description = CFCopyDescription(window_dict);
      if description.is_null() {
        continue;
      }
      let rust_string = cf_string_to_rust_string(description);
      CFRelease(description);
      if rust_string.is_empty() {
        println!("无法获取窗口{}描述", i);
        continue;
      }
      // 打印窗口描述
      let cursor = Cursor::new(&rust_string);
      let plist_value: Value = from_reader(cursor).unwrap();
      let mut cg_window: CGWindow = plist::from_value(&plist_value).unwrap();
      cg_window.index = i as usize;
      cg_windows.push(cg_window);
      windows_strs.push(rust_string.clone());
    }
    CFRelease(array_ptr);
    windows_strs
  }
}
