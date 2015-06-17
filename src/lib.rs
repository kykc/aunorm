extern crate audsp;

pub struct LinearNormalizer<TSample> {
    min: TSample,
    max: TSample,
}

#[allow(dead_code)]
pub struct LogNormalizer<TSample> {
    min: TSample,
    max: TSample,
    a: TSample,
    b: TSample,
}

pub trait Normalizer<TSample> {
    fn to_normal(&self, TSample) -> TSample;
    fn from_normal(&self, TSample) -> TSample;
}

pub trait NormalizerProvider<TSample> : Normalizer<TSample> {
    fn new(TSample, TSample) -> Self;
    fn boxed(min: TSample, max: TSample) -> Box<Normalizer<TSample>>;
}

impl<TSample: audsp::Numeric> Normalizer<TSample> for LinearNormalizer<TSample> {
    fn to_normal(&self, value: TSample) -> TSample {
        (value - self.min) / (self.max - self.min)
    }

    fn from_normal(&self, value: TSample) -> TSample {
        value * (self.max - self.min) + self.min
    }
}

impl<TSample: audsp::Numeric> Normalizer<TSample> for LogNormalizer<TSample> {
    fn to_normal(&self, value: TSample) -> TSample {
        TSample::ln(value/self.a)/self.b
    }

    fn from_normal(&self, value: TSample) -> TSample {
        TSample::exp(self.b * value) * self.a
    }
}

impl<TSample: audsp::Numeric> NormalizerProvider<TSample> for LinearNormalizer<TSample> where TSample: 'static {
    fn new(min: TSample, max: TSample) -> LinearNormalizer<TSample> {
        LinearNormalizer::<TSample>{min: min, max: max}
    }

    fn boxed(min: TSample, max: TSample) -> Box<Normalizer<TSample>> {
        Box::new(LinearNormalizer::new(min, max))
    }
}

impl<TSample: audsp::Numeric> NormalizerProvider<TSample> for LogNormalizer<TSample> where TSample: 'static {
    fn new(min: TSample, max: TSample) -> LogNormalizer<TSample> {
        let b: TSample = TSample::ln(min/max) / (TSample::zero() - TSample::one());
        let a: TSample = min;

        LogNormalizer{min: min, max: max, a: a, b: b}
    }

    fn boxed(min: TSample, max: TSample) -> Box<Normalizer<TSample>> {
        Box::new(LogNormalizer::new(min, max))
    }
}
