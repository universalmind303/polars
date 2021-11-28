use crate::prelude::JsResult;
use crate::series::*;
use napi::JsObject;

impl JsSeries {
  pub fn to_object(env: &napi::Env) -> JsResult<JsObject> {
    let mut series = env.create_object()?;
    let mut str_obj = env.create_object()?;
    let mut date_obj = env.create_object()?;

    date_obj.define_properties(&[
      napi::Property::new(env, "day")?.with_method(day),
      napi::Property::new(env, "hour")?.with_method(hour),
      napi::Property::new(env, "minute")?.with_method(minute),
      napi::Property::new(env, "month")?.with_method(month),
      napi::Property::new(env, "nanosecond")?.with_method(nanosecond),
      napi::Property::new(env, "ordinal_day")?.with_method(ordinal_day),
      napi::Property::new(env, "second")?.with_method(second),
      napi::Property::new(env, "strftime")?.with_method(strftime),
      napi::Property::new(env, "timestamp")?.with_method(timestamp),
      napi::Property::new(env, "week")?.with_method(week),
      napi::Property::new(env, "weekday")?.with_method(weekday),
      napi::Property::new(env, "year")?.with_method(year),
    ])?;

    str_obj.define_properties(&[
      napi::Property::new(env, "lengths")?.with_method(str_lengths),
      napi::Property::new(env, "contains")?.with_method(str_contains),
      napi::Property::new(env, "json_path_match")?.with_method(str_json_path_match),
      napi::Property::new(env, "extract")?.with_method(str_extract),
      napi::Property::new(env, "replace")?.with_method(str_replace),
      napi::Property::new(env, "to_uppercase")?.with_method(str_to_uppercase),
      napi::Property::new(env, "parse_date")?.with_method(str_parse_date),
      napi::Property::new(env, "parse_datetime")?.with_method(str_parse_datetime),
      napi::Property::new(env, "slice")?.with_method(str_slice),
    ])?;

    series.define_properties(&[
      napi::Property::new(env, "str")?.with_value(str_obj),
      napi::Property::new(env, "date")?.with_value(date_obj),
      napi::Property::new(env, "new_from_typed_array")?.with_method(new_from_typed_array),
      napi::Property::new(env, "dtype")?.with_method(dtype),
      napi::Property::new(env, "to_js")?.with_method(to_js),
      napi::Property::new(env, "new_bool")?.with_method(new_bool),
      napi::Property::new(env, "new_i8")?.with_method(new_i8),
      napi::Property::new(env, "new_i16")?.with_method(new_i16),
      napi::Property::new(env, "new_i32")?.with_method(new_i32),
      napi::Property::new(env, "new_i64")?.with_method(new_i64),
      napi::Property::new(env, "new_u8")?.with_method(new_u8),
      napi::Property::new(env, "new_u16")?.with_method(new_u16),
      napi::Property::new(env, "new_u32")?.with_method(new_u32),
      napi::Property::new(env, "new_u64")?.with_method(new_u64),
      napi::Property::new(env, "new_f32")?.with_method(new_f32),
      napi::Property::new(env, "new_f64")?.with_method(new_f64),
      napi::Property::new(env, "new_opt_u16")?.with_method(new_opt_u16),
      napi::Property::new(env, "new_opt_u32")?.with_method(new_opt_u32),
      napi::Property::new(env, "new_opt_u64")?.with_method(new_opt_u64),
      napi::Property::new(env, "new_opt_i8")?.with_method(new_opt_i8),
      napi::Property::new(env, "new_opt_i16")?.with_method(new_opt_i16),
      napi::Property::new(env, "new_opt_i32")?.with_method(new_opt_i32),
      napi::Property::new(env, "new_opt_i64")?.with_method(new_opt_i64),
      napi::Property::new(env, "new_opt_f32")?.with_method(new_opt_f32),
      napi::Property::new(env, "new_opt_f64")?.with_method(new_opt_f64),
      napi::Property::new(env, "new_opt_date")?.with_method(new_opt_date),
      napi::Property::new(env, "new_opt_bool")?.with_method(new_opt_bool),
      napi::Property::new(env, "new_str")?.with_method(new_str),
      napi::Property::new(env, "new_list")?.with_method(new_list),
      napi::Property::new(env, "new_object")?.with_method(new_object),
      napi::Property::new(env, "repeat")?.with_method(repeat),
      napi::Property::new(env, "get_fmt")?.with_method(get_fmt),
      napi::Property::new(env, "chunk_lengths")?.with_method(chunk_lengths),
      napi::Property::new(env, "name")?.with_method(name),
      napi::Property::new(env, "rename")?.with_method(rename),
      napi::Property::new(env, "tail")?.with_method(tail),
      napi::Property::new(env, "head")?.with_method(head),
      napi::Property::new(env, "get_idx")?.with_method(get_idx),
      napi::Property::new(env, "bitand")?.with_method(bitand),
      napi::Property::new(env, "bitor")?.with_method(bitor),
      napi::Property::new(env, "bitxor")?.with_method(bitxor),
      napi::Property::new(env, "n_chunks")?.with_method(n_chunks),
      napi::Property::new(env, "limit")?.with_method(limit),
      napi::Property::new(env, "slice")?.with_method(slice),
      napi::Property::new(env, "append")?.with_method(append),
      napi::Property::new(env, "filter")?.with_method(filter),
      napi::Property::new(env, "cumsum")?.with_method(cumsum),
      napi::Property::new(env, "cummax")?.with_method(cummax),
      napi::Property::new(env, "cummin")?.with_method(cummin),
      napi::Property::new(env, "cumprod")?.with_method(cumprod),
      napi::Property::new(env, "add")?.with_method(add),
      napi::Property::new(env, "sub")?.with_method(sub),
      napi::Property::new(env, "mul")?.with_method(mul),
      napi::Property::new(env, "div")?.with_method(div),
      napi::Property::new(env, "rem")?.with_method(rem),
      napi::Property::new(env, "mean")?.with_method(crate::series::mean),
      napi::Property::new(env, "max")?.with_method(crate::series::max),
      napi::Property::new(env, "min")?.with_method(crate::series::min),
      napi::Property::new(env, "sum")?.with_method(crate::series::sum),
      napi::Property::new(env, "sort")?.with_method(sort),
      napi::Property::new(env, "sort_in_place")?.with_method(sort_in_place),
      napi::Property::new(env, "argsort")?.with_method(argsort),
      napi::Property::new(env, "unique")?.with_method(unique),
      napi::Property::new(env, "value_counts")?.with_method(value_counts),
      napi::Property::new(env, "arg_unique")?.with_method(arg_unique),
      napi::Property::new(env, "arg_min")?.with_method(arg_min),
      napi::Property::new(env, "arg_max")?.with_method(arg_max),
      napi::Property::new(env, "take")?.with_method(take),
      napi::Property::new(env, "take_with_series")?.with_method(take_with_series),
      napi::Property::new(env, "null_count")?.with_method(null_count),
      napi::Property::new(env, "has_validity")?.with_method(has_validity),
      napi::Property::new(env, "is_null")?.with_method(crate::series::is_null),
      napi::Property::new(env, "is_not_null")?.with_method(crate::series::is_not_null),
      napi::Property::new(env, "is_not_nan")?.with_method(is_not_nan),
      napi::Property::new(env, "is_nan")?.with_method(is_nan),
      napi::Property::new(env, "is_finite")?.with_method(is_finite),
      napi::Property::new(env, "is_infinite")?.with_method(is_infinite),
      napi::Property::new(env, "is_unique")?.with_method(is_unique),
      napi::Property::new(env, "arg_true")?.with_method(arg_true),
      napi::Property::new(env, "is_duplicated")?.with_method(is_duplicated),
      napi::Property::new(env, "explode")?.with_method(explode),
      napi::Property::new(env, "sample_n")?.with_method(sample_n),
      napi::Property::new(env, "sample_frac")?.with_method(sample_frac),
      napi::Property::new(env, "take_every")?.with_method(take_every),
      napi::Property::new(env, "series_equal")?.with_method(series_equal),
      napi::Property::new(env, "eq")?.with_method(eq),
      napi::Property::new(env, "neq")?.with_method(neq),
      napi::Property::new(env, "gt")?.with_method(gt),
      napi::Property::new(env, "gt_eq")?.with_method(gt_eq),
      napi::Property::new(env, "lt")?.with_method(lt),
      napi::Property::new(env, "lt_eq")?.with_method(lt_eq),
      napi::Property::new(env, "_not")?.with_method(not),
      napi::Property::new(env, "as_str")?.with_method(as_str),
      napi::Property::new(env, "len")?.with_method(len),
      napi::Property::new(env, "median")?.with_method(crate::series::median),
      napi::Property::new(env, "quantile")?.with_method(crate::series::quantile),
      napi::Property::new(env, "drop_nulls")?.with_method(drop_nulls),
      napi::Property::new(env, "fill_null")?.with_method(fill_null),
      napi::Property::new(env, "clone")?.with_method(clone),
      napi::Property::new(env, "map")?.with_method(map),
      napi::Property::new(env, "shift")?.with_method(shift),
      napi::Property::new(env, "zip_with")?.with_method(zip_with),
      napi::Property::new(env, "strftime")?.with_method(strftime),
      napi::Property::new(env, "timestamp")?.with_method(timestamp),
      napi::Property::new(env, "to_dummies")?.with_method(to_dummies),
      napi::Property::new(env, "get_list")?.with_method(get_list),
      napi::Property::new(env, "arr_lengths")?.with_method(arr_lengths),
      napi::Property::new(env, "rolling_sum")?.with_method(rolling_sum),
      napi::Property::new(env, "rolling_mean")?.with_method(rolling_mean),
      napi::Property::new(env, "rolling_max")?.with_method(rolling_max),
      napi::Property::new(env, "rolling_min")?.with_method(rolling_min),
      napi::Property::new(env, "rolling_var")?.with_method(rolling_var),
      napi::Property::new(env, "year")?.with_method(year),
      napi::Property::new(env, "month")?.with_method(month),
      napi::Property::new(env, "weekday")?.with_method(weekday),
      napi::Property::new(env, "week")?.with_method(week),
      napi::Property::new(env, "day")?.with_method(day),
      napi::Property::new(env, "ordinal_day")?.with_method(ordinal_day),
      napi::Property::new(env, "hour")?.with_method(hour),
      napi::Property::new(env, "minute")?.with_method(minute),
      napi::Property::new(env, "second")?.with_method(second),
      napi::Property::new(env, "nanosecond")?.with_method(nanosecond),
      napi::Property::new(env, "peak_max")?.with_method(peak_max),
      napi::Property::new(env, "peak_min")?.with_method(peak_min),
      napi::Property::new(env, "n_unique")?.with_method(n_unique),
      napi::Property::new(env, "is_first")?.with_method(is_first),
      napi::Property::new(env, "round")?.with_method(round),
      napi::Property::new(env, "shrink_to_fit")?.with_method(crate::series::shrink_to_fit),
      napi::Property::new(env, "dot")?.with_method(dot),
      napi::Property::new(env, "hash")?.with_method(hash),
      napi::Property::new(env, "reinterpret")?.with_method(reinterpret),
      napi::Property::new(env, "mode")?.with_method(mode),
      napi::Property::new(env, "interpolate")?.with_method(interpolate),
      napi::Property::new(env, "rank")?.with_method(rank),
      napi::Property::new(env, "diff")?.with_method(diff),
      napi::Property::new(env, "skew")?.with_method(skew),
      napi::Property::new(env, "kurtosis")?.with_method(kurtosis),
      napi::Property::new(env, "rechunk")?.with_method(rechunk),
      napi::Property::new(env, "cast")?.with_method(crate::series::cast),
      napi::Property::new(env, "set_with_mask_str")?.with_method(set_with_mask_str),
      napi::Property::new(env, "set_with_mask_f64")?.with_method(set_with_mask_f64),
      napi::Property::new(env, "set_with_mask_f32")?.with_method(set_with_mask_f32),
      napi::Property::new(env, "set_with_mask_u8")?.with_method(set_with_mask_u8),
      napi::Property::new(env, "set_with_mask_u16")?.with_method(set_with_mask_u16),
      napi::Property::new(env, "set_with_mask_u32")?.with_method(set_with_mask_u32),
      napi::Property::new(env, "set_with_mask_u64")?.with_method(set_with_mask_u64),
      napi::Property::new(env, "set_with_mask_i8")?.with_method(set_with_mask_i8),
      napi::Property::new(env, "set_with_mask_i16")?.with_method(set_with_mask_i16),
      napi::Property::new(env, "set_with_mask_i32")?.with_method(set_with_mask_i32),
      napi::Property::new(env, "set_with_mask_i64")?.with_method(set_with_mask_i64),
      napi::Property::new(env, "set_with_mask_bool")?.with_method(set_with_mask_bool),
      napi::Property::new(env, "set_at_idx_str")?.with_method(set_at_idx_str),
      napi::Property::new(env, "set_at_idx_f64")?.with_method(set_at_idx_f64),
      napi::Property::new(env, "set_at_idx_f32")?.with_method(set_at_idx_f32),
      napi::Property::new(env, "set_at_idx_u8")?.with_method(set_at_idx_u8),
      napi::Property::new(env, "set_at_idx_u16")?.with_method(set_at_idx_u16),
      napi::Property::new(env, "set_at_idx_u32")?.with_method(set_at_idx_u32),
      napi::Property::new(env, "set_at_idx_u64")?.with_method(set_at_idx_u64),
      napi::Property::new(env, "set_at_idx_i8")?.with_method(set_at_idx_i8),
      napi::Property::new(env, "set_at_idx_i16")?.with_method(set_at_idx_i16),
      napi::Property::new(env, "set_at_idx_i32")?.with_method(set_at_idx_i32),
      napi::Property::new(env, "set_at_idx_i64")?.with_method(set_at_idx_i64),
      napi::Property::new(env, "get_bool")?.with_method(get_bool),
      napi::Property::new(env, "get_f32")?.with_method(get_f32),
      napi::Property::new(env, "get_f64")?.with_method(get_f64),
      napi::Property::new(env, "get_u8")?.with_method(get_u8),
      napi::Property::new(env, "get_u16")?.with_method(get_u16),
      napi::Property::new(env, "get_u32")?.with_method(get_u32),
      napi::Property::new(env, "get_u64")?.with_method(get_u64),
      napi::Property::new(env, "get_i8")?.with_method(get_i8),
      napi::Property::new(env, "get_i16")?.with_method(get_i16),
      napi::Property::new(env, "get_i32")?.with_method(get_i32),
      napi::Property::new(env, "get_i64")?.with_method(get_i64),
      napi::Property::new(env, "get_date")?.with_method(get_date),
      napi::Property::new(env, "get_datetime")?.with_method(get_datetime),
      napi::Property::new(env, "get_str")?.with_method(get_str),
      napi::Property::new(env, "add_u8")?.with_method(add_u8),
      napi::Property::new(env, "add_u16")?.with_method(add_u16),
      napi::Property::new(env, "add_u32")?.with_method(add_u32),
      napi::Property::new(env, "add_u64")?.with_method(add_u64),
      napi::Property::new(env, "add_i8")?.with_method(add_i8),
      napi::Property::new(env, "add_i16")?.with_method(add_i16),
      napi::Property::new(env, "add_i32")?.with_method(add_i32),
      napi::Property::new(env, "add_i64")?.with_method(add_i64),
      napi::Property::new(env, "add_f32")?.with_method(add_f32),
      napi::Property::new(env, "add_f64")?.with_method(add_f64),
      napi::Property::new(env, "sub_u8")?.with_method(sub_u8),
      napi::Property::new(env, "sub_u16")?.with_method(sub_u16),
      napi::Property::new(env, "sub_u32")?.with_method(sub_u32),
      napi::Property::new(env, "sub_u64")?.with_method(sub_u64),
      napi::Property::new(env, "sub_i8")?.with_method(sub_i8),
      napi::Property::new(env, "sub_i16")?.with_method(sub_i16),
      napi::Property::new(env, "sub_i32")?.with_method(sub_i32),
      napi::Property::new(env, "sub_i64")?.with_method(sub_i64),
      napi::Property::new(env, "sub_f32")?.with_method(sub_f32),
      napi::Property::new(env, "sub_f64")?.with_method(sub_f64),
      napi::Property::new(env, "div_u8")?.with_method(div_u8),
      napi::Property::new(env, "div_u16")?.with_method(div_u16),
      napi::Property::new(env, "div_u32")?.with_method(div_u32),
      napi::Property::new(env, "div_u64")?.with_method(div_u64),
      napi::Property::new(env, "div_i8")?.with_method(div_i8),
      napi::Property::new(env, "div_i16")?.with_method(div_i16),
      napi::Property::new(env, "div_i32")?.with_method(div_i32),
      napi::Property::new(env, "div_i64")?.with_method(div_i64),
      napi::Property::new(env, "div_f32")?.with_method(div_f32),
      napi::Property::new(env, "div_f64")?.with_method(div_f64),
      napi::Property::new(env, "mul_u8")?.with_method(mul_u8),
      napi::Property::new(env, "mul_u16")?.with_method(mul_u16),
      napi::Property::new(env, "mul_u32")?.with_method(mul_u32),
      napi::Property::new(env, "mul_u64")?.with_method(mul_u64),
      napi::Property::new(env, "mul_i8")?.with_method(mul_i8),
      napi::Property::new(env, "mul_i16")?.with_method(mul_i16),
      napi::Property::new(env, "mul_i32")?.with_method(mul_i32),
      napi::Property::new(env, "mul_i64")?.with_method(mul_i64),
      napi::Property::new(env, "mul_f32")?.with_method(mul_f32),
      napi::Property::new(env, "mul_f64")?.with_method(mul_f64),
      napi::Property::new(env, "rem_u8")?.with_method(rem_u8),
      napi::Property::new(env, "rem_u16")?.with_method(rem_u16),
      napi::Property::new(env, "rem_u32")?.with_method(rem_u32),
      napi::Property::new(env, "rem_u64")?.with_method(rem_u64),
      napi::Property::new(env, "rem_i8")?.with_method(rem_i8),
      napi::Property::new(env, "rem_i16")?.with_method(rem_i16),
      napi::Property::new(env, "rem_i32")?.with_method(rem_i32),
      napi::Property::new(env, "rem_i64")?.with_method(rem_i64),
      napi::Property::new(env, "rem_f32")?.with_method(rem_f32),
      napi::Property::new(env, "rem_f64")?.with_method(rem_f64),
      napi::Property::new(env, "add_u8_rhs")?.with_method(add_u8_rhs),
      napi::Property::new(env, "add_u16_rhs")?.with_method(add_u16_rhs),
      napi::Property::new(env, "add_u32_rhs")?.with_method(add_u32_rhs),
      napi::Property::new(env, "add_u64_rhs")?.with_method(add_u64_rhs),
      napi::Property::new(env, "add_i8_rhs")?.with_method(add_i8_rhs),
      napi::Property::new(env, "add_i16_rhs")?.with_method(add_i16_rhs),
      napi::Property::new(env, "add_i32_rhs")?.with_method(add_i32_rhs),
      napi::Property::new(env, "add_i64_rhs")?.with_method(add_i64_rhs),
      napi::Property::new(env, "add_f32_rhs")?.with_method(add_f32_rhs),
      napi::Property::new(env, "add_f64_rhs")?.with_method(add_f64_rhs),
      napi::Property::new(env, "sub_u8_rhs")?.with_method(sub_u8_rhs),
      napi::Property::new(env, "sub_u16_rhs")?.with_method(sub_u16_rhs),
      napi::Property::new(env, "sub_u32_rhs")?.with_method(sub_u32_rhs),
      napi::Property::new(env, "sub_u64_rhs")?.with_method(sub_u64_rhs),
      napi::Property::new(env, "sub_i8_rhs")?.with_method(sub_i8_rhs),
      napi::Property::new(env, "sub_i16_rhs")?.with_method(sub_i16_rhs),
      napi::Property::new(env, "sub_i32_rhs")?.with_method(sub_i32_rhs),
      napi::Property::new(env, "sub_i64_rhs")?.with_method(sub_i64_rhs),
      napi::Property::new(env, "sub_f32_rhs")?.with_method(sub_f32_rhs),
      napi::Property::new(env, "sub_f64_rhs")?.with_method(sub_f64_rhs),
      napi::Property::new(env, "div_u8_rhs")?.with_method(div_u8_rhs),
      napi::Property::new(env, "div_u16_rhs")?.with_method(div_u16_rhs),
      napi::Property::new(env, "div_u32_rhs")?.with_method(div_u32_rhs),
      napi::Property::new(env, "div_u64_rhs")?.with_method(div_u64_rhs),
      napi::Property::new(env, "div_i8_rhs")?.with_method(div_i8_rhs),
      napi::Property::new(env, "div_i16_rhs")?.with_method(div_i16_rhs),
      napi::Property::new(env, "div_i32_rhs")?.with_method(div_i32_rhs),
      napi::Property::new(env, "div_i64_rhs")?.with_method(div_i64_rhs),
      napi::Property::new(env, "div_f32_rhs")?.with_method(div_f32_rhs),
      napi::Property::new(env, "div_f64_rhs")?.with_method(div_f64_rhs),
      napi::Property::new(env, "mul_u8_rhs")?.with_method(mul_u8_rhs),
      napi::Property::new(env, "mul_u16_rhs")?.with_method(mul_u16_rhs),
      napi::Property::new(env, "mul_u32_rhs")?.with_method(mul_u32_rhs),
      napi::Property::new(env, "mul_u64_rhs")?.with_method(mul_u64_rhs),
      napi::Property::new(env, "mul_i8_rhs")?.with_method(mul_i8_rhs),
      napi::Property::new(env, "mul_i16_rhs")?.with_method(mul_i16_rhs),
      napi::Property::new(env, "mul_i32_rhs")?.with_method(mul_i32_rhs),
      napi::Property::new(env, "mul_i64_rhs")?.with_method(mul_i64_rhs),
      napi::Property::new(env, "mul_f32_rhs")?.with_method(mul_f32_rhs),
      napi::Property::new(env, "mul_f64_rhs")?.with_method(mul_f64_rhs),
      napi::Property::new(env, "rem_u8_rhs")?.with_method(rem_u8_rhs),
      napi::Property::new(env, "rem_u16_rhs")?.with_method(rem_u16_rhs),
      napi::Property::new(env, "rem_u32_rhs")?.with_method(rem_u32_rhs),
      napi::Property::new(env, "rem_u64_rhs")?.with_method(rem_u64_rhs),
      napi::Property::new(env, "rem_i8_rhs")?.with_method(rem_i8_rhs),
      napi::Property::new(env, "rem_i16_rhs")?.with_method(rem_i16_rhs),
      napi::Property::new(env, "rem_i32_rhs")?.with_method(rem_i32_rhs),
      napi::Property::new(env, "rem_i64_rhs")?.with_method(rem_i64_rhs),
      napi::Property::new(env, "rem_f32_rhs")?.with_method(rem_f32_rhs),
      napi::Property::new(env, "rem_f64_rhs")?.with_method(rem_f64_rhs),
      napi::Property::new(env, "eq_u8")?.with_method(eq_u8),
      napi::Property::new(env, "eq_u16")?.with_method(eq_u16),
      napi::Property::new(env, "eq_u32")?.with_method(eq_u32),
      napi::Property::new(env, "eq_u64")?.with_method(eq_u64),
      napi::Property::new(env, "eq_i8")?.with_method(eq_i8),
      napi::Property::new(env, "eq_i16")?.with_method(eq_i16),
      napi::Property::new(env, "eq_i32")?.with_method(eq_i32),
      napi::Property::new(env, "eq_i64")?.with_method(eq_i64),
      napi::Property::new(env, "eq_f32")?.with_method(eq_f32),
      napi::Property::new(env, "eq_f64")?.with_method(eq_f64),
      napi::Property::new(env, "eq_str")?.with_method(eq_str),
      napi::Property::new(env, "neq_u8")?.with_method(neq_u8),
      napi::Property::new(env, "neq_u16")?.with_method(neq_u16),
      napi::Property::new(env, "neq_u32")?.with_method(neq_u32),
      napi::Property::new(env, "neq_u64")?.with_method(neq_u64),
      napi::Property::new(env, "neq_i8")?.with_method(neq_i8),
      napi::Property::new(env, "neq_i16")?.with_method(neq_i16),
      napi::Property::new(env, "neq_i32")?.with_method(neq_i32),
      napi::Property::new(env, "neq_i64")?.with_method(neq_i64),
      napi::Property::new(env, "neq_f32")?.with_method(neq_f32),
      napi::Property::new(env, "neq_f64")?.with_method(neq_f64),
      napi::Property::new(env, "neq_str")?.with_method(neq_str),
      napi::Property::new(env, "gt_u8")?.with_method(gt_u8),
      napi::Property::new(env, "gt_u16")?.with_method(gt_u16),
      napi::Property::new(env, "gt_u32")?.with_method(gt_u32),
      napi::Property::new(env, "gt_u64")?.with_method(gt_u64),
      napi::Property::new(env, "gt_i8")?.with_method(gt_i8),
      napi::Property::new(env, "gt_i16")?.with_method(gt_i16),
      napi::Property::new(env, "gt_i32")?.with_method(gt_i32),
      napi::Property::new(env, "gt_i64")?.with_method(gt_i64),
      napi::Property::new(env, "gt_f32")?.with_method(gt_f32),
      napi::Property::new(env, "gt_f64")?.with_method(gt_f64),
      napi::Property::new(env, "gt_str")?.with_method(gt_str),
      napi::Property::new(env, "gt_eq_u8")?.with_method(gt_eq_u8),
      napi::Property::new(env, "gt_eq_u16")?.with_method(gt_eq_u16),
      napi::Property::new(env, "gt_eq_u32")?.with_method(gt_eq_u32),
      napi::Property::new(env, "gt_eq_u64")?.with_method(gt_eq_u64),
      napi::Property::new(env, "gt_eq_i8")?.with_method(gt_eq_i8),
      napi::Property::new(env, "gt_eq_i16")?.with_method(gt_eq_i16),
      napi::Property::new(env, "gt_eq_i32")?.with_method(gt_eq_i32),
      napi::Property::new(env, "gt_eq_i64")?.with_method(gt_eq_i64),
      napi::Property::new(env, "gt_eq_f32")?.with_method(gt_eq_f32),
      napi::Property::new(env, "gt_eq_f64")?.with_method(gt_eq_f64),
      napi::Property::new(env, "gt_eq_str")?.with_method(gt_eq_str),
      napi::Property::new(env, "lt_u8")?.with_method(lt_u8),
      napi::Property::new(env, "lt_u16")?.with_method(lt_u16),
      napi::Property::new(env, "lt_u32")?.with_method(lt_u32),
      napi::Property::new(env, "lt_u64")?.with_method(lt_u64),
      napi::Property::new(env, "lt_i8")?.with_method(lt_i8),
      napi::Property::new(env, "lt_i16")?.with_method(lt_i16),
      napi::Property::new(env, "lt_i32")?.with_method(lt_i32),
      napi::Property::new(env, "lt_i64")?.with_method(lt_i64),
      napi::Property::new(env, "lt_f32")?.with_method(lt_f32),
      napi::Property::new(env, "lt_f64")?.with_method(lt_f64),
      napi::Property::new(env, "lt_str")?.with_method(lt_str),
      napi::Property::new(env, "lt_eq_u8")?.with_method(lt_eq_u8),
      napi::Property::new(env, "lt_eq_u16")?.with_method(lt_eq_u16),
      napi::Property::new(env, "lt_eq_u32")?.with_method(lt_eq_u32),
      napi::Property::new(env, "lt_eq_u64")?.with_method(lt_eq_u64),
      napi::Property::new(env, "lt_eq_i8")?.with_method(lt_eq_i8),
      napi::Property::new(env, "lt_eq_i16")?.with_method(lt_eq_i16),
      napi::Property::new(env, "lt_eq_i32")?.with_method(lt_eq_i32),
      napi::Property::new(env, "lt_eq_i64")?.with_method(lt_eq_i64),
      napi::Property::new(env, "lt_eq_f32")?.with_method(lt_eq_f32),
      napi::Property::new(env, "lt_eq_f64")?.with_method(lt_eq_f64),
      napi::Property::new(env, "lt_eq_str")?.with_method(lt_eq_str),
    ])?;

    Ok(series)
  }
}
