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

use bencher::{benchmark_group, benchmark_main, black_box, Bencher};
use num_traits::{FromPrimitive, ToPrimitive};
use rust_decimal::Decimal;

#[inline(always)]
fn rust_decimal_parse(s: &str) -> Decimal {
    s.parse().unwrap()
}

fn rust_decimal_parse_u8(bench: &mut Bencher) {
    bench.iter(|| {
        let _n = rust_decimal_parse(black_box("255"));
    })
}

fn rust_decimal_parse_u16(bench: &mut Bencher) {
    bench.iter(|| {
        let _n = rust_decimal_parse(black_box("65535"));
    })
}

fn rust_decimal_parse_u32(bench: &mut Bencher) {
    bench.iter(|| {
        let _n = rust_decimal_parse(black_box("4294967295"));
    })
}

fn rust_decimal_parse_u64(bench: &mut Bencher) {
    bench.iter(|| {
        let _n = rust_decimal_parse(black_box("18446744073709551615"));
    })
}

fn rust_decimal_parse_u128(bench: &mut Bencher) {
    bench.iter(|| {
        let _n = rust_decimal_parse(black_box("79228162514264337593543950335"));
    })
}

fn rust_decimal_parse_f32(bench: &mut Bencher) {
    bench.iter(|| {
        let _n = rust_decimal_parse(black_box("12345678900"));
    })
}

fn rust_decimal_parse_f64(bench: &mut Bencher) {
    bench.iter(|| {
        let _n = rust_decimal_parse(black_box("12345678901.23456789"));
    })
}

fn rust_decimal_into_u8(bench: &mut Bencher) {
    let val = rust_decimal_parse("255");
    bench.iter(|| {
        let _n: u8 = black_box(&val).to_u8().unwrap();
    })
}

fn rust_decimal_into_u16(bench: &mut Bencher) {
    let val = rust_decimal_parse("65535");
    bench.iter(|| {
        let _n: u16 = black_box(&val).to_u16().unwrap();
    })
}

fn rust_decimal_into_u32(bench: &mut Bencher) {
    let val = rust_decimal_parse("4294967295");
    bench.iter(|| {
        let _n: u32 = black_box(&val).to_u32().unwrap();
    })
}

fn rust_decimal_into_u64(bench: &mut Bencher) {
    let val = rust_decimal_parse("18446744073709551615");
    bench.iter(|| {
        let _n: u64 = black_box(&val).to_u64().unwrap();
    })
}

fn rust_decimal_into_u128(bench: &mut Bencher) {
    let val = rust_decimal_parse("79228162514264337593543950335");
    bench.iter(|| {
        let _n: u128 = black_box(&val).to_u128().unwrap();
    })
}

fn rust_decimal_into_f32(bench: &mut Bencher) {
    let val = rust_decimal_parse("12345678900");
    bench.iter(|| {
        let _n: f32 = black_box(&val).to_f32().unwrap();
    })
}

fn rust_decimal_into_f64(bench: &mut Bencher) {
    let val = rust_decimal_parse("12345678901.23456789");
    bench.iter(|| {
        let _n: f64 = black_box(&val).to_f64().unwrap();
    })
}

#[inline(always)]
fn rust_decimal_from<T: Into<Decimal>>(val: T) -> Decimal {
    val.into()
}

fn rust_decimal_from_u8(bench: &mut Bencher) {
    bench.iter(|| {
        let _n = rust_decimal_from(black_box(255_u8));
    })
}

fn rust_decimal_from_u16(bench: &mut Bencher) {
    bench.iter(|| {
        let _n = rust_decimal_from(black_box(65535_u16));
    })
}

fn rust_decimal_from_u32(bench: &mut Bencher) {
    bench.iter(|| {
        let _n = rust_decimal_from(black_box(4294967295_u32));
    })
}

fn rust_decimal_from_u64(bench: &mut Bencher) {
    bench.iter(|| {
        let _n = rust_decimal_from(black_box(18446744073709551615_u64));
    })
}

fn rust_decimal_from_u128(bench: &mut Bencher) {
    bench.iter(|| {
        let _n = Decimal::from_u128(black_box(18446744073709551615_u128)).unwrap();
    })
}

fn rust_decimal_from_f32(bench: &mut Bencher) {
    bench.iter(|| {
        let _n = Decimal::from_f32(black_box(1.23456789e10_f32)).unwrap();
    })
}

fn rust_decimal_from_f64(bench: &mut Bencher) {
    bench.iter(|| {
        let _n = Decimal::from_f64(black_box(1.234567890123456789e10_f64)).unwrap();
    })
}

#[inline(always)]
fn rust_decimal_add(x: &Decimal, y: &Decimal) -> Decimal {
    x + y
}

fn rust_decimal_add_u8(bench: &mut Bencher) {
    let x = rust_decimal_parse("127");
    bench.iter(|| {
        let _n = rust_decimal_add(black_box(&x), black_box(&x));
    })
}

fn rust_decimal_add_u16(bench: &mut Bencher) {
    let x = rust_decimal_parse("32767");
    bench.iter(|| {
        let _n = rust_decimal_add(black_box(&x), black_box(&x));
    })
}

