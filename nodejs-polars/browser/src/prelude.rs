pub use polars_core::prelude::*;
pub use crate::conversion::prelude::*;
pub use crate::error::JsPolarsEr;
pub use wasm_bindgen::prelude::*;

pub type JsResult<T> = std::result::Result<T, JsValue>;
