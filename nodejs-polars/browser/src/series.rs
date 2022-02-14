use crate::prelude::*;
use polars::prelude::*;
use polars_core::utils::CustomIterTools;
use wasm_bindgen::JsCast;

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

macro_rules! typed_to_series {
    ($series_name:expr, $arr:expr,$pl_type:ty) => {{
        let s = ChunkedArray::<$pl_type>::from_vec(&$series_name, $arr.to_vec()).into_series();
        Ok(JsSeries::from(s))
    }};
}
macro_rules! init_method {
    ($name:ident, $type:ty) => {
        #[wasm_bindgen(js_class=series)]
        impl JsSeries {
            #[wasm_bindgen]
            pub fn $name(params: JsValue) -> JsResult<JsSeries> {
                let params = WrappedObject(params);
                let name = params.get_as::<String>("name")?;
                let values = params.get_as::<Vec<$type>>("values")?;
                Ok(Series::new(&name, values).into())
            }
        }
    };
}

init_method!(new_bool, bool);
init_method!(new_str, String);
init_method!(new_i8, i8);
init_method!(new_i16, i16);
init_method!(new_i32, i32);
init_method!(new_i64, i64);
init_method!(new_u8, u8);
init_method!(new_u16, u16);
init_method!(new_u32, u32);
init_method!(new_u64, u64);
init_method!(new_f32, f32);
init_method!(new_f64, f64);

#[wasm_bindgen(js_class=series)]
impl JsSeries {
    #[wasm_bindgen]
    pub fn new_from_typed_array(params: JsValue) -> JsResult<JsSeries> {
        let params = WrappedObject(params);
        let name = params.get_as::<String>("name")?;
        let values: TypedArrayType = params.get("values")?.into();
        match values {
            TypedArrayType::Int8(v) => typed_to_series!(&name, v, Int8Type),
            TypedArrayType::Uint8(v) => typed_to_series!(&name, v, UInt8Type),
            TypedArrayType::Uint8Clamped(v) => typed_to_series!(&name, v, UInt8Type),
            TypedArrayType::Int16(v) => typed_to_series!(&name, v, Int16Type),
            TypedArrayType::Uint16(v) => typed_to_series!(&name, v, UInt16Type),
            TypedArrayType::Int32(v) => typed_to_series!(&name, v, Int32Type),
            TypedArrayType::Uint32(v) => typed_to_series!(&name, v, UInt32Type),
            TypedArrayType::Float32(v) => typed_to_series!(&name, v, Float32Type),
            TypedArrayType::Float64(v) => typed_to_series!(&name, v, Float64Type),
            TypedArrayType::BigInt64(v) => {
                return Err(JsError::new("BigInt64Array not supported").into())
            }
            TypedArrayType::BigUint64(v) => {
                return Err(JsError::new("BigUint64Array not supported").into())
            }
        }
    }

    #[wasm_bindgen]
    pub fn new_date(params: JsValue) -> JsResult<JsSeries> {
        let params = WrappedObject(params);
        let name = params.get_as::<String>("name")?;
        let values = params.get_as::<Vec<String>>("values")?;
        Ok(Series::new(&name, values).into())
    }

    #[wasm_bindgen]
    pub fn new_opt_bool(params: JsValue) -> JsResult<JsSeries> {
        let params = WrappedObject(params);
        let name = params.get_as::<String>("name")?;
        let strict = params.get_as::<bool>("strict")?;
        let arr: js_sys::Array = params.get("values")?.into();
        let len = arr.length();
        let mut builder = BooleanChunkedBuilder::new(&name, len as usize);
        for i in 0..len {
            let item: JsValue = arr.get(i);

            match item.as_bool() {
                Some(b) => builder.append_value(b),
                None => {
                    if strict {
                        return Err("".into());
                    }
                    builder.append_null()
                }
            }
        }
        let ca: BooleanChunked = builder.finish();
        Ok(ca.into_series().into())
    }

    #[wasm_bindgen]
    pub fn new_opt_str(params: JsValue) -> JsResult<JsSeries> {
        let params = WrappedObject(params);
        let name = params.get_as::<String>("name")?;
        let strict = params.get_as::<bool>("strict")?;
        let arr: js_sys::Array = params.get("values")?.into();
        let len = arr.length();
        let u_len = len as usize;
        let mut builder = Utf8ChunkedBuilder::new("", u_len, u_len * 25);
        for i in 0..len {
            let item: JsValue = arr.get(i);

            match item.as_string() {
                Some(b) => builder.append_value(b),
                None => {
                    if strict {
                        return Err("".into());
                    }
                    builder.append_null()
                }
            }
        }
        let ca: Utf8Chunked = builder.finish();
        Ok(ca.into_series().into())
    }

