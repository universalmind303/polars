use crate::prelude::{AnonymousScanOptions, PhysicalExpr};
use polars_core::prelude::*;
use std::fmt::{Debug, Formatter};

pub trait AnonymousScan: Send + Sync {
    fn finish(&self, scan_opts: AnonymousScanOptions, predicate: Option<Arc<dyn PhysicalExpr>>) -> Result<DataFrame>;
    fn schema(&self, _infer_schema_length: Option<usize>) -> Result<Schema> {
        Err(PolarsError::ComputeError(
            "Must supply either a schema or a schema function".into(),
        ))
    }
}

impl<F> AnonymousScan for F
where
    F: Fn(AnonymousScanOptions, Option<Arc<dyn PhysicalExpr>>) -> Result<DataFrame> + Send + Sync,
{
    fn finish(&self, scan_opts: AnonymousScanOptions, predicate: Option<Arc<dyn PhysicalExpr>>) -> Result<DataFrame> {
        self(scan_opts, predicate)
    }
}


impl Debug for dyn AnonymousScan {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "anonymous_scan")
    }
}
