use polars_core::series::ops::NullBehavior;
use polars_core::prelude::FillNullStrategy;
use napi::{CallContext, JsObject, Result};
use crate::conversion::wrap::*;
use crate::prelude::JsResult;
use polars::prelude::*;
use crate::error::JsPolarsEr;

pub fn get_params(cx: &CallContext) -> Result<WrappedObject> {
  Ok(cx.get::<JsObject>(0)?.into())
}

pub(crate) fn parse_strategy(strat: String) -> FillNullStrategy {
  match strat.as_str() {
    "backward" => FillNullStrategy::Backward,
    "forward" => FillNullStrategy::Forward,
    "min" => FillNullStrategy::Min,
    "max" => FillNullStrategy::Max,
    "mean" => FillNullStrategy::Mean,
    "zero" => FillNullStrategy::Zero,
    "one" => FillNullStrategy::One,
    s => panic!("Strategy {} not supported", s),
  }
}
pub(crate) fn str_to_rankmethod(method: String) -> JsResult<RankMethod> {
  let method = match method.as_str() {
    "min" => RankMethod::Min,
    "max" => RankMethod::Max,
    "average" => RankMethod::Average,
    "dense" => RankMethod::Dense,
    "ordinal" => RankMethod::Ordinal,
    "random" => RankMethod::Random,
    _ => {
      return Err(
        JsPolarsEr::Other("use one of 'avg, min, max, dense, ordinal'".to_string()).into(),
      )
    }
  };
  Ok(method)
}

pub(crate) fn str_to_null_behavior(null_behavior: String) -> JsResult<NullBehavior> {
  let null_behavior = match null_behavior.as_str() {
    "drop" => NullBehavior::Drop,
    "ignore" => NullBehavior::Ignore,
    _ => return Err(JsPolarsEr::Other("use one of 'drop', 'ignore'".to_string()).into()),
  };
  Ok(null_behavior)
}