import {jsTypeToPolarsType} from "./internals/construction";
import pli from "./internals/polars_internal";


export type TypedArray = Int8Array | Int16Array | Int32Array | BigInt64Array | Uint8Array | Uint16Array | Uint32Array | BigInt64Array | Float32Array | Float64Array;


export type Optional<T> = T | undefined | null;

export namespace DataType {
  export type List = {List: DataType};
  export function List(inner: DataType) {
    return {List: inner} as any as DataType;
  }
  export type Struct = {Struct: {[key: string]: DataType}};
  export function Struct(dtypes) {
    return {Struct: dtypes} as any as DataType;
  }
}
export enum DataType {
  Int8 = "Int8",
  Int16 = "Int16",
  Int32 = "Int32",
  Int64 = "Int64",
  UInt8 = "UInt8",
  UInt16 = "UInt16",
  UInt32 = "UInt32",
  UInt64 = "UInt64",
  Float32 = "Float32",
  Float64 = "Float64",
  Bool = "Bool",
  Utf8 = "Utf8",
  Date = "Date",
  Datetime = "Datetime",
  Time = "Time",
  Object = "Object",
  Categorical = "Categorical",
}


export type JsDataFrame = any;
export type NullValues = string | Array<string> | Record<string, string>;

export type JoinBaseOptions = {
  how?: "left" | "inner" | "outer" | "cross";
  suffix?: string;
}
export type JoinOptions = {
  leftOn?: string | Array<string>;
  rightOn?: string | Array<string>;
  on?: string | Array<string>;
  how?: "left" | "inner" | "outer" | "cross";
  suffix?: string;
};


export const DTYPE_TO_FFINAME: Record<string, string> = {
  [DataType.Int8]: "I8",
  [DataType.Int16]: "I16",
  [DataType.Int32]: "I32",
  [DataType.Int64]: "I64",
  [DataType.UInt8]: "U8",
  [DataType.UInt16]: "U16",
  [DataType.UInt32]: "U32",
  [DataType.UInt64]: "U64",
  [DataType.Float32]: "F32",
  [DataType.Float64]: "F64",
  [DataType.Bool]: "Bool",
  [DataType.Utf8]: "Str",
  [DataType.Date]: "Date",
  [DataType.Datetime]: "Datetime",
  [DataType.Time]: "Time",
  [DataType.Object]: "Object",
  [DataType.Categorical]: "Categorical",
  List: "List",
  Struct: "Struct",
};

const POLARS_TYPE_TO_CONSTRUCTOR: Record<string, any> = {
  Float32(name, values, strict?) {
    return pli.JsSeries.newOptF64(name, values, strict);
  },
  Float64(name, values, strict?) {
    return pli.JsSeries.newOptF64(name, values, strict);
  },
  Int8(name, values, strict?) {
    return pli.JsSeries.newOptI32(name, values, strict);
  },
  Int16(name, values, strict?) {
    return pli.JsSeries.newOptI32(name, values, strict);
  },
  Int32(name, values, strict?) {
    return pli.JsSeries.newOptI32(name, values, strict);
  },
  Int64(name, values, strict?) {
    return pli.JsSeries.newOptI64(name, values, strict);
  },
  UInt8(name, values, strict?) {
    return pli.JsSeries.newOptU32(name, values, strict);
  },
  UInt16(name, values, strict?) {
    return pli.JsSeries.newOptU32(name, values, strict);
  },
  UInt32(name, values, strict?) {
    return pli.JsSeries.newOptU32(name, values, strict);
  },
  UInt64(name, values, strict?) {
    return pli.JsSeries.newOptU64(name, values, strict);
  },
  Date(name, values, strict?) {
    return pli.JsSeries.newOptI64(name, values, strict);
  },
  Datetime(name, values, strict?) {
    return pli.JsSeries.newOptI64(name, values, strict);
  },
  Bool(name, values, strict?) {
    return pli.JsSeries.newOptBool(name, values, strict);
  },
  Utf8(name, values, strict?) {
    return (pli.JsSeries.newOptStr as any)(name, values, strict);
  },
  Categorical(name, values, strict?) {
    return (pli.JsSeries.newOptStr as any)(name, values, strict);
  },
  List(name, values, _strict, dtype) {
    return pli.JsSeries.newList(name, values, dtype);
  },
};

export const polarsTypeToConstructor = (dtype: any): CallableFunction => {
  if((typeof dtype === "object" && (dtype as any).constructor === Object) || typeof dtype === "function") {
    return POLARS_TYPE_TO_CONSTRUCTOR.List;
  }
  const constructor = POLARS_TYPE_TO_CONSTRUCTOR[dtype];
  if (!constructor) {
    throw new Error(`Cannot construct Series for type ${dtype}.`);
  }


  return constructor;
};
