// Copyright 2021-2022 CoD Technologies Corp.
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

//! bigdecimal benchmark

#![allow(clippy::excessive_precision)]

use bencher::{black_box, Bencher};
use bigdecimal::{BigDecimal, ParseBigDecimalError};
use num_traits::ToPrimitive;
use std::convert::TryInto;

#[inline(always)]
fn parse(s: &str) -> BigDecimal {
    s.parse().unwrap()
}

pub fn bigdecimal_parse(bench: &mut Bencher) {
    bench.iter(|| {
        let _n = parse(black_box("12345678901.23456789"));
    })
}

pub fn bigdecimal_to_string(bench: &mut Bencher) {
    let val = parse("12345678901.23456789");
    bench.iter(|| {
        let _n = black_box(&val).to_string();
    })
}

pub fn bigdecimal_into_f64(bench: &mut Bencher) {
    let val = parse("12345678901.23456789");
    bench.iter(|| {
        let _n: f64 = black_box(&val).to_f64().unwrap();
    })
}

pub fn bigdecimal_into_u64(bench: &mut Bencher) {
    let val = parse("12345678901.23456789");
    bench.iter(|| {
        let _n: u64 = black_box(&val).to_u64().unwrap();
    })
}

#[inline(always)]
fn try_from<T: TryInto<BigDecimal, Error = ParseBigDecimalError>>(val: T) -> BigDecimal {
    val.try_into().unwrap()
}

pub fn bigdecimal_from_f64(bench: &mut Bencher) {
    bench.iter(|| {
        let _n = try_from(black_box(12345678901.23456789_f64));
    })
}

pub fn bigdecimal_cmp(bench: &mut Bencher) {
    let x = parse("12345678901.23456789");
    let y = parse("123456.7890123456789");
    bench.iter(|| {
        let _n = black_box(x > y);
    })
}

pub fn bigdecimal_cmp2(bench: &mut Bencher) {
    let x = parse("12345678901.234567");
    let y = parse("123456.789012");
    bench.iter(|| {
        let _n = black_box(x > y);
    })
}

#[inline(always)]
fn add(x: &BigDecimal, y: &BigDecimal) -> BigDecimal {
    x + y
}

pub fn bigdecimal_add(bench: &mut Bencher) {
    let x = parse("12345678901.23456789");
    let y = parse("123456.7890123456789");
    bench.iter(|| {
        let _n = add(black_box(&x), black_box(&y));
    })
}

#[inline(always)]
fn sub(x: &BigDecimal, y: &BigDecimal) -> BigDecimal {
    x - y
}

pub fn bigdecimal_sub(bench: &mut Bencher) {
    let x = parse("12345678901.23456789");
    let y = parse("123456.7890123456789");
    bench.iter(|| {
        let _n = sub(black_box(&x), black_box(&y));
    })
}

#[inline(always)]
fn mul(x: &BigDecimal, y: &BigDecimal) -> BigDecimal {
    x * y
}

pub fn bigdecimal_mul(bench: &mut Bencher) {
    let x = parse("12345678901.23456789");
    let y = parse("123456.7890123456789");
    bench.iter(|| {
        let _n = mul(black_box(&x), black_box(&y));
    })
}

#[inline(always)]
fn div(x: &BigDecimal, y: &BigDecimal) -> BigDecimal {
    x / y
}

pub fn bigdecimal_div(bench: &mut Bencher) {
    let x = parse("12345678901.23456789");
    let y = parse("123456.7890123456789");
    bench.iter(|| {
        let _n = div(black_box(&x), black_box(&y));
    })
}

pub fn bigdecimal_sqrt(bench: &mut Bencher) {
    let x = parse("12345678901.23456789");
    bench.iter(|| {
        let _n = black_box(&x).sqrt().unwrap();
    })
}
