#![deny(clippy::all)]

use napi::bindgen_prelude::Buffer;

#[macro_use]
extern crate napi_derive;

#[napi]
pub fn extract_text(buf: Buffer) -> String {
  match pdf_extract::extract_text_from_mem(&buf) {
    Ok(text) => text,
    Err(_) => "".to_string(),
  }
}
