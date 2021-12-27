use crate::conversion::prelude::*;
use crate::datatypes::JsDataType;
use crate::error::JsPolarsEr;
use crate::file::JsWriteStream;
use crate::prelude::JsResult;
use napi::{
    CallContext, Either, JsBoolean, JsExternal, JsNumber, JsObject, JsString, JsUndefined,
    JsUnknown,
};
use polars::frame::groupby::GroupBy;
use polars::frame::row::{rows_to_schema, Row};
use polars::prelude::*;
use std::fs::File;
use std::io::{BufReader, Cursor};
use std::path::{Path, PathBuf};




#[js_function(1)]
pub(crate) fn read_columns(cx: CallContext) -> JsResult<JsExternal> {
  let params = get_params(&cx)?;
  let columns: JsObject = params.0.get_named_property("columns")?;
  let len = columns.get_array_length()?;
  let cols: Vec<Series> = (0..len)
      .map(|idx| {
          let item: JsExternal = columns.get_element(idx).expect("Out of bounds");
          let series: &Series = cx
              .env
              .get_value_external(&item)
              .expect("item is not 'series'");
          series.to_owned()
      })
      .collect();

  DataFrame::new(cols)
      .map_err(JsPolarsEr::from)?
      .try_into_js(&cx)
}

#[js_function(1)]
pub(crate) fn read_csv(cx: CallContext) -> JsResult<JsExternal> {
  let params = get_params(&cx)?;

  let chunk_size: usize = params.get_as("batchSize")?;
  let columns: Option<Vec<String>> = params.get_as("columns")?;
  let comment_char: Option<&str> = params.get_as("commentChar")?;
  let encoding: &str = params.get_as("encoding")?;
  let has_header: bool = params.get_as("hasHeader")?;
  let ignore_errors: bool = params.get_as("ignoreErrors")?;
  let infer_schema_length: Option<usize> = params.get_as("inferSchemaLength")?;
  let inline: Option<bool> = params.get_as("inline")?;
  let low_memory: bool = params.get_as("lowMemory")?;
  let n_threads: Option<usize> = params.get_as("numThreads")?;
  let null_values: Option<Wrap<NullValues>> = params.get_as("nullValues")?;
  let parse_dates: bool = params.get_as("parseDates")?;
  let path = params.get_as::<String>("file")?;
  let projection: Option<Vec<usize>> = params.get_as("projection")?;
  let quote_char: Option<&str> = params.get_as("quoteChar")?;
  let rechunk: bool = params.get_as("rechunk")?;
  let sep: &str = params.get_as("sep")?;
  let skip_rows: usize = params.get_as("startRows")?;
  let stop_after_n_rows: Option<usize> = params.get_as("endRows")?;
  let null_values = null_values.map(|w| w.0);
  let comment_char = comment_char.map(|s| s.as_bytes()[0]);

  let quote_char = if let Some(s) = quote_char {
      if s.is_empty() {
          None
      } else {
          Some(s.as_bytes()[0])
      }
  } else {
      None
  };
  let encoding = match encoding {
      "utf8" => CsvEncoding::Utf8,
      "utf8-lossy" => CsvEncoding::LossyUtf8,
      e => return Err(JsPolarsEr::Other(format!("encoding not {} not implemented.", e)).into()),
  };
  let df = if inline.unwrap_or(false) {
      let data: JsString = params.0.get_named_property("file")?;
      let utf = data.into_utf8()?;
      let string_slice = utf.as_slice();
      let c = Cursor::new(string_slice);
      CsvReader::new(c)
          .infer_schema(infer_schema_length)
          .has_header(has_header)
          .with_n_rows(stop_after_n_rows)
          .with_delimiter(sep.as_bytes()[0])
          .with_skip_rows(skip_rows)
          .with_ignore_parser_errors(ignore_errors)
          .with_projection(projection)
          .with_rechunk(rechunk)
          .with_chunk_size(chunk_size)
          .with_encoding(encoding)
          .with_columns(columns)
          .with_n_threads(n_threads)
          .low_memory(low_memory)
          .with_comment_char(comment_char)
          .with_null_values(null_values)
          .with_parse_dates(parse_dates)
          .with_quote_char(quote_char)
          .finish()
          .map_err(JsPolarsEr::from)?
  } else {
      CsvReader::from_path(path)
          .expect("unable to read file")
          .infer_schema(infer_schema_length)
          .has_header(has_header)
          .with_n_rows(stop_after_n_rows)
          .with_delimiter(sep.as_bytes()[0])
          .with_skip_rows(skip_rows)
          .with_ignore_parser_errors(ignore_errors)
          .with_projection(projection)
          .with_rechunk(rechunk)
          .with_chunk_size(chunk_size)
          .with_encoding(encoding)
          .with_columns(columns)
          .with_n_threads(n_threads)
          .low_memory(low_memory)
          .with_comment_char(comment_char)
          .with_null_values(null_values)
          .with_parse_dates(parse_dates)
          .with_quote_char(quote_char)
          .finish()
          .map_err(JsPolarsEr::from)?
  };
  df.try_into_js(&cx)
}

