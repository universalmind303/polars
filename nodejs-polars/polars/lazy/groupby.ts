import {Expr} from "./expr";
import {selectionToExprList} from "../utils";
import pli from "../internals/polars_internal";
import {LazyDataFrame} from "./dataframe";

export interface LazyGroupBy {
  agg(...aggs: Expr[]): LazyDataFrame
  head(n?: number): LazyDataFrame
  tail(n?: number): LazyDataFrame
}


export const LazyGroupBy = (
  _ldf: any,
  by: any[],
  maintainOrder: boolean
): LazyGroupBy => {
  by = selectionToExprList(by, false);

  const baseArgs = {by, _ldf, maintainOrder};
  const unwrap = (args) => LazyDataFrame(pli.ldf.groupby(args));
  const agg = (...aggs: Expr[]) => unwrap({
    aggs: aggs.map(a => a._expr),
    aggMethod: "agg",
    ...baseArgs
  });
  const head = (n=5) => unwrap({
    n,
    aggs: [],
    aggMethod: "head",
    ...baseArgs
  });
  const tail = (n=5) => unwrap({
    n,
    aggs: [],
    aggMethod: "tail",
    ...baseArgs
  });

  return {
    agg,
    head,
    tail
  };
};
