/*
 * Copyright (C) 2023 Asim Ihsan
 * SPDX-License-Identifier: AGPL-3.0-only
 *
 * This program is free software: you can redistribute it and/or modify it under
 * the terms of the GNU Affero General Public License as published by the Free
 * Software Foundation, version 3.
 *
 * This program is distributed in the hope that it will be useful, but WITHOUT ANY
 * WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A
 * PARTICULAR PURPOSE. See the GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License along
 * with this program. If not, see <https://www.gnu.org/licenses/>
 */

#[derive(Debug, thiserror::Error)]
pub enum CounterError {
    #[error("Overflow")]
    Overflow,
}

pub struct Counter {
    value: i64,
}

impl Counter {
    pub fn new(initial_value: i64) -> Self {
        Self {
            value: initial_value,
        }
    }

    pub fn increment(&mut self, delta: i64) -> Result<(), CounterError> {
        self.value = self
            .value
            .checked_add(delta)
            .ok_or(CounterError::Overflow)?;
        Ok(())
    }

    pub fn get_value(&self) -> i64 {
        self.value
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    #[test]
    fn test_initially_zero() {
        let counter = Counter::new(0);
        assert_eq!(counter.get_value(), 0);
    }

    #[test]
    fn test_increment() {
        let mut counter = Counter::new(1);
        let increment_result = counter.increment(1);
        assert!(increment_result.is_ok());
        assert_eq!(counter.get_value(), 2);
    }

    proptest! {
        // Test for random initial_value and delta that if the actual addition can be done without
        // overflow then the counter returns ok and the value is the expected value. but if the
        // addition overflows then the counter returns an error and the value is unchanged.
        #[test]
        fn test_increment_random(initial_value in i64::MIN..i64::MAX, delta in i64::MIN..i64::MAX) {
            let mut counter = Counter::new(initial_value);
            let expected_value = initial_value.checked_add(delta);
            let increment_result = counter.increment(delta);
            match expected_value {
                Some(expected_value) => {
                    assert!(increment_result.is_ok());
                    assert_eq!(counter.get_value(), expected_value);
                },
                None => {
                    assert!(increment_result.is_err());
                    assert_eq!(counter.get_value(), initial_value);
                }
            }
        }
    }
}
