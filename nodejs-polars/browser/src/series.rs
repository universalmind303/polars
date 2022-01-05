use polars_core::prelude::*;
use wasm_bindgen::prelude::*;
use crate::conversion::*;

#[wasm_bindgen]
#[repr(transparent)]
pub struct JsSeries {
    pub(crate) series: Series,
}

impl From<Series> for JsSeries {
    fn from(series: Series) -> Self {
        Self { series }
    }
}

type JsResult<T> = std::result::Result<T, JsValue>;

#[wasm_bindgen]
impl JsSeries {
    #[wasm_bindgen]
    pub fn new_bool(params: JsValue) -> JsResult<JsSeries> {
        // todo!()
        let n = JsValue::from_str("name");
        let v = JsValue::from_str("values");

        let name = js_sys::Reflect::get(&params, &n)?;
        let name = String::from_js(name)?;
        let values = js_sys::Reflect::get(&params, &v)?;
        let values = Vec::<Option<bool>>::from_js(values)?;
    
        Ok(Series::new(&name, values).into())
    }

    // #[wasm_bindgen(js_name = toString)]
    pub fn to_string(&self) -> String {
        format!("{}", self.series)
    }
}
