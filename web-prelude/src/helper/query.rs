use std::collections::{HashMap, HashSet};

use chezmoi_entity::metric::{Metric, MetricHeader, MetricTags};
use chezmoi_entity::CowStr;

use crate::helper::value::TimedValue;

#[derive(Debug, Default)]
pub struct QueryCollector<'a> {
    pub history: HashSet<&'a MetricHeader<'a>>,
    pub latest: HashSet<&'a MetricHeader<'a>>,
}

pub struct QueryResult {
    pub history: HistoryResult,
    pub latest: LatestResult,
}

pub struct HistoryResult(HashMap<CowStr<'static>, HashMap<MetricTags<'static>, Vec<TimedValue>>>);

impl HistoryResult {
    pub fn from_iter(iter: impl Iterator<Item = Metric>) -> Self {
        let mut res = HashMap::new();
        for metric in iter {
            let Metric {
                timestamp,
                header,
                value,
            } = metric;
            let MetricHeader { name, tags } = header;
            let by_name: &mut HashMap<MetricTags<'static>, Vec<TimedValue>> =
                res.entry(name).or_default();
            let by_tag: &mut Vec<TimedValue> = by_name.entry(tags).or_default();
            by_tag.push(TimedValue::new(timestamp, value));
        }
        Self(res)
    }

    pub fn find(&self, header: &MetricHeader<'_>) -> Option<Vec<TimedValue>> {
        self.0
            .get(&header.name)
            .and_then(|tags| tags.get(&header.tags))
            .cloned()
    }
}

pub struct LatestResult(HashMap<CowStr<'static>, HashMap<MetricTags<'static>, TimedValue>>);

impl From<Vec<Metric>> for LatestResult {
    fn from(list: Vec<Metric>) -> Self {
        Self::from_iter(list.into_iter())
    }
}

impl LatestResult {
    pub fn from_iter(iter: impl Iterator<Item = Metric>) -> Self {
        let mut res = HashMap::new();
        for metric in iter {
            let Metric {
                timestamp,
                header,
                value,
            } = metric;
            let MetricHeader { name, tags } = header;
            let by_name: &mut HashMap<MetricTags<'static>, TimedValue> =
                res.entry(name).or_default();
            by_name.insert(tags, TimedValue::new(timestamp, value));
        }
        LatestResult(res)
    }

    pub fn find(&self, header: &MetricHeader<'_>) -> Option<TimedValue> {
        self.0
            .get(&header.name)
            .and_then(|tags| tags.get(&header.tags))
            .copied()
    }
}