    #[wasm_bindgen]
    pub fn new_opt_u64(params: JsValue) -> JsResult<JsSeries> {
        let params = WrappedObject(params);
        let name = params.get_as::<String>("name")?;
        let strict = params.get_as::<bool>("strict")?;
        let arr: js_sys::Array = params.get("values")?.into();
        let len = arr.length();
        let u_len = len as usize;
        let mut builder = PrimitiveChunkedBuilder::<UInt64Type>::new(&name, u_len);
        for i in 0..len {
            let item: JsValue = arr.get(i);
            if item.is_bigint() {
                let bigint = item.as_string().unwrap().parse::<u64>().unwrap();

                builder.append_value(bigint)
            } else {
                if strict {
                    return Err("".into());
                }
                builder.append_null()
            }
        }
        let ca: ChunkedArray<UInt64Type> = builder.finish();
        Ok(ca.into_series().into())
    }

    #[wasm_bindgen]
    pub fn new_opt_date(params: JsValue) -> JsResult<JsSeries> {
        let params = WrappedObject(params);
        let name = params.get_as::<String>("name")?;
        let strict = params.get_as::<bool>("strict")?;
        let arr: js_sys::Array = params.get("values")?.into();
        let len = arr.length();
        let mut builder = PrimitiveChunkedBuilder::<Int64Type>::new(&name, len as usize);
        for i in 0..len {
            let item: JsValue = arr.get(i);
            if js_sys::Date::instanceof(&item) {
                let d: js_sys::Date = item.unchecked_into();
                let millis = d.get_milliseconds();

                builder.append_value(millis as i64)
            } else {
                if strict {
                    return Err("".into());
                }
                builder.append_null()
            }
        }
        let ca: ChunkedArray<Int64Type> = builder.finish();
        Ok(ca
            .into_datetime(TimeUnit::Milliseconds, None)
            .into_series()
            .into())
    }

    #[wasm_bindgen]
    pub fn get_fmt(&self) -> String {
        format!("{}", self.series)
    }
    pub fn abs(&self) -> JsResult<JsValue> {
        let s = self.series.abs().map_err(JsPolarsEr::from)?;
        let s = JsSeries::from(s);
        Ok(s.into())
    }
    pub fn chunk_lengths(&self) -> JsValue {
        todo!()
        // js_sys::Float64Array::view(self.series.chunk_lengths().into()).into()
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

    pub fn dtype(&self) -> String {
        let dt: JsDataType = self.series.dtype().into();
        dt.to_string()
    }

    pub fn len(&self) -> JsValue {
        let float_len = self.series.len() as f64;
        float_len.into()
    }

    pub fn cast(&mut self, params: JsValue) -> JsResult<JsSeries> {
        let params = WrappedObject(params);
        let dtype = params.get_as::<DataType>("dtype")?;
        let strict = params.get_as::<bool>("strict")?;
        let out = if strict {
            self.series.strict_cast(&dtype)
        } else {
            self.series.cast(&dtype)
        };
        let s = out.map_err(JsPolarsEr::from)?;
        Ok(s.into())
    }
    pub fn to_js(&self) -> JsValue {
        JsValue::from_serde(&self.series).unwrap()
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
                    match <$native>::from_js(item) {
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

macro_rules! impl_get {
    ($name:ident, $series_variant:ident) => {
        #[wasm_bindgen(js_class=series)]
        impl JsSeries {
            #[wasm_bindgen]
            pub fn $name(&self, params: JsValue) -> JsResult<JsValue> {
                let params = WrappedObject(params);
                let index = params.get_as::<i64>("index")?;
                match self.series.$series_variant() {
                    Ok(ca) => {
                        let index = if index < 0 {
                            (ca.len() as i64 + index) as usize
                        } else {
                            index as usize
                        };
                        match ca.get(index) {
                            Some(v) => Ok(v.into()),
                            None => Ok(JsValue::NULL),
                        }
                    }
                    Err(_) => Ok(JsValue::NULL),
                }
            }
        }
    };
}

impl_get!(get_f32, f32);
impl_get!(get_f64, f64);
impl_get!(get_u8, u8);
impl_get!(get_u16, u16);
impl_get!(get_u32, u32);
impl_get!(get_i8, i8);
impl_get!(get_i16, i16);
impl_get!(get_i32, i32);
// impl_equality!(eq, equal);
