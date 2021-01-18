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

//! pgnumeric benchmark

use bencher::{black_box, Bencher};
use pgnumeric::{NumericBuf, NumericTryFromError};
use std::convert::{TryFrom, TryInto};

#[inline(always)]
fn parse(s: &str) -> NumericBuf {
    s.parse().unwrap()
}

pub fn pgnumeric_parse(bench: &mut Bencher) {
    bench.iter(|| {
        let _n = parse(black_box("12345678901.23456789"));
    })
}

#[inline(always)]
fn into<'a, T: TryFrom<&'a NumericBuf, Error = NumericTryFromError>>(val: &'a NumericBuf) -> T {
    TryFrom::try_from(val).unwrap()
}

pub fn pgnumeric_into_f64(bench: &mut Bencher) {
    let val = parse("12345678901.23456789");
    bench.iter(|| {
        let _n: f64 = into(black_box(&val));
    })
}

#[inline(always)]
fn try_from<T: TryInto<NumericBuf, Error = NumericTryFromError>>(val: T) -> NumericBuf {
    val.try_into().unwrap()
}

pub fn pgnumeric_from_f64(bench: &mut Bencher) {
    bench.iter(|| {
        let _n = try_from(black_box(12345678901.23456789_f64));
    })
}

#[inline(always)]
fn add(x: &NumericBuf, y: &NumericBuf) -> NumericBuf {
    x + y
}

pub fn pgnumeric_add(bench: &mut Bencher) {
    let x = parse("12345678901.23456789");
    let y = parse("123456.7890123456789");
    bench.iter(|| {
        let _n = add(black_box(&x), black_box(&y));
    })
}

#[inline(always)]
fn sub(x: &NumericBuf, y: &NumericBuf) -> NumericBuf {
    x - y
}

pub fn pgnumeric_sub(bench: &mut Bencher) {
    let x = parse("12345678901.23456789");
    let y = parse("123456.7890123456789");
    bench.iter(|| {
        let _n = sub(black_box(&x), black_box(&y));
    })
}

#[inline(always)]
fn mul(x: &NumericBuf, y: &NumericBuf) -> NumericBuf {
    x * y
}

pub fn pgnumeric_mul(bench: &mut Bencher) {
    let x = parse("12345678901.23456789");
    let y = parse("123456.7890123456789");
    bench.iter(|| {
        let _n = mul(black_box(&x), black_box(&y));
    })
}

#[inline(always)]
fn div(x: &NumericBuf, y: &NumericBuf) -> NumericBuf {
    x / y
}

pub fn pgnumeric_div(bench: &mut Bencher) {
    let x = parse("12345678901.23456789");
    let y = parse("123456.7890123456789");
    bench.iter(|| {
        let _n = div(black_box(&x), black_box(&y));
    })
}
