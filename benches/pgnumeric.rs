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

use bencher::{benchmark_group, benchmark_main, black_box, Bencher};
use pgnumeric::{NumericBuf, NumericTryFromError};
use std::convert::{TryFrom, TryInto};

#[inline(always)]
fn pgnumeric_parse(s: &str) -> NumericBuf {
    s.parse().unwrap()
}

fn pgnumeric_parse_u8(bench: &mut Bencher) {
    bench.iter(|| {
        let _n = pgnumeric_parse(black_box("255"));
    })
}

fn pgnumeric_parse_u16(bench: &mut Bencher) {
    bench.iter(|| {
        let _n = pgnumeric_parse(black_box("65535"));
    })
}

fn pgnumeric_parse_u32(bench: &mut Bencher) {
    bench.iter(|| {
        let _n = pgnumeric_parse(black_box("4294967295"));
    })
}

fn pgnumeric_parse_u64(bench: &mut Bencher) {
    bench.iter(|| {
        let _n = pgnumeric_parse(black_box("18446744073709551615"));
    })
}

fn pgnumeric_parse_u128(bench: &mut Bencher) {
    bench.iter(|| {
        let _n = pgnumeric_parse(black_box("340282366920938463463374607431768211455"));
    })
}

fn pgnumeric_parse_f32(bench: &mut Bencher) {
    bench.iter(|| {
        let _n = pgnumeric_parse(black_box("1.23456789e10"));
    })
}

fn pgnumeric_parse_f64(bench: &mut Bencher) {
    bench.iter(|| {
        let _n = pgnumeric_parse(black_box("1.234567890123456789e10"));
    })
}

#[inline(always)]
fn pgnumeric_into<'a, T: TryFrom<&'a NumericBuf, Error = NumericTryFromError>>(
    val: &'a NumericBuf,
) -> T {
    TryFrom::try_from(val).unwrap()
}

fn pgnumeric_into_u8(bench: &mut Bencher) {
    let val = pgnumeric_parse("255");
    bench.iter(|| {
        let _n: u8 = pgnumeric_into(black_box(&val));
    })
}

fn pgnumeric_into_u16(bench: &mut Bencher) {
    let val = pgnumeric_parse("65535");
    bench.iter(|| {
        let _n: u16 = pgnumeric_into(black_box(&val));
    })
}

fn pgnumeric_into_u32(bench: &mut Bencher) {
    let val = pgnumeric_parse("4294967295");
    bench.iter(|| {
        let _n: u32 = pgnumeric_into(black_box(&val));
    })
}

fn pgnumeric_into_u64(bench: &mut Bencher) {
    let val = pgnumeric_parse("18446744073709551615");
    bench.iter(|| {
        let _n: u64 = pgnumeric_into(black_box(&val));
    })
}

fn pgnumeric_into_u128(bench: &mut Bencher) {
    let val = pgnumeric_parse("340282366920938463463374607431768211455");
    bench.iter(|| {
        let _n: u128 = pgnumeric_into(black_box(&val));
    })
}

fn pgnumeric_into_f32(bench: &mut Bencher) {
    let val = pgnumeric_parse("1.23456789e10");
    bench.iter(|| {
        let _n: f32 = pgnumeric_into(black_box(&val));
    })
}

fn pgnumeric_into_f64(bench: &mut Bencher) {
    let val = pgnumeric_parse("1.234567890123456789e10");
    bench.iter(|| {
        let _n: f32 = pgnumeric_into(black_box(&val));
    })
}

#[inline(always)]
fn pgnumeric_from<T: Into<NumericBuf>>(val: T) -> NumericBuf {
    val.into()
}

#[inline(always)]
fn pgnumeric_try_from<T: TryInto<NumericBuf, Error = NumericTryFromError>>(val: T) -> NumericBuf {
    val.try_into().unwrap()
}

fn pgnumeric_from_u8(bench: &mut Bencher) {
    bench.iter(|| {
        let _n = pgnumeric_from(black_box(255_u8));
    })
}

