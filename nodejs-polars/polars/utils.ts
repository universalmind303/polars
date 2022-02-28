import {Expr, exprToLitOrExpr} from "./lazy/expr.js";
import type {Series} from "./series/series.js";
import type {DataFrame} from "./dataframe.js";
import path from "path";
import * as typeHelpers from "util/types";
import {TypedArray} from "./datatypes.js";


export type ValueOrArray<T> = T | Array<ValueOrArray<T>>;
export type ColumnSelection = ValueOrArray<string>
export type ExpressionSelection = ValueOrArray<Expr>
export type ColumnsOrExpr = ColumnSelection | ExpressionSelection
export type ExprOrString = Expr | string
export type DownsampleRule =  "month" | "week" | "day" | "hour" | "minute" | "second"
export type FillNullStrategy = "backward" |  "forward" | "mean" | "min" | "max" | "zero" | "one"
export type RankMethod = "average" | "min" | "max" | "dense" | "ordinal" | "random";
export type RollingOptions = {
  windowSize: number,
  weights?: Array<number>,
  minPeriods?: number,
  center?: boolean
};

export function columnOrColumns(columns: ColumnSelection |  string | Array<string> | undefined): Array<string> | undefined {
  if (columns) {
    return columnOrColumnsStrict(columns);
  }
}
export function columnOrColumnsStrict(...columns: string[] | ValueOrArray<string>[]): Array<string> {
  return columns.flat(3) as any;
}
export function selectionToExprList(columns: any[], stringToLit?): Expr[] {
  return [columns].flat(3).map(expr => exprToLitOrExpr(expr, stringToLit)._expr);
}

export function isPath(s: string, expectedExtensions?: string[]): boolean {
  const {base, ext, name} = path.parse(s);

  return Boolean(base && ext && name) && !!(expectedExtensions?.includes(ext));
}

export const range = (start: number, end: number) => {
  const length = end - start;

  return Array.from({ length }, (_, i) => start + i);
};


export const isBrowser = () => typeof window !== `undefined`;
export const isExternal = (ty): boolean => {
  if(isBrowser()) {
    return typeof ty.ptr === "number";
  } else {
    return typeHelpers.isExternal(ty);
  }
};

export const isTypedArray = (ty): ty is TypedArray => {
  if(isBrowser()) {
    switch (ty[Symbol.toStringTag]){
    case "Int8Array":
    case "Int16Array":
    case "Int32Array":
    case "Uint8Array":
    case "Uint8ClampedArray":
    case "Uint16Array":
    case "Uint32Array":
    case "Float32Array":
    case "Float64Array":
    case "BigInt64Array":
    case "BigUint64Array":
      return true;
    default:
      return false;
    }
  } else {
    return typeHelpers.isTypedArray(ty);
  }
};

export const isDataFrameArray = (ty: any): ty is DataFrame[] => Array.isArray(ty) &&  isExternal(ty[0]?._df);
export const isSeriesArray = <T>(ty: any): ty is Series<T>[] => Array.isArray(ty) &&  isExternal(ty[0]?._series);
export const isExprArray = (ty: any): ty is Expr[] => Array.isArray(ty) && isExternal(ty[0]?._expr);
export const isIterator = <T>(ty: any): ty is Iterable<T> => ty !== null && typeof ty[Symbol.iterator] === "function";
export const regexToString = (r: string | RegExp): string => {
  if(isRegExp(r)) {
    return r.source;
  }


  return r;
};

export const INSPECT_SYMBOL = Symbol.for("nodejs.util.inspect.custom");


function isObject(arg): arg is object {
  return typeof arg === "object" && arg !== null;
}

function objectToString(o): string {
  return Object.prototype.toString.call(o);
}

function isRegExp(re): re is RegExp {
  return isObject(re) && objectToString(re) === "[object RegExp]";
}