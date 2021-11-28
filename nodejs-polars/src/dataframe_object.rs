use crate::dataframe::*;
use crate::prelude::JsResult;
use napi::JsObject;

impl JsDataFrame {
  pub fn to_object(env: &napi::Env) -> JsResult<JsObject> {
    let mut df = env.create_object()?;

    df.define_properties(&[
      napi::Property::new(env, "add")?.with_method(add),
      napi::Property::new(env, "as_str")?.with_method(as_str),
      napi::Property::new(env, "clone")?.with_method(clone),
      napi::Property::new(env, "column")?.with_method(column),
      napi::Property::new(env, "columns")?.with_method(columns),
      napi::Property::new(env, "div")?.with_method(div),
      napi::Property::new(env, "drop_in_place")?.with_method(drop_in_place),
      napi::Property::new(env, "drop_nulls")?.with_method(drop_nulls),
      napi::Property::new(env, "drop")?.with_method(drop),
      napi::Property::new(env, "dtypes")?.with_method(dtypes),
      napi::Property::new(env, "fill_null")?.with_method(fill_null),
      napi::Property::new(env, "filter")?.with_method(filter),
      napi::Property::new(env, "find_idx_by_name")?.with_method(find_idx_by_name),
      napi::Property::new(env, "frame_equal")?.with_method(frame_equal),
      napi::Property::new(env, "get_columns")?.with_method(get_columns),
      napi::Property::new(env, "groupby")?.with_method(groupby),
      napi::Property::new(env, "hash_rows")?.with_method(hash_rows),
      napi::Property::new(env, "head")?.with_method(head),
      napi::Property::new(env, "height")?.with_method(height),
      napi::Property::new(env, "hstack")?.with_method(hstack),
      napi::Property::new(env, "insert_at_idx")?.with_method(insert_at_idx),
      napi::Property::new(env, "is_duplicated")?.with_method(is_duplicated),
      napi::Property::new(env, "is_unique")?.with_method(is_unique),
      napi::Property::new(env, "join")?.with_method(join),
      napi::Property::new(env, "max")?.with_method(max),
      napi::Property::new(env, "mean")?.with_method(mean),
      napi::Property::new(env, "median")?.with_method(median),
      napi::Property::new(env, "melt")?.with_method(melt),
      napi::Property::new(env, "min")?.with_method(min),
      napi::Property::new(env, "mul")?.with_method(mul),
      napi::Property::new(env, "n_chunks")?.with_method(n_chunks),
      napi::Property::new(env, "null_count")?.with_method(null_count),
      napi::Property::new(env, "quantile")?.with_method(quantile),
      napi::Property::new(env, "read_array_rows")?.with_method(read_array_rows),
      napi::Property::new(env, "read_columns")?.with_method(read_columns),
      napi::Property::new(env, "read_csv")?.with_method(read_csv),
      napi::Property::new(env, "read_json")?.with_method(read_json),
      napi::Property::new(env, "read_parquet")?.with_method(read_parquet),
      napi::Property::new(env, "read_rows")?.with_method(read_rows),
      napi::Property::new(env, "rechunk")?.with_method(rechunk),
      napi::Property::new(env, "rem")?.with_method(rem),
      napi::Property::new(env, "rename")?.with_method(rename),
      napi::Property::new(env, "replace_at_idx")?.with_method(replace_at_idx),
      napi::Property::new(env, "replace")?.with_method(replace),
      napi::Property::new(env, "sample_frac")?.with_method(sample_frac),
      napi::Property::new(env, "sample_n")?.with_method(sample_n),
      napi::Property::new(env, "select_at_idx")?.with_method(select_at_idx),
      napi::Property::new(env, "select")?.with_method(select),
      napi::Property::new(env, "set_column_names")?.with_method(set_column_names),
      napi::Property::new(env, "shape")?.with_method(shape),
      napi::Property::new(env, "shift")?.with_method(shift),
      napi::Property::new(env, "slice")?.with_method(slice),
      napi::Property::new(env, "sort_in_place")?.with_method(sort_in_place),
      napi::Property::new(env, "sort")?.with_method(sort),
      napi::Property::new(env, "std")?.with_method(std),
      napi::Property::new(env, "sub")?.with_method(sub),
      napi::Property::new(env, "sum")?.with_method(sum),
      napi::Property::new(env, "tail")?.with_method(tail),
      napi::Property::new(env, "take_with_series")?.with_method(take_with_series),
      napi::Property::new(env, "take")?.with_method(take),
      napi::Property::new(env, "to_csv")?.with_method(to_csv),
      napi::Property::new(env, "to_json")?.with_method(to_json),
      napi::Property::new(env, "to_rows")?.with_method(to_rows),
      napi::Property::new(env, "var")?.with_method(var),
      napi::Property::new(env, "vstack")?.with_method(vstack),
      napi::Property::new(env, "width")?.with_method(width),
      napi::Property::new(env, "with_row_count")?.with_method(with_row_count),
      napi::Property::new(env, "write_json")?.with_method(write_json),
    ])?;

    Ok(df)
  }
}
