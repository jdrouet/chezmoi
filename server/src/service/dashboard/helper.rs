struct TimelineMerger<'a> {
    first: &'a [(u64, f64)],
    second: &'a [(u64, f64)],
}

impl<'a> Iterator for TimelineMerger<'a> {
    type Item = (u64, f64, f64);

    fn next(&mut self) -> Option<Self::Item> {
        let (Some(first), Some(second)) = (self.first.get(0), self.second.get(0)) else {
            return None;
        };
        let ts = first.0.max(second.0);
        match (self.first.get(1), self.second.get(1)) {
            (Some((first_ts, _)), Some((second_ts, _))) => {
                if first_ts == second_ts {
                    self.first = &self.first[1..];
                    self.second = &self.second[1..];
                } else if first_ts < second_ts {
                    self.first = &self.first[1..];
                } else {
                    self.second = &self.second[1..];
                }
            }
            (Some(_), None) => {
                self.first = &self.first[1..];
            }
            (None, Some(_)) => {
                self.second = &self.second[1..];
            }
            (None, None) => {
                self.first = &self.first[1..];
                self.second = &self.second[1..];
            }
        };
        Some((ts, first.1, second.1))
    }
}

pub(crate) fn merge_timelines<'a>(
    first: &'a [(u64, f64)],
    second: &'a [(u64, f64)],
) -> impl Iterator<Item = (u64, f64, f64)> + 'a {
    TimelineMerger { first, second }
}

#[cfg(test)]
mod tests {
    use super::merge_timelines;

    #[test]
    fn should_return_short_if_one_empty() {
        let first: Vec<(u64, f64)> = Vec::new();
        let second: Vec<(u64, f64)> = (0..5).map(|i| (i as u64, i as f64)).collect();
        let result = Vec::from_iter(merge_timelines(&first, &second));
        assert_eq!(result, vec![])
    }

    #[test]
    fn should_merge_same_timelines() {
        let first: Vec<(u64, f64)> = (0..5).map(|i| (i as u64, i as f64)).collect();
        let second: Vec<(u64, f64)> = (0..5).map(|i| (i as u64, i as f64)).collect();
        let result = Vec::from_iter(merge_timelines(&first, &second));
        assert_eq!(
            result,
            vec![
                (0, 0.0, 0.0),
                (1, 1.0, 1.0),
                (2, 2.0, 2.0),
                (3, 3.0, 3.0),
                (4, 4.0, 4.0)
            ]
        )
    }

    #[test]
    fn should_merge_alternating_timelines() {
        let first: Vec<(u64, f64)> = vec![(0, 0.0), (2, 1.0), (4, 2.0)];
        let second: Vec<(u64, f64)> = vec![(1, 0.0), (3, 1.0)];
        let result = Vec::from_iter(merge_timelines(&first, &second));
        assert_eq!(
            result,
            vec![(1, 0.0, 0.0), (2, 1.0, 0.0), (3, 1.0, 1.0), (4, 2.0, 1.0)]
        )
    }
}
