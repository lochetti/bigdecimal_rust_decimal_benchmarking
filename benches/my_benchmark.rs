use std::str::FromStr;

use bigdecimal::{BigDecimal, Zero};
use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use rust_decimal::Decimal;

const PRICES: [&'static str; 10] = [
    "28.95", "13.958", "9.11", "18.44", "19.50", "18.50", "34.751", "19.91", "9.33", "14.501",
];

const QUANTITIES: [i32; 10] = [2, 10, 9, 3, 50, 18, 22, 19, 41, 30];

fn prepare_bigdecimal() -> Vec<BigDecimalTrade> {
    let mut trades = Vec::new();
    for i in 0..10 {
        trades.push(BigDecimalTrade {
            price: BigDecimal::from_str(PRICES[i]).unwrap(),
            quantity: QUANTITIES[i],
        })
    }
    trades
}

fn prepare_rust_decimal() -> Vec<DecimalTrade> {
    let mut trades = Vec::new();
    for i in 0..10 {
        trades.push(DecimalTrade {
            price: Decimal::from_str(PRICES[i]).unwrap(),
            quantity: QUANTITIES[i],
        })
    }
    trades
}

struct BigDecimalTrade {
    price: BigDecimal,
    quantity: i32,
}

struct DecimalTrade {
    price: Decimal,
    quantity: i32,
}

fn mul_sum_bigdecimal(trades: &Vec<BigDecimalTrade>) -> BigDecimal {
    let mut result = BigDecimal::zero();
    for trade in trades {
        result += &result + (&trade.price * trade.quantity);
    }

    result
}

fn mul_sum_rust_decimal(trades: &Vec<DecimalTrade>) -> Decimal {
    let mut result = Decimal::zero();
    for trade in trades {
        result += result + (trade.price.saturating_mul(trade.quantity.into()));
    }

    result
}

fn bench_sum(c: &mut Criterion) {
    let mut group = c.benchmark_group("Sum Trades");
    let bigdecimals = prepare_bigdecimal();
    let decimals = prepare_rust_decimal();
    group.bench_with_input(BenchmarkId::new("BigDecimal", "."), &bigdecimals, |b, i| {
        b.iter(|| mul_sum_bigdecimal(black_box(i)))
    });
    group.bench_with_input(BenchmarkId::new("Decimals", "."), &decimals, |b, i| {
        b.iter(|| mul_sum_rust_decimal(black_box(i)))
    });

    group.finish();
}

criterion_group!(benches, bench_sum);
criterion_main!(benches);
