use polars::prelude::DataType;

#[repr(u32)]
pub enum JsDataType {
  Int8,
  Int16,
  Int32,
  Int64,
  UInt8,
  UInt16,
  UInt32,
  UInt64,
  Float32,
  Float64,
  Bool,
  Utf8,
  List,
  Date,
  Datetime,
  Time,
  Object,
  Categorical,
}

impl JsDataType {
  pub fn to_string(self) -> String {
    match self {
      JsDataType::Int8 => "Int8",
      JsDataType::Int16 => "Int16",
      JsDataType::Int32 => "Int32",
      JsDataType::Int64 => "Int64",
      JsDataType::UInt8 => "UInt8",
      JsDataType::UInt16 => "UInt16",
      JsDataType::UInt32 => "UInt32",
      JsDataType::UInt64 => "UInt64",
      JsDataType::Float32 => "Float32",
      JsDataType::Float64 => "Float64",
      JsDataType::Bool => "Bool",
      JsDataType::Utf8 => "Utf8",
      JsDataType::List => "List",
      JsDataType::Date => "Date",
      JsDataType::Datetime => "Datetime",
      JsDataType::Time => "Time",
      JsDataType::Object => "Object",
      JsDataType::Categorical => "Categorical",
    }
    .to_owned()
  }
  pub fn to_str(&self) -> &str {
    match self {
      JsDataType::Int8 => "Int8",
      JsDataType::Int16 => "Int16",
      JsDataType::Int32 => "Int32",
      JsDataType::Int64 => "Int64",
      JsDataType::UInt8 => "UInt8",
      JsDataType::UInt16 => "UInt16",
      JsDataType::UInt32 => "UInt32",
      JsDataType::UInt64 => "UInt64",
      JsDataType::Float32 => "Float32",
      JsDataType::Float64 => "Float64",
      JsDataType::Bool => "Bool",
      JsDataType::Utf8 => "Utf8",
      JsDataType::List => "List",
      JsDataType::Date => "Date",
      JsDataType::Datetime => "Datetime",
      JsDataType::Time => "Time",
      JsDataType::Object => "Object",
      JsDataType::Categorical => "Categorical",
    }
  }
}

impl From<&DataType> for JsDataType {
  fn from(dt: &DataType) -> Self {
    use JsDataType::*;
    match dt {
      DataType::Int8 => Int8,
      DataType::Int16 => Int16,
      DataType::Int32 => Int32,
      DataType::Int64 => Int64,
      DataType::UInt8 => UInt8,
      DataType::UInt16 => UInt16,
      DataType::UInt32 => UInt32,
      DataType::UInt64 => UInt64,
      DataType::Float32 => Float32,
      DataType::Float64 => Float64,
      DataType::Boolean => Bool,
      DataType::Utf8 => Utf8,
      DataType::List(_) => List,
      DataType::Date => Date,
      DataType::Datetime(_, _) => Datetime,
      DataType::Time => Time,

      // DataType::Object(_) => Object,
      DataType::Categorical => Categorical,
      _ => {
        panic!("null or unknown not expected here")
      }
    }
  }
}