#[js_function(1)]
pub(crate) fn read_parquet(_cx: CallContext) -> JsResult<JsExternal> {
  todo!()
}

#[js_function(1)]
#[cfg(feature = "ipc")]
pub(crate) fn read_ipc(_cx: CallContext) -> JsResult<JsExternal> {
  todo!()
}

#[js_function(1)]
pub(crate) fn read_json(cx: CallContext) -> JsResult<JsExternal> {
  let params = get_params(&cx)?;
  let inline = params.get_as::<Option<bool>>("inline")?;
  let infer_schema_length = params.get_as::<Option<usize>>("inferSchemaLength")?;
  let batch_size = params.get_as::<usize>("batchSize")?;
  let df = if inline.unwrap_or(false) {
      let data: JsString = params.0.get_named_property("file")?;
      let utf = data.into_utf8()?;
      let string_slice = utf.as_slice();
      let c = Cursor::new(string_slice);
      let reader = BufReader::new(c);

      JsonReader::new(reader)
          .infer_schema(infer_schema_length)
          .with_batch_size(batch_size)
          .finish()
          .unwrap()
  } else {
      let path = params.get_as::<&str>("file")?;
      let f = File::open(path)?;
      let reader = BufReader::new(f);
      JsonReader::new(reader)
          .infer_schema(infer_schema_length)
          .with_batch_size(batch_size)
          .finish()
          .unwrap()
  };

  df.try_into_js(&cx)
}

#[js_function(1)]
pub(crate) fn to_json(cx: CallContext) -> JsResult<JsUndefined> {
  let params = get_params(&cx)?;
  let df = params.get_external::<DataFrame>(&cx, "_df")?;
  let stream = params.get::<JsObject>("writeStream")?;
  let writeable = JsWriteStream {
      inner: stream,
      env: cx.env,
  };
  serde_json::to_writer(writeable, df)?;

  cx.env.get_undefined()
}

#[js_function(1)]
pub(crate) fn to_js(cx: CallContext) -> JsResult<JsUnknown> {
  let params = get_params(&cx)?;
  let df = params.get_external::<DataFrame>(&cx, "_df")?;
  cx.env.to_js_value(df)
}

#[js_function(1)]
pub(crate) fn write_json_stream(cx: CallContext) -> JsResult<JsUndefined> {
  let params = get_params(&cx)?;
  let df = params.get_external::<DataFrame>(&cx, "_df")?;
  let stream = params.get::<JsObject>("writeStream")?;
  let writeable = JsWriteStream {
      inner: stream,
      env: cx.env,
  };
  let w = JsonWriter::new(writeable);
  w.finish(df).unwrap();
  cx.env.get_undefined()
}

#[js_function(1)]
pub(crate) fn write_json(cx: CallContext) -> JsResult<JsUndefined> {
  let params = get_params(&cx)?;
  let df = params.get_external::<DataFrame>(&cx, "_df")?;
  let path = params.get_as::<String>("path")?;
  let p = std::path::Path::new(&path);
  let p = resolve_homedir(p);
  let f = File::create(&p)?;
  let w = JsonWriter::new(f);
  w.finish(df).unwrap();
  cx.env.get_undefined()
}

