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
use bigdecimal::{BigDecimal, ParseBigDecimalError};
use num_traits::{FromPrimitive, ToPrimitive};
use std::convert::TryInto;

#[inline(always)]
fn bigdecimal_parse(s: &str) -> BigDecimal {
    s.parse().unwrap()
}

fn bigdecimal_parse_u8(bench: &mut Bencher) {
    bench.iter(|| {
        let _n = bigdecimal_parse(black_box("255"));
    })
}

fn bigdecimal_parse_u16(bench: &mut Bencher) {
    bench.iter(|| {
        let _n = bigdecimal_parse(black_box("65535"));
    })
}

fn bigdecimal_parse_u32(bench: &mut Bencher) {
    bench.iter(|| {
        let _n = bigdecimal_parse(black_box("4294967295"));
    })
}

fn bigdecimal_parse_u64(bench: &mut Bencher) {
    bench.iter(|| {
        let _n = bigdecimal_parse(black_box("18446744073709551615"));
    })
}

fn bigdecimal_parse_u128(bench: &mut Bencher) {
    bench.iter(|| {
        let _n = bigdecimal_parse(black_box("340282366920938463463374607431768211455"));
    })
}

fn bigdecimal_parse_f32(bench: &mut Bencher) {
    bench.iter(|| {
        let _n = bigdecimal_parse(black_box("1.23456789e10"));
    })
}

fn bigdecimal_parse_f64(bench: &mut Bencher) {
    bench.iter(|| {
        let _n = bigdecimal_parse(black_box("1.234567890123456789e10"));
    })
}

// #[inline(always)]
// fn bigdecimal_into<'a, T: TryFrom<&'a BigDecimal, Error=ParseBigDecimalError>>(
//     val: &'a BigDecimal,
// ) -> T {
//     TryFrom::try_from(val).unwrap()
// }

fn bigdecimal_into_u8(bench: &mut Bencher) {
    let val = bigdecimal_parse("255");
    bench.iter(|| {
        let _n: u8 = black_box(&val).to_u8().unwrap();
    })
}

fn bigdecimal_into_u16(bench: &mut Bencher) {
    let val = bigdecimal_parse("65535");
    bench.iter(|| {
        let _n: u16 = black_box(&val).to_u16().unwrap();
    })
}

fn bigdecimal_into_u32(bench: &mut Bencher) {
    let val = bigdecimal_parse("4294967295");
    bench.iter(|| {
        let _n: u32 = black_box(&val).to_u32().unwrap();
    })
}

fn bigdecimal_into_u64(bench: &mut Bencher) {
    let val = bigdecimal_parse("18446744073709551615");
    bench.iter(|| {
        let _n: u64 = black_box(&val).to_u64().unwrap();
    })
}

// fn bigdecimal_into_u128(bench: &mut Bencher) {
//     let val = bigdecimal_parse("79228162514264337593543950335");
//     bench.iter(|| {
//         let _n: u128 = black_box(&val).to_u128().unwrap();
//     })
// }

fn bigdecimal_into_f32(bench: &mut Bencher) {
    let val = bigdecimal_parse("1.23456789e10");
    bench.iter(|| {
        let _n: f32 = black_box(&val).to_f32().unwrap();
    })
}

fn bigdecimal_into_f64(bench: &mut Bencher) {
    let val = bigdecimal_parse("1.234567890123456789e10");
    bench.iter(|| {
        let _n: f64 = black_box(&val).to_f64().unwrap();
    })
}

#[inline(always)]
fn bigdecimal_from<T: Into<BigDecimal>>(val: T) -> BigDecimal {
    val.into()
}

#[inline(always)]
fn bigdecimal_try_from<T: TryInto<BigDecimal, Error = ParseBigDecimalError>>(val: T) -> BigDecimal {
    val.try_into().unwrap()
}

fn bigdecimal_from_u8(bench: &mut Bencher) {
    bench.iter(|| {
        let _n = bigdecimal_from(black_box(255_u8));
    })
}

