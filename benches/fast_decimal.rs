// Copyright 2022 CoD Technologies Corp.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! fast-decimal benchmark

use bencher::{black_box, Bencher};
use fast_decimal::Decimal;

#[inline(always)]
fn parse(s: &str) -> Decimal {
    s.parse().unwrap()
}

pub fn fast_decimal_cmp(bench: &mut Bencher) {
    let x = parse("12345678901.23456789");
    let y = parse("123456.7890123456789");
    bench.iter(|| {
        let _n = black_box(x > y);
    })
}

pub fn fast_decimal_cmp2(bench: &mut Bencher) {
    let x = parse("12345678901.234567");
    let y = parse("123456.789012");
    bench.iter(|| {
        let _n = black_box(x > y);
    })
}
