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

use rust::Counter;
use serde_derive::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct CounterWrapper {
    internal: Counter,
}

#[wasm_bindgen]
impl CounterWrapper {
    #[wasm_bindgen(constructor)]
    pub fn new(initial_value: i64) -> Self {
        let counter = Counter::new(initial_value);
        Self { internal: counter }
    }

    pub fn increment(&mut self, delta: i64) {
        self.internal.increment(delta);
    }

    pub fn get_value(&self) -> i64 {
        self.internal.get_value()
    }
}

#[derive(Deserialize)]
pub struct GetIncrementResultInput {
    pub initial_value: String,
}

#[derive(Serialize)]
pub struct GetIncrementResultOutput {
    pub result: String,
}

#[wasm_bindgen]
pub fn get_increment_result(input: String) -> JsValue {
    let result = GetIncrementResultOutput {
        result: String::from("1"),
    };
    JsValue::from_serde(&result).unwrap()
}