fn bigdecimal_from_u16(bench: &mut Bencher) {
    bench.iter(|| {
        let _n = bigdecimal_from(black_box(65535_u16));
    })
}

fn bigdecimal_from_u32(bench: &mut Bencher) {
    bench.iter(|| {
        let _n = bigdecimal_from(black_box(4294967295_u32));
    })
}

fn bigdecimal_from_u64(bench: &mut Bencher) {
    bench.iter(|| {
        let _n = bigdecimal_from(black_box(18446744073709551615_u64));
    })
}

fn bigdecimal_from_u128(bench: &mut Bencher) {
    bench.iter(|| {
        let _n = BigDecimal::from_u128(black_box(340282366920938463463374607431768211455_u128));
    })
}

fn bigdecimal_from_f32(bench: &mut Bencher) {
    bench.iter(|| {
        let _n = bigdecimal_try_from(black_box(1.23456789e10_f32));
    })
}

fn bigdecimal_from_f64(bench: &mut Bencher) {
    bench.iter(|| {
        let _n = bigdecimal_try_from(black_box(1.234567890123456789e10_f64));
    })
}

#[inline(always)]
fn bigdecimal_add(x: &BigDecimal, y: &BigDecimal) -> BigDecimal {
    x + y
}

fn bigdecimal_add_u8(bench: &mut Bencher) {
    let x = bigdecimal_parse("127");
    bench.iter(|| {
        let _n = bigdecimal_add(black_box(&x), black_box(&x));
    })
}

fn bigdecimal_add_u16(bench: &mut Bencher) {
    let x = bigdecimal_parse("32767");
    bench.iter(|| {
        let _n = bigdecimal_add(black_box(&x), black_box(&x));
    })
}

fn bigdecimal_add_u32(bench: &mut Bencher) {
    let x = bigdecimal_parse("2147483647");
    bench.iter(|| {
        let _n = bigdecimal_add(black_box(&x), black_box(&x));
    })
}

fn bigdecimal_add_u64(bench: &mut Bencher) {
    let x = bigdecimal_parse("9223372036854775807");
    bench.iter(|| {
        let _n = bigdecimal_add(black_box(&x), black_box(&x));
    })
}

fn bigdecimal_add_u128(bench: &mut Bencher) {
    let x = bigdecimal_parse("170141183460469231731687303715884105727");
    bench.iter(|| {
        let _n = bigdecimal_add(black_box(&x), black_box(&x));
    })
}

fn bigdecimal_add_f32(bench: &mut Bencher) {
    let x = bigdecimal_parse("1.23456789e10");
    bench.iter(|| {
        let _n = bigdecimal_add(black_box(&x), black_box(&x));
    })
}

fn bigdecimal_add_f64(bench: &mut Bencher) {
    let x = bigdecimal_parse("1.234567890123456789e10");
    bench.iter(|| {
        let _n = bigdecimal_add(black_box(&x), black_box(&x));
    })
}

#[inline(always)]
fn bigdecimal_sub(x: &BigDecimal, y: &BigDecimal) -> BigDecimal {
    x - y
}

fn bigdecimal_sub_u8(bench: &mut Bencher) {
    let x = bigdecimal_parse("127");
    let y = bigdecimal_parse("12");
    bench.iter(|| {
        let _n = bigdecimal_sub(black_box(&x), black_box(&y));
    })
}

fn bigdecimal_sub_u16(bench: &mut Bencher) {
    let x = bigdecimal_parse("32767");
    let y = bigdecimal_parse("3276");
    bench.iter(|| {
        let _n = bigdecimal_sub(black_box(&x), black_box(&y));
    })
}

fn bigdecimal_sub_u32(bench: &mut Bencher) {
    let x = bigdecimal_parse("2147483647");
    let y = bigdecimal_parse("214748364");
    bench.iter(|| {
        let _n = bigdecimal_sub(black_box(&x), black_box(&y));
    })
}

fn bigdecimal_sub_u64(bench: &mut Bencher) {
    let x = bigdecimal_parse("9223372036854775807");
    let y = bigdecimal_parse("922337203685477580");
    bench.iter(|| {
        let _n = bigdecimal_sub(black_box(&x), black_box(&y));
    })
}

