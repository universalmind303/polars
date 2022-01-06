use crate::error::*;

use crate::{console_log, log};
use crate::conversion::wrap::Wrap;
use crate::prelude::*;
type JsResult<T> = std::result::Result<T, JsValue>;

pub trait FromJsValue: Sized + Send {
  fn from_js(obj: JsValue) -> Result<Self>;
}

impl<V> FromJsValue for Vec<V>
where
  V: FromJsValue + std::fmt::Debug,
{
  fn from_js(val: JsValue) -> JsResult<Self> {
    if js_sys::Array::is_array(&val) {
      let obj: js_sys::Array = val.into();

      let len = obj.length();

      let mut arr: Self = Vec::with_capacity(len as usize);
      for i in 0..len {
        let item = obj.get(i);
        
        let item: V = V::from_js(item)?;
        arr.push(item);
      }
      Ok(arr)
    } else {
      Err(JsPolarsEr::Other("invalid cast".into()).into())
    }
  }
}

impl<V> FromJsValue for Option<V>
where
  V: FromJsValue,
{
  fn from_js(val: JsValue) -> JsResult<Self> {
    let v = V::from_js(val);
    match v {
      Ok(v) => Ok(Some(v)),
      Err(_) => Ok(None),
    }
  }
}
impl FromJsValue for String {
  fn from_js(val: JsValue) -> JsResult<Self> {
    let e: JsValue = JsPolarsEr::Other("invalid cast".into()).into();
    val.as_string().ok_or(e)
  }
}

impl FromJsValue for bool {
  fn from_js(val: JsValue) -> JsResult<Self> {
    let e: JsValue = JsPolarsEr::Other("invalid cast".into()).into();
    val.as_bool().ok_or(e)
  }
}

impl FromJsValue for Utf8Chunked {
  fn from_js(val: JsValue) -> JsResult<Self> {
      if js_sys::Array::is_array(&val) {
          let obj: js_sys::Array = val.into();
          let len = obj.length();
          let u_len = len as usize;
          let mut builder = Utf8ChunkedBuilder::new("", u_len, u_len * 25);
          for i in 0..len {
              let item = obj.get(i);
              match String::from_js(item) {
                  Ok(val) => builder.append_value(val),
                  Err(_) => builder.append_null(),
              }
          }
          Ok(builder.finish())
      } else {
          Err(JsPolarsEr::Other("incorrect value type".to_owned()).into())
      }
  }
}