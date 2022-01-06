use crate::prelude::*;
use polars_core::utils::CustomIterTools;

#[wasm_bindgen(js_name=series)]
#[repr(transparent)]
pub struct JsSeries {
    pub(crate) series: Series,
}

impl From<Series> for JsSeries {
    fn from(series: Series) -> Self {
        Self { series }
    }
}

#[wasm_bindgen(js_class=series)]
impl JsSeries {
    #[wasm_bindgen]
    pub fn new_bool(params: JsValue) -> JsResult<JsSeries> {
        let params = WrappedObject(params);
        let name = params.get_as::<String>("name")?;
        let values = params.get_as::<Vec<bool>>("values")?;
        Ok(Series::new(&name, values).into())
    }

    #[wasm_bindgen]
    pub fn new_str(params: JsValue) -> JsResult<JsSeries> {
        let params = WrappedObject(params);
        let name = params.get_as::<String>("name")?;
        let val = params.get_as::<Utf8Chunked>("values")?;
        let mut s = val.into_series();
        s.rename(&name);
        Ok(s.into())
    }
    #[wasm_bindgen]
    pub fn repeat(cx: CallContext) -> JsResult<JsSeries> {
        let params = WrappedObject(params);
        let name = params.get_as::<String>("name")?;
        let val: JsValue = params.get("value")?;
        let dtype = DataType::Null;
        // let dtype: DataType = params.get_as::<DataType>("dtype")?;
        let s: Series = match dtype {
            DataType::Utf8 => {
                let val = String::from_js(val).unwrap();
                let mut ca: Utf8Chunked = (0..n).map(|_| val).collect_trusted();
                ca.rename(&name);
                ca.into_series()
    
            }
            DataType::Float64 => {
                let val = val.as_f64();
                let mut ca: Float64Chunked = (0..n).map(|_| val).collect_trusted();
                ca.rename(&name);
                ca.into_series()
            }
            DataType::Boolean => {
                let val: bool = bool::from_js(val).unwrap();
                let mut ca: BooleanChunked = (0..n).map(|_| val).collect_trusted();
                ca.rename(&name);
                ca.into_series()
            }
            dt => {
                panic!("cannot create repeat with dtype: {:?}", dt);
            }
        };
        Ok(s.into())
        // s.try_into_js(&cx)
    }

    #[wasm_bindgen(js_name = toString)]
    pub fn to_string(&self) -> String {
        format!("{}", self.series)
    }
}