fn rust_decimal_add_u32(bench: &mut Bencher) {
    let x = rust_decimal_parse("2147483647");
    bench.iter(|| {
        let _n = rust_decimal_add(black_box(&x), black_box(&x));
    })
}

fn rust_decimal_add_u64(bench: &mut Bencher) {
    let x = rust_decimal_parse("9223372036854775807");
    bench.iter(|| {
        let _n = rust_decimal_add(black_box(&x), black_box(&x));
    })
}

fn rust_decimal_add_u128(bench: &mut Bencher) {
    let x = rust_decimal_parse("18446744073709551615");
    bench.iter(|| {
        let _n = rust_decimal_add(black_box(&x), black_box(&x));
    })
}

fn rust_decimal_add_f32(bench: &mut Bencher) {
    let x = rust_decimal_parse("12345678900");
    bench.iter(|| {
        let _n = rust_decimal_add(black_box(&x), black_box(&x));
    })
}

fn rust_decimal_add_f64(bench: &mut Bencher) {
    let x = rust_decimal_parse("12345678901.23456789");
    bench.iter(|| {
        let _n = rust_decimal_add(black_box(&x), black_box(&x));
    })
}

#[inline(always)]
fn rust_decimal_sub(x: &Decimal, y: &Decimal) -> Decimal {
    x - y
}

fn rust_decimal_sub_u8(bench: &mut Bencher) {
    let x = rust_decimal_parse("127");
    let y = rust_decimal_parse("12");
    bench.iter(|| {
        let _n = rust_decimal_sub(black_box(&x), black_box(&y));
    })
}

fn rust_decimal_sub_u16(bench: &mut Bencher) {
    let x = rust_decimal_parse("32767");
    let y = rust_decimal_parse("3276");
    bench.iter(|| {
        let _n = rust_decimal_sub(black_box(&x), black_box(&y));
    })
}

fn rust_decimal_sub_u32(bench: &mut Bencher) {
    let x = rust_decimal_parse("2147483647");
    let y = rust_decimal_parse("214748364");
    bench.iter(|| {
        let _n = rust_decimal_sub(black_box(&x), black_box(&y));
    })
}

fn rust_decimal_sub_u64(bench: &mut Bencher) {
    let x = rust_decimal_parse("9223372036854775807");
    let y = rust_decimal_parse("922337203685477580");
    bench.iter(|| {
        let _n = rust_decimal_sub(black_box(&x), black_box(&y));
    })
}

fn rust_decimal_sub_u128(bench: &mut Bencher) {
    let x = rust_decimal_parse("79228162514264337593543950335");
    let y = rust_decimal_parse("7922816251426433759354395033");
    bench.iter(|| {
        let _n = rust_decimal_sub(black_box(&x), black_box(&y));
    })
}

fn rust_decimal_sub_f32(bench: &mut Bencher) {
    let x = rust_decimal_parse("12345678900");
    let y = rust_decimal_parse("1234567890");
    bench.iter(|| {
        let _n = rust_decimal_sub(black_box(&x), black_box(&y));
    })
}

fn rust_decimal_sub_f64(bench: &mut Bencher) {
    let x = rust_decimal_parse("12345678901.23456789");
    let y = rust_decimal_parse("1234567890.123456789");
    bench.iter(|| {
        let _n = rust_decimal_sub(black_box(&x), black_box(&y));
    })
}

#[inline(always)]
fn rust_decimal_mul(x: &Decimal, y: &Decimal) -> Decimal {
    x * y
}

fn rust_decimal_mul_u8(bench: &mut Bencher) {
    let x = rust_decimal_parse("127");
    let y = rust_decimal_parse("12");
    bench.iter(|| {
        let _n = rust_decimal_mul(black_box(&x), black_box(&y));
    })
}

fn rust_decimal_mul_u16(bench: &mut Bencher) {
    let x = rust_decimal_parse("32767");
    let y = rust_decimal_parse("3276");
    bench.iter(|| {
        let _n = rust_decimal_mul(black_box(&x), black_box(&y));
    })
}

fn rust_decimal_mul_u32(bench: &mut Bencher) {
    let x = rust_decimal_parse("2147483647");
    let y = rust_decimal_parse("214748364");
    bench.iter(|| {
        let _n = rust_decimal_mul(black_box(&x), black_box(&y));
    })
}

// fn rust_decimal_mul_u64(bench: &mut Bencher) {
//     let x = rust_decimal_parse("9223372036854775807");
//     let y = rust_decimal_parse("922337203685477580");
//     bench.iter(|| {
//         let _n = rust_decimal_mul(black_box(&x), black_box(&y));
//     })
// }

// fn rust_decimal_mul_u128(bench: &mut Bencher) {
//     let x = rust_decimal_parse("18446744073709551615");
//     let y = rust_decimal_parse("1844674407370955161");
//     bench.iter(|| {
//         let _n = rust_decimal_mul(black_box(&x), black_box(&y));
//     })
// }