fn pgnumeric_from_u16(bench: &mut Bencher) {
    bench.iter(|| {
        let _n = pgnumeric_from(black_box(65535_u16));
    })
}

fn pgnumeric_from_u32(bench: &mut Bencher) {
    bench.iter(|| {
        let _n = pgnumeric_from(black_box(4294967295_u32));
    })
}

fn pgnumeric_from_u64(bench: &mut Bencher) {
    bench.iter(|| {
        let _n = pgnumeric_from(black_box(18446744073709551615_u64));
    })
}

fn pgnumeric_from_u128(bench: &mut Bencher) {
    bench.iter(|| {
        let _n = pgnumeric_from(black_box(340282366920938463463374607431768211455_u128));
    })
}

fn pgnumeric_from_f32(bench: &mut Bencher) {
    bench.iter(|| {
        let _n = pgnumeric_try_from(black_box(1.23456789e10_f32));
    })
}

fn pgnumeric_from_f64(bench: &mut Bencher) {
    bench.iter(|| {
        let _n = pgnumeric_try_from(black_box(1.234567890123456789e10_f64));
    })
}

#[inline(always)]
fn pgnumeric_add(x: &NumericBuf, y: &NumericBuf) -> NumericBuf {
    x + y
}

fn pgnumeric_add_u8(bench: &mut Bencher) {
    let x = pgnumeric_parse("127");
    bench.iter(|| {
        let _n = pgnumeric_add(black_box(&x), black_box(&x));
    })
}

fn pgnumeric_add_u16(bench: &mut Bencher) {
    let x = pgnumeric_parse("32767");
    bench.iter(|| {
        let _n = pgnumeric_add(black_box(&x), black_box(&x));
    })
}

fn pgnumeric_add_u32(bench: &mut Bencher) {
    let x = pgnumeric_parse("2147483647");
    bench.iter(|| {
        let _n = pgnumeric_add(black_box(&x), black_box(&x));
    })
}

fn pgnumeric_add_u64(bench: &mut Bencher) {
    let x = pgnumeric_parse("9223372036854775807");
    bench.iter(|| {
        let _n = pgnumeric_add(black_box(&x), black_box(&x));
    })
}

fn pgnumeric_add_u128(bench: &mut Bencher) {
    let x = pgnumeric_parse("170141183460469231731687303715884105727");
    bench.iter(|| {
        let _n = pgnumeric_add(black_box(&x), black_box(&x));
    })
}

fn pgnumeric_add_f32(bench: &mut Bencher) {
    let x = pgnumeric_parse("1.23456789e10");
    bench.iter(|| {
        let _n = pgnumeric_add(black_box(&x), black_box(&x));
    })
}

fn pgnumeric_add_f64(bench: &mut Bencher) {
    let x = pgnumeric_parse("1.234567890123456789e10");
    bench.iter(|| {
        let _n = pgnumeric_add(black_box(&x), black_box(&x));
    })
}

#[inline(always)]
fn pgnumeric_sub(x: &NumericBuf, y: &NumericBuf) -> NumericBuf {
    x - y
}

fn pgnumeric_sub_u8(bench: &mut Bencher) {
    let x = pgnumeric_parse("127");
    let y = pgnumeric_parse("12");
    bench.iter(|| {
        let _n = pgnumeric_sub(black_box(&x), black_box(&y));
    })
}

fn pgnumeric_sub_u16(bench: &mut Bencher) {
    let x = pgnumeric_parse("32767");
    let y = pgnumeric_parse("3276");
    bench.iter(|| {
        let _n = pgnumeric_sub(black_box(&x), black_box(&y));
    })
}

fn pgnumeric_sub_u32(bench: &mut Bencher) {
    let x = pgnumeric_parse("2147483647");
    let y = pgnumeric_parse("214748364");
    bench.iter(|| {
        let _n = pgnumeric_sub(black_box(&x), black_box(&y));
    })
}

