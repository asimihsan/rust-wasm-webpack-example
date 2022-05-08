pub struct Counter {
    value: i64,
}

impl Counter {
    pub fn new(initial_value: i64) -> Self {
        Self {
            value: initial_value,
        }
    }

    pub fn increment(&mut self, delta: i64) {
        self.value = self.value.checked_add(delta).unwrap();
    }

    pub fn get_value(&self) -> i64 {
        self.value
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initially_zero() {
        let mut counter = Counter::new(0);
        assert_eq!(counter.get_value(), 0);
    }

    #[test]
    fn test_increment() {
        let mut counter = Counter::new(1);
        counter.increment(1);
        assert_eq!(counter.get_value(), 2);
    }
}
