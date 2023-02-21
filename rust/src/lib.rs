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