fn bigdecimal_sub_u128(bench: &mut Bencher) {
    let x = bigdecimal_parse("170141183460469231731687303715884105727");
    let y = bigdecimal_parse("17014118346046923173168730371588410572");
    bench.iter(|| {
        let _n = bigdecimal_sub(black_box(&x), black_box(&y));
    })
}

fn bigdecimal_sub_f32(bench: &mut Bencher) {
    let x = bigdecimal_parse("1.23456789e10");
    let y = bigdecimal_parse("1.23456789e9");
    bench.iter(|| {
        let _n = bigdecimal_sub(black_box(&x), black_box(&y));
    })
}

fn bigdecimal_sub_f64(bench: &mut Bencher) {
    let x = bigdecimal_parse("1.234567890123456789e10");
    let y = bigdecimal_parse("1.234567890123456789e9");
    bench.iter(|| {
        let _n = bigdecimal_sub(black_box(&x), black_box(&y));
    })
}

#[inline(always)]
fn bigdecimal_mul(x: &BigDecimal, y: &BigDecimal) -> BigDecimal {
    x * y
}

fn bigdecimal_mul_u8(bench: &mut Bencher) {
    let x = bigdecimal_parse("127");
    let y = bigdecimal_parse("12");
    bench.iter(|| {
        let _n = bigdecimal_mul(black_box(&x), black_box(&y));
    })
}

fn bigdecimal_mul_u16(bench: &mut Bencher) {
    let x = bigdecimal_parse("32767");
    let y = bigdecimal_parse("3276");
    bench.iter(|| {
        let _n = bigdecimal_mul(black_box(&x), black_box(&y));
    })
}

fn bigdecimal_mul_u32(bench: &mut Bencher) {
    let x = bigdecimal_parse("2147483647");
    let y = bigdecimal_parse("214748364");
    bench.iter(|| {
        let _n = bigdecimal_mul(black_box(&x), black_box(&y));
    })
}

fn bigdecimal_mul_u64(bench: &mut Bencher) {
    let x = bigdecimal_parse("9223372036854775807");
    let y = bigdecimal_parse("922337203685477580");
    bench.iter(|| {
        let _n = bigdecimal_mul(black_box(&x), black_box(&y));
    })
}

fn bigdecimal_mul_u128(bench: &mut Bencher) {
    let x = bigdecimal_parse("170141183460469231731687303715884105727");
    let y = bigdecimal_parse("17014118346046923173168730371588410572");
    bench.iter(|| {
        let _n = bigdecimal_mul(black_box(&x), black_box(&y));
    })
}

fn bigdecimal_mul_f32(bench: &mut Bencher) {
    let x = bigdecimal_parse("1.23456789e10");
    let y = bigdecimal_parse("1.23456789e9");
    bench.iter(|| {
        let _n = bigdecimal_mul(black_box(&x), black_box(&y));
    })
}

fn bigdecimal_mul_f64(bench: &mut Bencher) {
    let x = bigdecimal_parse("1.234567890123456789e10");
    let y = bigdecimal_parse("1.234567890123456789e9");
    bench.iter(|| {
        let _n = bigdecimal_mul(black_box(&x), black_box(&y));
    })
}

#[inline(always)]
fn bigdecimal_div(x: &BigDecimal, y: &BigDecimal) -> BigDecimal {
    x / y
}

fn bigdecimal_div_u8(bench: &mut Bencher) {
    let x = bigdecimal_parse("127");
    let y = bigdecimal_parse("12");
    bench.iter(|| {
        let _n = bigdecimal_div(black_box(&x), black_box(&y));
    })
}

fn bigdecimal_div_u16(bench: &mut Bencher) {
    let x = bigdecimal_parse("32767");
    let y = bigdecimal_parse("3276");
    bench.iter(|| {
        let _n = bigdecimal_div(black_box(&x), black_box(&y));
    })
}

