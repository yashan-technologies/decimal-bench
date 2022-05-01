// Copyright 2021 CoD Technologies Corp.
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

//! rust-decimal benchmark

#![allow(clippy::excessive_precision)]

use bencher::{black_box, Bencher};
use num_traits::{FromPrimitive, ToPrimitive};
use rust_decimal::{Decimal, MathematicalOps};

#[inline(always)]
fn parse(s: &str) -> Decimal {
    s.parse().unwrap()
}

pub fn rust_decimal_parse(bench: &mut Bencher) {
    bench.iter(|| {
        let _n = parse(black_box("12345678901.23456789"));
    })
}

pub fn rust_decimal_to_string(bench: &mut Bencher) {
    let val = parse("12345678901.23456789");
    bench.iter(|| {
        let _n = black_box(&val).to_string();
    })
}

pub fn rust_decimal_into_f64(bench: &mut Bencher) {
    let val = parse("12345678901.23456789");
    bench.iter(|| {
        let _n: f64 = black_box(&val).to_f64().unwrap();
    })
}

pub fn rust_decimal_into_u64(bench: &mut Bencher) {
    let val = parse("12345678901.23456789");
    bench.iter(|| {
        let _n: u64 = black_box(&val).to_u64().unwrap();
    })
}

pub fn rust_decimal_from_f64(bench: &mut Bencher) {
    bench.iter(|| {
        let _n = Decimal::from_f64(black_box(12345678901.23456789_f64)).unwrap();
    })
}

pub fn rust_decimal_cmp(bench: &mut Bencher) {
    let x = parse("12345678901.23456789");
    let y = parse("123456.7890123456789");
    bench.iter(|| {
        let _n = black_box(x > y);
    })
}

pub fn rust_decimal_cmp2(bench: &mut Bencher) {
    let x = parse("12345678901.234567");
    let y = parse("123456.789012");
    bench.iter(|| {
        let _n = black_box(x > y);
    })
}

#[inline(always)]
fn add(x: &Decimal, y: &Decimal) -> Decimal {
    x + y
}

pub fn rust_decimal_add(bench: &mut Bencher) {
    let x = parse("12345678901.23456789");
    let y = parse("123456.7890123456789");
    bench.iter(|| {
        let _n = add(black_box(&x), black_box(&y));
    })
}

#[inline(always)]
fn sub(x: &Decimal, y: &Decimal) -> Decimal {
    x - y
}

pub fn rust_decimal_sub(bench: &mut Bencher) {
    let x = parse("12345678901.23456789");
    let y = parse("123456.7890123456789");
    bench.iter(|| {
        let _n = sub(black_box(&x), black_box(&y));
    })
}

#[inline(always)]
fn mul(x: &Decimal, y: &Decimal) -> Decimal {
    x * y
}

pub fn rust_decimal_mul(bench: &mut Bencher) {
    let x = parse("12345678901.23456789");
    let y = parse("123456.7890123456789");
    bench.iter(|| {
        let _n = mul(black_box(&x), black_box(&y));
    })
}

#[inline(always)]
fn div(x: &Decimal, y: &Decimal) -> Decimal {
    x / y
}

pub fn rust_decimal_div(bench: &mut Bencher) {
    let x = parse("12345678901.23456789");
    let y = parse("123456.7890123456789");
    bench.iter(|| {
        let _n = div(black_box(&x), black_box(&y));
    })
}

pub fn rust_decimal_sqrt(bench: &mut Bencher) {
    let x = parse("12345678901.23456789");
    bench.iter(|| {
        let _n = black_box(&x).sqrt().unwrap();
    })
}
