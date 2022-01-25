use crate::error::*;

use crate::prelude::*;
use crate::{console_log, log};
use wasm_bindgen::JsCast;
type JsResult<T> = std::result::Result<T, JsValue>;

pub trait FromJsValue: Sized + Send {
  fn from_js(obj: JsValue) -> JsResult<Self>;
}

impl FromJsValue for DataType {
  fn from_js(val: JsValue) -> JsResult<Self> {
    let val = val.as_f64().unwrap();
    Ok(num_to_polarstype(val as u32))
  }
}

// Don't change the order of these!
pub fn num_to_polarstype(n: u32) -> DataType {
  match n {
    0 => DataType::Int8,
    1 => DataType::Int16,
    2 => DataType::Int32,
    3 => DataType::Int64,
    4 => DataType::UInt8,
    5 => DataType::UInt16,
    6 => DataType::UInt32,
    7 => DataType::UInt64,
    8 => DataType::Float32,
    9 => DataType::Float64,
    10 => DataType::Boolean,
    11 => DataType::Utf8,
    12 => DataType::List(DataType::Null.into()),
    13 => DataType::Date,
    14 => DataType::Datetime(TimeUnit::Milliseconds, None),
    15 => DataType::Time,
    // 16 => DataType::Object("object"),
    17 => DataType::Categorical,
    tp => panic!("Type {} not implemented in num_to_polarstype", tp),
  }
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
    Ok(val.as_string().unwrap())
  }
}

impl FromJsValue for bool {
  fn from_js(val: JsValue) -> JsResult<Self> {
    Ok(val.as_bool().unwrap())
  }
}

macro_rules! numbers {
    ($($n:ident)*) => ($(
        impl FromJsValue for $n {
            fn from_js(val: JsValue) -> JsResult<Self>  {
              let val = val.as_f64().unwrap();
              Ok(val as $n)
            }
        }
    )*)
}

// todo support bigint
numbers! { i8 u8 i16 u16 i32 u32 f32 f64 i64 u64 usize }

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

pub struct WrappedObject(pub(crate) JsValue);

impl From<JsValue> for WrappedObject {
  fn from(h: JsValue) -> Self {
    Self(h)
  }
}

impl WrappedObject {
  pub fn get_as<V: FromJsValue>(&self, key: &str) -> JsResult<V> {
    let key = JsValue::from_str(key);
    let val = js_sys::Reflect::get(&self.0, &key)?;
    V::from_js(val)
  }
  pub fn get(&self, key: &str) -> JsResult<JsValue> {
    let key = JsValue::from_str(key);
    js_sys::Reflect::get(&self.0, &key)
  }
}

pub struct WrappedValue(pub(crate) JsValue);

impl From<JsValue> for WrappedValue {
  fn from(jsv: JsValue) -> Self {
    Self(jsv)
  }
}

impl WrappedValue {
  pub fn extract<V: FromJsValue>(self) -> JsResult<V> {
    V::from_js(self.0)
  }
}


