#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Interval {
    pub min: f64,
    pub max: f64,
}

impl Interval {
    pub const fn new(min: f64, max: f64) -> Self {
        Interval { min, max }
    }

    pub const fn size(&self) -> f64 {
        self.max - self.min
    }

    pub const fn contains(&self, x: f64) -> bool {
        self.min <= x && x <= self.max
    }

    pub const fn surrounds(&self, x: f64) -> bool {
        self.min < x && x < self.max
    }

    pub const fn empty() -> Self {
        Interval {
            min: f64::INFINITY,
            max: f64::NEG_INFINITY,
        }
    }

    pub const fn universe() -> Self {
        Interval {
            min: f64::NEG_INFINITY,
            max: f64::INFINITY,
        }
    }
}