fn pgnumeric_sub_u64(bench: &mut Bencher) {
    let x = pgnumeric_parse("9223372036854775807");
    let y = pgnumeric_parse("922337203685477580");
    bench.iter(|| {
        let _n = pgnumeric_sub(black_box(&x), black_box(&y));
    })
}

fn pgnumeric_sub_u128(bench: &mut Bencher) {
    let x = pgnumeric_parse("170141183460469231731687303715884105727");
    let y = pgnumeric_parse("17014118346046923173168730371588410572");
    bench.iter(|| {
        let _n = pgnumeric_sub(black_box(&x), black_box(&y));
    })
}

fn pgnumeric_sub_f32(bench: &mut Bencher) {
    let x = pgnumeric_parse("1.23456789e10");
    let y = pgnumeric_parse("1.23456789e9");
    bench.iter(|| {
        let _n = pgnumeric_sub(black_box(&x), black_box(&y));
    })
}

fn pgnumeric_sub_f64(bench: &mut Bencher) {
    let x = pgnumeric_parse("1.234567890123456789e10");
    let y = pgnumeric_parse("1.234567890123456789e9");
    bench.iter(|| {
        let _n = pgnumeric_sub(black_box(&x), black_box(&y));
    })
}

#[inline(always)]
fn pgnumeric_mul(x: &NumericBuf, y: &NumericBuf) -> NumericBuf {
    x * y
}

fn pgnumeric_mul_u8(bench: &mut Bencher) {
    let x = pgnumeric_parse("127");
    let y = pgnumeric_parse("12");
    bench.iter(|| {
        let _n = pgnumeric_mul(black_box(&x), black_box(&y));
    })
}

fn pgnumeric_mul_u16(bench: &mut Bencher) {
    let x = pgnumeric_parse("32767");
    let y = pgnumeric_parse("3276");
    bench.iter(|| {
        let _n = pgnumeric_mul(black_box(&x), black_box(&y));
    })
}

fn pgnumeric_mul_u32(bench: &mut Bencher) {
    let x = pgnumeric_parse("2147483647");
    let y = pgnumeric_parse("214748364");
    bench.iter(|| {
        let _n = pgnumeric_mul(black_box(&x), black_box(&y));
    })
}

fn pgnumeric_mul_u64(bench: &mut Bencher) {
    let x = pgnumeric_parse("9223372036854775807");
    let y = pgnumeric_parse("922337203685477580");
    bench.iter(|| {
        let _n = pgnumeric_mul(black_box(&x), black_box(&y));
    })
}

fn pgnumeric_mul_u128(bench: &mut Bencher) {
    let x = pgnumeric_parse("170141183460469231731687303715884105727");
    let y = pgnumeric_parse("17014118346046923173168730371588410572");
    bench.iter(|| {
        let _n = pgnumeric_mul(black_box(&x), black_box(&y));
    })
}

fn pgnumeric_mul_f32(bench: &mut Bencher) {
    let x = pgnumeric_parse("1.23456789e10");
    let y = pgnumeric_parse("1.23456789e9");
    bench.iter(|| {
        let _n = pgnumeric_mul(black_box(&x), black_box(&y));
    })
}

fn pgnumeric_mul_f64(bench: &mut Bencher) {
    let x = pgnumeric_parse("1.234567890123456789e10");
    let y = pgnumeric_parse("1.234567890123456789e9");
    bench.iter(|| {
        let _n = pgnumeric_mul(black_box(&x), black_box(&y));
    })
}

#[inline(always)]
fn pgnumeric_div(x: &NumericBuf, y: &NumericBuf) -> NumericBuf {
    x / y
}

fn pgnumeric_div_u8(bench: &mut Bencher) {
    let x = pgnumeric_parse("127");
    let y = pgnumeric_parse("12");
    bench.iter(|| {
        let _n = pgnumeric_div(black_box(&x), black_box(&y));
    })
}

fn pgnumeric_div_u16(bench: &mut Bencher) {
    let x = pgnumeric_parse("32767");
    let y = pgnumeric_parse("3276");
    bench.iter(|| {
        let _n = pgnumeric_div(black_box(&x), black_box(&y));
    })
}

