import polars_internal from './internals/polars_internal';
import {Stream} from 'stream';

export type DtypeToPrimitive<T> = T extends DataType.Bool ? boolean :
 T extends DataType.Utf8 ? string : T extends DataType.Date ? Date : T extends DataType.UInt64 ? bigint : number

export type PrimitiveToDtype<T> = T extends boolean ? DataType.Bool :
 T extends string ? DataType.Utf8 : T extends Date ? DataType.Date :
 T extends number ? DataType.Float64 : T extends bigint ? DataType.UInt64 :
 T extends ArrayLike<any> ? DataType.List : DataType.Object


export type Optional<T> = T | undefined;

export enum DataType {
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


export type JsDataFrame = any;
export type NullValues = string | Array<string> | Record<string, string> | Map<string, string>;
export type ReadCsvOptions = {
  file: string;
  hasHeader: boolean;
  inferSchemaLength?: number;
  batchSize?: number;
  ignoreErrors?: boolean;
  endRows?: string;
  startRows?: number;
  projection?: Array<number>;
  sep?: string;
  columns?: Array<string>;
  rechunk?: boolean;
  encoding?: 'utf8' | 'utf8-lossy';
  numThreads?: number;
  lowMemory?: boolean;
  commentChar?: string;
  quoteChar?: string;
  nullValues?: NullValues;
  parseDates?: boolean;
};
export type ReadJsonOptions = Partial<{
  file: string;
  inferSchemaLength?: number;
  batchSize?: number;
  inline?: boolean;
}>;
export type JoinOptions = {
  leftOn?: string | Array<string>;
  rightOn?: string | Array<string>;
  on?: string | Array<string>;
  how?: "left" | "inner" | "outer" | "cross";
  suffix?: string;
};
export type WriteCsvOptions = {
  dest?: string | Stream;
  hasHeader?: boolean;
  sep?: string;
};
export type WriteJsonOptions = {
  file?: string;
  multiline?: boolean;
};


export const DTYPE_TO_FFINAME: Record<DataType, string> = {
  [DataType.Int8]: 'i8',
  [DataType.Int16]: 'i16',
  [DataType.Int32]: 'i32',
  [DataType.Int64]: 'i64',
  [DataType.UInt8]: 'u8',
  [DataType.UInt16]: 'u16',
  [DataType.UInt32]: 'u32',
  [DataType.UInt64]: 'u64',
  [DataType.Float32]: 'f32',
  [DataType.Float64]: 'f64',
  [DataType.Bool]: 'bool',
  [DataType.Utf8]: 'str',
  [DataType.List]: 'list',
  [DataType.Date]: 'date',
  [DataType.Datetime]: 'datetime',
  [DataType.Time]: 'time',
  [DataType.Object]: 'object',
  [DataType.Categorical]: 'categorical',
};

const POLARS_TYPE_TO_CONSTRUCTOR: Record<string, string> = {
  Float32: 'new_opt_f32',
  Float64: 'new_opt_f64',
  Int8: 'new_opt_i8',
  Int16: 'new_opt_i16',
  Int32: 'new_opt_i32',
  Int64: 'new_opt_i64',
  UInt8: 'new_opt_u8',
  UInt16: 'new_opt_u16',
  UInt32: 'new_opt_u32',
  UInt64: 'new_opt_u64',
  Date: 'new_opt_date',
  Datetime: 'new_opt_date',
  Bool: 'new_opt_bool',
  Utf8: 'new_str',
  Object: 'new_object',
  List: 'new_list',
};

export const polarsTypeToConstructor = (dtype: DataType): CallableFunction => {
  const constructor = POLARS_TYPE_TO_CONSTRUCTOR[DataType[dtype]];

  if (!constructor) {
    throw new Error(`Cannot construct Series for type ${DataType[dtype]}.`);
  }

  return polars_internal.series[constructor];
};
