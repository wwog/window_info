#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

mod platforms;

#[napi]
pub fn list_windows() -> Vec<String> {
  platforms::list_windows()
}
