
import * as func from "../lazy/functions.js";
import * as gb from "../lazy/groupby.js";
import * as expr from "../lazy/expr.js";
import * as whenthen from "../lazy/whenthen.js";
namespace lazy {
  export import GroupBy = gb.LazyGroupBy;
  export import Expr = expr;
  export import funcs = func;
  export import when = whenthen;
  export import col = func.col
  export import cols = func.cols
  export import lit = func.lit
  export import arange = func.arange
  export import argSortBy = func.argSortBy
  export import avg = func.avg
  export import concatList = func.concatList
  export import concatString = func.concatString
  export import count = func.count
  export import cov = func.cov
  export import exclude = func.exclude
  export import first = func.first
  export import format = func.format
  export import groups = func.groups
  export import head = func.head
  export import last = func.last
  export import mean = func.mean
  export import median = func.median
  export import nUnique = func.nUnique
  export import pearsonCorr = func.pearsonCorr
  export import quantile = func.quantile
  export import select = func.select
  export import spearmanRankCorr = func.spearmanRankCorr
  export import tail = func.tail
  export import list = func.list
}

export default lazy;
