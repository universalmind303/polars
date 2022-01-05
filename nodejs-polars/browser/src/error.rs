use polars_core::prelude::PolarsError;
use thiserror::Error;
use wasm_bindgen::prelude::JsValue;

#[derive(Debug, Error)]
pub enum JsPolarsEr {
    #[error(transparent)]
    Any(#[from] PolarsError),
    #[error("{0}")]
    Other(String),
}

impl std::convert::From<JsPolarsEr> for JsValue {
    fn from(err: JsPolarsEr) -> JsValue {
        let reason = format!("{}", err);
        js_sys::Error::new(&reason).into()
    }
}
