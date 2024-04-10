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

mod bigdecimal;
mod decimal_rs;
mod pgnumeric;
mod rust_decimal;

use crate::bigdecimal::*;
use crate::decimal_rs::*;
use crate::pgnumeric::*;
use crate::rust_decimal::*;

use bencher::{benchmark_group, benchmark_main};

benchmark_group!(
    bigdecimal_benches,
    bigdecimal_parse,
    bigdecimal_to_string,
    bigdecimal_into_f64,
    bigdecimal_from_f64,
    bigdecimal_into_u64,
    bigdecimal_cmp,
    bigdecimal_cmp2,
    bigdecimal_add,
    bigdecimal_sub,
    bigdecimal_mul,
    bigdecimal_div,
    bigdecimal_sqrt,
    bigdecimal_trunc
);

benchmark_group!(
    decimal_rs_benches,
    decimal_rs_parse,
    decimal_rs_to_string,
    decimal_rs_into_f64,
    decimal_rs_from_f64,
    decimal_rs_into_u64,
    decimal_rs_cmp,
    decimal_rs_cmp2,
    decimal_rs_add,
    decimal_rs_sub,
    decimal_rs_mul,
    decimal_rs_div,
    decimal_rs_sqrt,
    decimal_rs_trunc
);

benchmark_group!(
    pgnumeric_benches,
    pgnumeric_parse,
    pgnumeric_to_string,
    pgnumeric_into_f64,
    pgnumeric_from_f64,
    pgnumeric_into_u64,
    pgnumeric_cmp,
    pgnumeric_cmp2,
    pgnumeric_add,
    pgnumeric_sub,
    pgnumeric_mul,
    pgnumeric_div,
    pgnumeric_sqrt,
    pgnumeric_trunc
);

benchmark_group!(
    rust_decimal_benches,
    rust_decimal_parse,
    rust_decimal_to_string,
    rust_decimal_into_f64,
    rust_decimal_from_f64,
    rust_decimal_into_u64,
    rust_decimal_cmp,
    rust_decimal_cmp2,
    rust_decimal_add,
    rust_decimal_sub,
    rust_decimal_mul,
    rust_decimal_div,
    rust_decimal_sqrt,
    rust_decimal_trunc
);

benchmark_main!(
    bigdecimal_benches,
    decimal_rs_benches,
    pgnumeric_benches,
    rust_decimal_benches
);
