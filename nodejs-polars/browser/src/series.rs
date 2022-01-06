use crate::prelude::*;
use polars::prelude::*;
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
    pub fn get_fmt(&self) -> String {
        format!("{}", self.series)
    }

    pub fn dtype(&self) -> String {
        let dt: JsDataType = self.series.dtype().into();
        dt.to_string()
    }
    pub fn name(&self) -> String {
        self.series.name().into()
    }
    pub fn rename(&mut self, params: JsValue) -> JsResult<(())> {
        let params = WrappedObject(params);
        let name = params.get_as::<String>("name")?;
        self.series.rename(&name);
        Ok(())
    }
}

macro_rules! init_method_opt {
    ($name:ident, $type:ty, $native:ty) => {
        #[wasm_bindgen(js_class=series)]
        impl JsSeries {
            #[wasm_bindgen]
            pub fn $name(params: JsValue) -> JsResult<JsSeries> {
                let params = WrappedObject(params);
                let name = params.get_as::<String>("name")?;
                let strict = params.get_as::<bool>("strict")?;
                let arr: js_sys::Array = params.get("values")?.into();
                let len = arr.length();
                let mut builder = PrimitiveChunkedBuilder::<$type>::new(&name, len as usize);
                for i in 0..len {
                    let item = arr.get(i);
                    let item: WrappedValue = item.into();
                    match item.extract::<$native>() {
                        Ok(val) => builder.append_value(val),
                        Err(e) => {
                            if strict {
                                return Err(e);
                            }
                            builder.append_null()
                        }
                    }
                }
                let ca: ChunkedArray<$type> = builder.finish();
                Ok(ca.into_series().into())
            }
        }
    };
}
init_method_opt!(new_opt_u16, UInt16Type, u16);
init_method_opt!(new_opt_u32, UInt32Type, u32);
init_method_opt!(new_opt_u64, UInt64Type, u64);
init_method_opt!(new_opt_i8, Int8Type, i8);
init_method_opt!(new_opt_i16, Int16Type, i16);
init_method_opt!(new_opt_i32, Int32Type, i32);
init_method_opt!(new_opt_i64, Int64Type, i64);
init_method_opt!(new_opt_f32, Float32Type, f32);
init_method_opt!(new_opt_f64, Float64Type, f64);

macro_rules! impl_from_chunked {
    ($name:ident) => {
        #[wasm_bindgen(js_class=series)]
        impl JsSeries {
            #[wasm_bindgen]
            pub fn $name(&self, params: JsValue) -> JsResult<JsSeries> {
                let params = WrappedObject(params);
                Ok(self.series.$name().into_series().into())
            }
        }
    };
}

impl_from_chunked!(is_null);
impl_from_chunked!(is_not_null);
impl_from_chunked!(peak_max);
impl_from_chunked!(peak_min);
macro_rules! impl_from_chunked_with_err {
    ($name:ident) => {
        #[wasm_bindgen(js_class=series)]
        impl JsSeries {
            #[wasm_bindgen]
            pub fn $name(&self, params: JsValue) -> JsResult<JsSeries> {
                let params = WrappedObject(params);
                let s = self.series.$name().map_err(JsPolarsEr::from)?.into_series();
                Ok(s.into())
            }
        }
    };
}
impl_from_chunked_with_err!(arg_unique);
impl_from_chunked_with_err!(is_not_nan);
impl_from_chunked_with_err!(is_nan);
impl_from_chunked_with_err!(is_finite);
impl_from_chunked_with_err!(is_infinite);
impl_from_chunked_with_err!(is_unique);
impl_from_chunked_with_err!(arg_true);
impl_from_chunked_with_err!(is_duplicated);
impl_from_chunked_with_err!(year);
impl_from_chunked_with_err!(month);
impl_from_chunked_with_err!(weekday);
impl_from_chunked_with_err!(week);
impl_from_chunked_with_err!(day);
impl_from_chunked_with_err!(ordinal_day);
impl_from_chunked_with_err!(hour);
impl_from_chunked_with_err!(minute);
impl_from_chunked_with_err!(second);
impl_from_chunked_with_err!(nanosecond);
impl_from_chunked_with_err!(is_first);
impl_from_chunked_with_err!(timestamp);
macro_rules! impl_method {
    ($name:ident) => {
        #[wasm_bindgen(js_class=series)]
        impl JsSeries {
            #[wasm_bindgen]
            pub fn $name(&self) -> JsSeries {
                self.series.$name().into()
            }
        }
    };
    ($name:ident, $type:ty, $property:expr ) => {
        #[wasm_bindgen(js_class=series)]
        impl JsSeries {
            #[wasm_bindgen]
            pub fn $name(&self, params: JsValue) -> JsResult<JsSeries> {
                let params = WrappedObject(params);
                let prop = params.get_as::<$type>($property)?;
                Ok(self.series.$name(prop).into())
            }
        }
    };
}
impl_method!(clone);
impl_method!(drop_nulls);
impl_method!(interpolate);

impl_method!(cumsum, bool, "reverse");
impl_method!(cummax, bool, "reverse");
impl_method!(cummin, bool, "reverse");
impl_method!(cumprod, bool, "reverse");
impl_method!(sort, bool, "reverse");
impl_method!(tail, Option<usize>, "length");
impl_method!(head, Option<usize>, "length");
impl_method!(limit, usize, "num_elements");
impl_method!(shift, i64, "periods");
impl_method!(take_every, usize, "n");

macro_rules! impl_method_with_err {
    ($name:ident) => {
        #[wasm_bindgen(js_class=series)]
        impl JsSeries {
            #[wasm_bindgen]
            pub fn $name(&self) -> JsResult<JsSeries> {
                let s = self.series.$name().map_err(JsPolarsEr::from)?;
                Ok(s.into())
            }
        }
    };
    ($name:ident, $type:ty ) => {
        #[wasm_bindgen(js_class=series)]
        impl JsSeries {
            #[wasm_bindgen]
            pub fn $name(&self, params: JsValue) -> JsResult<$type> {
                let params = WrappedObject(params);
                let v = self.series.$name().map_err(JsPolarsEr::from)?;
                Ok(v)
            }
        }
    };
    ($name:ident, $type:ty, $property:expr ) => {
        #[wasm_bindgen(js_class=series)]
        impl JsSeries {
            #[wasm_bindgen]
            pub fn $name(&self, params: JsValue) -> JsResult<JsSeries> {
                let params = WrappedObject(params);
                let prop = params.get_as::<$type>($property)?;
                let s = self.series.$name().map_err(JsPolarsEr::from)?;
                Ok(s.into())
            }
        }
    };
}

impl_method_with_err!(unique);
impl_method_with_err!(explode);
impl_method_with_err!(floor);
impl_method_with_err!(mode);
impl_method_with_err!(n_unique, usize);
// impl_method_with_err!(round, u32, "decimals");

// impl_method_with_err!(strftime, &str, "fmt");

macro_rules! impl_equality {
    ($name:ident, $method:ident) => {
        #[wasm_bindgen(js_class=series)]
        impl JsSeries {
            #[wasm_bindgen]
            pub fn $name(&self, params: JsValue) -> JsResult<JsSeries> {
                let params = WrappedObject(params);
                let rhs: JsSeries = params.get("rhs")?.into();
                let s = self.series.$method(rhs.series).into_series().into();
                Ok(s)
            }
        }
    };
}

impl_equality!(eq, equal);