fn bigdecimal_div_u32(bench: &mut Bencher) {
    let x = bigdecimal_parse("2147483647");
    let y = bigdecimal_parse("214748364");
    bench.iter(|| {
        let _n = bigdecimal_div(black_box(&x), black_box(&y));
    })
}

fn bigdecimal_div_u64(bench: &mut Bencher) {
    let x = bigdecimal_parse("9223372036854775807");
    let y = bigdecimal_parse("922337203685477580");
    bench.iter(|| {
        let _n = bigdecimal_div(black_box(&x), black_box(&y));
    })
}

fn bigdecimal_div_u128(bench: &mut Bencher) {
    let x = bigdecimal_parse("170141183460469231731687303715884105727");
    let y = bigdecimal_parse("17014118346046923173168730371588410572");
    bench.iter(|| {
        let _n = bigdecimal_div(black_box(&x), black_box(&y));
    })
}

fn bigdecimal_div_f32(bench: &mut Bencher) {
    let x = bigdecimal_parse("1.23456789e10");
    let y = bigdecimal_parse("1.23456789e9");
    bench.iter(|| {
        let _n = bigdecimal_div(black_box(&x), black_box(&y));
    })
}

fn bigdecimal_div_f64(bench: &mut Bencher) {
    let x = bigdecimal_parse("1.234567890123456789e10");
    let y = bigdecimal_parse("1.234567890123456789e9");
    bench.iter(|| {
        let _n = bigdecimal_div(black_box(&x), black_box(&y));
    })
}

benchmark_group!(
    bigdecimal_parse_benches,
    bigdecimal_parse_u8,
    bigdecimal_parse_u16,
    bigdecimal_parse_u32,
    bigdecimal_parse_u64,
    bigdecimal_parse_u128,
    bigdecimal_parse_f32,
    bigdecimal_parse_f64,
);
benchmark_group!(
    bigdecimal_convert_benches,
    bigdecimal_into_u8,
    bigdecimal_into_u16,
    bigdecimal_into_u32,
    bigdecimal_into_u64,
    // bigdecimal_into_u128,
    bigdecimal_into_f32,
    bigdecimal_into_f64,
);
benchmark_group!(
    bigdecimal_from_benches,
    bigdecimal_from_u8,
    bigdecimal_from_u16,
    bigdecimal_from_u32,
    bigdecimal_from_u64,
    bigdecimal_from_u128,
    bigdecimal_from_f32,
    bigdecimal_from_f64,
);
benchmark_group!(
    bigdecimal_add_benches,
    bigdecimal_add_u8,
    bigdecimal_add_u16,
    bigdecimal_add_u32,
    bigdecimal_add_u64,
    bigdecimal_add_u128,
    bigdecimal_add_f32,
    bigdecimal_add_f64,
);
benchmark_group!(
    bigdecimal_sub_benches,
    bigdecimal_sub_u8,
    bigdecimal_sub_u16,
    bigdecimal_sub_u32,
    bigdecimal_sub_u64,
    bigdecimal_sub_u128,
    bigdecimal_sub_f32,
    bigdecimal_sub_f64,
);
benchmark_group!(
    bigdecimal_mul_benches,
    bigdecimal_mul_u8,
    bigdecimal_mul_u16,
    bigdecimal_mul_u32,
    bigdecimal_mul_u64,
    bigdecimal_mul_u128,
    bigdecimal_mul_f32,
    bigdecimal_mul_f64,
);
benchmark_group!(
    bigdecimal_div_benches,
    bigdecimal_div_u8,
    bigdecimal_div_u16,
    bigdecimal_div_u32,
    bigdecimal_div_u64,
    bigdecimal_div_u128,
    bigdecimal_div_f32,
    bigdecimal_div_f64,
);

benchmark_main!(
    bigdecimal_parse_benches,
    bigdecimal_convert_benches,
    bigdecimal_from_benches,
    bigdecimal_add_benches,
    bigdecimal_sub_benches,
    bigdecimal_mul_benches,
    bigdecimal_div_benches,
);