fn rust_decimal_mul_f32(bench: &mut Bencher) {
    let x = rust_decimal_parse("12345678900");
    let y = rust_decimal_parse("1234567890");
    bench.iter(|| {
        let _n = rust_decimal_mul(black_box(&x), black_box(&y));
    })
}

fn rust_decimal_mul_f64(bench: &mut Bencher) {
    let x = rust_decimal_parse("12345678901.23456789");
    let y = rust_decimal_parse("1234567890.123456789");
    bench.iter(|| {
        let _n = rust_decimal_mul(black_box(&x), black_box(&y));
    })
}

#[inline(always)]
fn rust_decimal_div(x: &Decimal, y: &Decimal) -> Decimal {
    x / y
}

fn rust_decimal_div_u8(bench: &mut Bencher) {
    let x = rust_decimal_parse("127");
    let y = rust_decimal_parse("12");
    bench.iter(|| {
        let _n = rust_decimal_div(black_box(&x), black_box(&y));
    })
}

fn rust_decimal_div_u16(bench: &mut Bencher) {
    let x = rust_decimal_parse("32767");
    let y = rust_decimal_parse("3276");
    bench.iter(|| {
        let _n = rust_decimal_div(black_box(&x), black_box(&y));
    })
}

fn rust_decimal_div_u32(bench: &mut Bencher) {
    let x = rust_decimal_parse("2147483647");
    let y = rust_decimal_parse("214748364");
    bench.iter(|| {
        let _n = rust_decimal_div(black_box(&x), black_box(&y));
    })
}

fn rust_decimal_div_u64(bench: &mut Bencher) {
    let x = rust_decimal_parse("9223372036854775807");
    let y = rust_decimal_parse("922337203685477580");
    bench.iter(|| {
        let _n = rust_decimal_div(black_box(&x), black_box(&y));
    })
}

fn rust_decimal_div_u128(bench: &mut Bencher) {
    let x = rust_decimal_parse("79228162514264337593543950335");
    let y = rust_decimal_parse("7922816251426433759354395033");
    bench.iter(|| {
        let _n = rust_decimal_div(black_box(&x), black_box(&y));
    })
}

fn rust_decimal_div_f32(bench: &mut Bencher) {
    let x = rust_decimal_parse("12345678900");
    let y = rust_decimal_parse("1234567890");
    bench.iter(|| {
        let _n = rust_decimal_div(black_box(&x), black_box(&y));
    })
}

fn rust_decimal_div_f64(bench: &mut Bencher) {
    let x = rust_decimal_parse("12345678901.23456789");
    let y = rust_decimal_parse("1234567890.123456789");
    bench.iter(|| {
        let _n = rust_decimal_div(black_box(&x), black_box(&y));
    })
}

benchmark_group!(
    rust_decimal_parse_benches,
    rust_decimal_parse_u8,
    rust_decimal_parse_u16,
    rust_decimal_parse_u32,
    rust_decimal_parse_u64,
    rust_decimal_parse_u128,
    rust_decimal_parse_f32,
    rust_decimal_parse_f64,
);
benchmark_group!(
    rust_decimal_convert_benches,
    rust_decimal_into_u8,
    rust_decimal_into_u16,
    rust_decimal_into_u32,
    rust_decimal_into_u64,
    rust_decimal_into_u128,
    rust_decimal_into_f32,
    rust_decimal_into_f64,
);
benchmark_group!(
    rust_decimal_from_benches,
    rust_decimal_from_u8,
    rust_decimal_from_u16,
    rust_decimal_from_u32,
    rust_decimal_from_u64,
    rust_decimal_from_u128,
    rust_decimal_from_f32,
    rust_decimal_from_f64,
);
benchmark_group!(
    rust_decimal_add_benches,
    rust_decimal_add_u8,
    rust_decimal_add_u16,
    rust_decimal_add_u32,
    rust_decimal_add_u64,
    rust_decimal_add_u128,
    rust_decimal_add_f32,
    rust_decimal_add_f64,
);
benchmark_group!(
    rust_decimal_sub_benches,
    rust_decimal_sub_u8,
    rust_decimal_sub_u16,
    rust_decimal_sub_u32,
    rust_decimal_sub_u64,
    rust_decimal_sub_u128,
    rust_decimal_sub_f32,
    rust_decimal_sub_f64,
);
benchmark_group!(
    rust_decimal_mul_benches,
    rust_decimal_mul_u8,
    rust_decimal_mul_u16,
    rust_decimal_mul_u32,
    // rust_decimal_mul_u64,
    // rust_decimal_mul_u128,
    rust_decimal_mul_f32,
    rust_decimal_mul_f64,
);
benchmark_group!(
    rust_decimal_div_benches,
    rust_decimal_div_u8,
    rust_decimal_div_u16,
    rust_decimal_div_u32,
    rust_decimal_div_u64,
    rust_decimal_div_u128,
    rust_decimal_div_f32,
    rust_decimal_div_f64,
);

benchmark_main!(
    rust_decimal_parse_benches,
    rust_decimal_convert_benches,
    rust_decimal_from_benches,
    rust_decimal_add_benches,
    rust_decimal_sub_benches,
    rust_decimal_mul_benches,
    rust_decimal_div_benches,
);
