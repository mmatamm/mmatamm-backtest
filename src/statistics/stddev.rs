pub struct StdDevAggregator {
    count: f32,
    mean: f32,
    m2: f32, // NOTE This is unrelated to the M2 measurement
}

impl StdDevAggregator {
    pub fn update(&mut self, value: f32) {
        self.count += 1.0;
        let delta = value - self.mean;
        self.mean += delta / self.count;
        let delta2 = value - self.mean;
        self.m2 += delta * delta2;
    }

    pub fn calc_mean(&self) -> Option<f32> {
        if self.count >= 1.0 {
            Some(self.mean)
        } else {
            None
        }
    }

    pub fn calc_variance(&self, sample: bool) -> Option<f32> {
        if self.count >= 2.0 {
            let count = if sample { self.count - 1.0 } else { self.count };
            Some(self.m2 / count)
        } else {
            None
        }
    }

    pub fn calc_stddev(&self, sample: bool) -> Option<f32> {
        self.calc_variance(sample).map(f32::sqrt)
    }
}

impl Default for StdDevAggregator {
    fn default() -> Self {
        StdDevAggregator {
            count: 0.0,
            mean: 0.0,
            m2: 0.0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use float_eq::assert_float_eq;

    #[test]
    fn mean() {
        let mut aggregator = StdDevAggregator::default();

        aggregator.update(3.0);
        aggregator.update(4.0);
        aggregator.update(5.0);
        assert_float_eq!(aggregator.mean, 4.0, ulps <= 10);
    }

    #[test]
    fn population_variance() {
        let mut aggregator = StdDevAggregator::default();

        aggregator.update(3.0);
        aggregator.update(4.0);
        aggregator.update(5.0);
        assert_float_eq!(
            aggregator.calc_variance(false).unwrap(),
            0.6666667,
            ulps <= 10
        );
    }

    #[test]
    fn sample_variance() {
        let mut aggregator = StdDevAggregator::default();

        aggregator.update(3.0);
        aggregator.update(4.0);
        aggregator.update(5.0);
        assert_float_eq!(aggregator.calc_variance(true).unwrap(), 1.0, ulps <= 10);
    }

    #[test]
    fn population_stddev() {
        let mut aggregator = StdDevAggregator::default();

        aggregator.update(3.0);
        aggregator.update(4.0);
        aggregator.update(5.0);
        assert_float_eq!(
            aggregator.calc_stddev(false).unwrap(),
            0.8164965,
            ulps <= 10
        );
    }
}