fn pgnumeric_div_u32(bench: &mut Bencher) {
    let x = pgnumeric_parse("2147483647");
    let y = pgnumeric_parse("214748364");
    bench.iter(|| {
        let _n = pgnumeric_div(black_box(&x), black_box(&y));
    })
}

fn pgnumeric_div_u64(bench: &mut Bencher) {
    let x = pgnumeric_parse("9223372036854775807");
    let y = pgnumeric_parse("922337203685477580");
    bench.iter(|| {
        let _n = pgnumeric_div(black_box(&x), black_box(&y));
    })
}

fn pgnumeric_div_u128(bench: &mut Bencher) {
    let x = pgnumeric_parse("170141183460469231731687303715884105727");
    let y = pgnumeric_parse("17014118346046923173168730371588410572");
    bench.iter(|| {
        let _n = pgnumeric_div(black_box(&x), black_box(&y));
    })
}

fn pgnumeric_div_f32(bench: &mut Bencher) {
    let x = pgnumeric_parse("1.23456789e10");
    let y = pgnumeric_parse("1.23456789e9");
    bench.iter(|| {
        let _n = pgnumeric_div(black_box(&x), black_box(&y));
    })
}

fn pgnumeric_div_f64(bench: &mut Bencher) {
    let x = pgnumeric_parse("1.234567890123456789e10");
    let y = pgnumeric_parse("1.234567890123456789e9");
    bench.iter(|| {
        let _n = pgnumeric_div(black_box(&x), black_box(&y));
    })
}

benchmark_group!(
    pgnumeric_parse_benches,
    pgnumeric_parse_u8,
    pgnumeric_parse_u16,
    pgnumeric_parse_u32,
    pgnumeric_parse_u64,
    pgnumeric_parse_u128,
    pgnumeric_parse_f32,
    pgnumeric_parse_f64,
);
benchmark_group!(
    pgnumeric_convert_benches,
    pgnumeric_into_u8,
    pgnumeric_into_u16,
    pgnumeric_into_u32,
    pgnumeric_into_u64,
    pgnumeric_into_u128,
    pgnumeric_into_f32,
    pgnumeric_into_f64,
);
benchmark_group!(
    pgnumeric_from_benches,
    pgnumeric_from_u8,
    pgnumeric_from_u16,
    pgnumeric_from_u32,
    pgnumeric_from_u64,
    pgnumeric_from_u128,
    pgnumeric_from_f32,
    pgnumeric_from_f64,
);
benchmark_group!(
    pgnumeric_add_benches,
    pgnumeric_add_u8,
    pgnumeric_add_u16,
    pgnumeric_add_u32,
    pgnumeric_add_u64,
    pgnumeric_add_u128,
    pgnumeric_add_f32,
    pgnumeric_add_f64,
);
benchmark_group!(
    pgnumeric_sub_benches,
    pgnumeric_sub_u8,
    pgnumeric_sub_u16,
    pgnumeric_sub_u32,
    pgnumeric_sub_u64,
    pgnumeric_sub_u128,
    pgnumeric_sub_f32,
    pgnumeric_sub_f64,
);
benchmark_group!(
    pgnumeric_mul_benches,
    pgnumeric_mul_u8,
    pgnumeric_mul_u16,
    pgnumeric_mul_u32,
    pgnumeric_mul_u64,
    pgnumeric_mul_u128,
    pgnumeric_mul_f32,
    pgnumeric_mul_f64,
);
benchmark_group!(
    pgnumeric_div_benches,
    pgnumeric_div_u8,
    pgnumeric_div_u16,
    pgnumeric_div_u32,
    pgnumeric_div_u64,
    pgnumeric_div_u128,
    pgnumeric_div_f32,
    pgnumeric_div_f64,
);

benchmark_main!(
    pgnumeric_parse_benches,
    pgnumeric_convert_benches,
    pgnumeric_from_benches,
    pgnumeric_add_benches,
    pgnumeric_sub_benches,
    pgnumeric_mul_benches,
    pgnumeric_div_benches,
);