#[js_function(1)]
pub(crate) fn to_rows(cx: CallContext) -> JsResult<JsObject> {
  let params = get_params(&cx)?;
  let df = params.get_external::<DataFrame>(&cx, "_df")?;
  let mut arr = cx.env.create_array()?;
  for idx in 0..df.height() {
      let mut arr_row = cx.env.create_array()?;
      for (i, col) in df.get_columns().iter().enumerate() {
          let val: Wrap<AnyValue> = col.get(idx).into();
          let jsv = val.into_js(&cx);
          arr_row.set_element(i as u32, jsv)?;
      }
      arr.set_element(idx as u32, arr_row)?;
  }
  Ok(arr)
}
#[js_function(1)]
pub(crate) fn to_row(cx: CallContext) -> JsResult<JsObject> {
  let params = get_params(&cx)?;
  let df = params.get_external::<DataFrame>(&cx, "_df")?;
  let idx = params.get_as::<i64>("idx")?;
  let idx = if idx < 0 {
      (df.height() as i64 + idx) as usize
  } else {
      idx as usize
  };

  let mut row = cx.env.create_array()?;
  for (i, col) in df.get_columns().iter().enumerate() {
      let val: Wrap<AnyValue> = col.get(idx).into();
      let jsv = val.into_js(&cx);
      row.set_element(i as u32, jsv)?;
  }
  Ok(row)
}

#[js_function(1)]
pub(crate) fn read_rows(cx: CallContext) -> JsResult<JsExternal> {
  let params = get_params(&cx)?;
  let rows = params.get::<JsObject>("rows")?;
  let len = rows.get_array_length()?;
  let keys = rows
      .get_element_unchecked::<JsObject>(0)?
      .get_property_names()?
      .into_unknown();

  let keys = Vec::<String>::from_js(keys)?;

  let rows: Vec<Row> = (0..len)
      .map(|idx| {
          let obj: JsObject = rows.get_element_unchecked(idx).unwrap();
          let keys = obj.get_property_names().unwrap();
          let keys_len = keys.get_array_length_unchecked().unwrap();
          Row((0..keys_len)
              .map(|key_idx| {
                  let key: JsString = keys.get_element_unchecked(key_idx).unwrap();
                  let value: JsUnknown = obj.get_property(key).unwrap();
                  AnyValue::from_js(value).unwrap()
              })
              .collect())
      })
      .collect();
  let mut df = finish_from_rows(rows)?;
  df.set_column_names(&keys).map_err(JsPolarsEr::from)?;
  df.try_into_js(&cx)
}

#[js_function(1)]
pub(crate) fn read_array_rows(cx: CallContext) -> JsResult<JsExternal> {
  let params = get_params(&cx)?;
  let rows = params.get::<JsObject>("data")?;
  let len = rows.get_array_length()?;
  let err_message = "There was an error while processing rows, \ndata must be an array of arrays";
  let rows: Vec<Row> = (0..len)
      .map(|idx| {
          let arr: JsObject = rows.get_element_unchecked(idx).expect(err_message);
          let arr_len = arr.get_array_length().expect(err_message);
          Row((0..arr_len)
              .map(|arr_idx| {
                  let value: JsUnknown = arr.get_element(arr_idx).expect(err_message);
                  AnyValue::from_js(value).expect("unable to cast value")
              })
              .collect())
      })
      .collect();
  finish_from_rows(rows)?.try_into_js(&cx)
}

#[js_function(1)]
pub(crate) fn write_csv_stream(cx: CallContext) -> JsResult<JsUndefined> {
  let params = get_params(&cx)?;
  let df = params.get_external::<DataFrame>(&cx, "_df")?;
  let has_headers: bool = params.get_as("hasHeader")?;

  let sep: String = params.get_as("sep")?;
  let sep = sep.chars().next().unwrap();

  let stream = params.get::<JsObject>("writeStream")?;
  let writeable = JsWriteStream {
      inner: stream,
      env: cx.env,
  };

  CsvWriter::new(writeable)
      .has_header(has_headers)
      .with_delimiter(sep as u8)
      .finish(&df)
      .map_err(JsPolarsEr::from)?;
  cx.env.get_undefined()
}

#[js_function(1)]
pub(crate) fn write_csv(cx: CallContext) -> JsResult<JsUndefined> {
  let params = get_params(&cx)?;
  let df = params.get_external::<DataFrame>(&cx, "_df")?;
  let has_headers: bool = params.get_as("hasHeader")?;

  let sep: String = params.get_as("sep")?;
  let sep = sep.chars().next().unwrap();

  let path = params.get_as::<String>("path")?;
  let p = std::path::Path::new(&path);
  let p = resolve_homedir(p);
  let f = File::create(&p)?;

  CsvWriter::new(f)
      .has_header(has_headers)
      .with_delimiter(sep as u8)
      .finish(&df)
      .map_err(JsPolarsEr::from)?;
  cx.env.get_undefined()
}