use crate::error::*;
use wasm_bindgen::prelude::*;
type Result<T> = std::result::Result<T, JsValue>;
use crate::{console_log, log};
pub trait FromJsValue: Sized + Send {
  fn from_js(obj: JsValue) -> Result<Self>;
}

impl<V> FromJsValue for Vec<V>
where
  V: FromJsValue + std::fmt::Debug,
{
  fn from_js(val: JsValue) -> Result<Self> {
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
  fn from_js(val: JsValue) -> Result<Self> {
    let v = V::from_js(val);
    match v {
      Ok(v) => Ok(Some(v)),
      Err(_) => Ok(None),
    }
  }
}
impl FromJsValue for String {
  fn from_js(val: JsValue) -> Result<Self> {
    let e: JsValue = JsPolarsEr::Other("invalid cast".into()).into();
    val.as_string().ok_or(e)
  }
}

impl FromJsValue for bool {
  fn from_js(val: JsValue) -> Result<Self> {
    let e: JsValue = JsPolarsEr::Other("invalid cast".into()).into();
    val.as_bool().ok_or(e)
  }
}
