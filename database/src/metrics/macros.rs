#[macro_export(local_inner_macros)]
macro_rules! tag_value_bind {
    ($qb:ident, $value:ident) => {
        match $value {
            $crate::metrics::MetricTagValue::Text(inner) => $qb.push_bind(inner.as_ref()),
            $crate::metrics::MetricTagValue::ArcText(inner) => $qb.push_bind(inner.as_ref()),
            $crate::metrics::MetricTagValue::Float(inner) => $qb.push_bind(inner),
            $crate::metrics::MetricTagValue::Int(inner) => $qb.push_bind(inner),
            $crate::metrics::MetricTagValue::Boolean(inner) => $qb.push_bind(inner),
        }
    };
}
