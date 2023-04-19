use criterion::{criterion_group, criterion_main, Bencher, Criterion};
use std::hint::black_box;

use eyre::{eyre, EyreHandler, Report, WrapErr};
use std::error::Error as StdError;

struct FastestEyreHandlerInTheWest;

impl EyreHandler for FastestEyreHandlerInTheWest {
    fn debug(
        &self,
        _: &(dyn StdError + 'static),
        _: &mut core::fmt::Formatter<'_>,
    ) -> core::fmt::Result {
        Ok(())
    }
}

fn spin_1000_plain() -> Result<(), Report> {
    for _ in 0..1000 {
        // the error case of course will never happen; we use black_box to pretend it might.
        black_box(Ok::<_, Report>(())).wrap_err(eyre!("error"))?;
    }

    Ok(())
}

fn spin_1000_with() -> Result<(), Report> {
    for _ in 0..1000 {
        // the error case of course will never happen; we use black_box to pretend it might.
        black_box(Ok::<_, Report>(())).wrap_err_with(|| eyre!("error"))?;
    }

    Ok(())
}

fn plain_bench(b: &mut Bencher) {
    b.iter(|| {
        // make eyre!'s env a contended read
        let jh = std::thread::spawn(move || {
            black_box(spin_1000_plain()).unwrap();
        });

        black_box(spin_1000_plain()).unwrap();
        jh.join().unwrap();
    })
}

fn with_bench(b: &mut Bencher) {
    b.iter(|| {
        // make eyre!'s env a contended read
        let jh = std::thread::spawn(move || {
            black_box(spin_1000_with()).unwrap();
        });

        black_box(spin_1000_with()).unwrap();
        jh.join().unwrap();
    })
}

fn criterion_benchmark(c: &mut Criterion) {
    eyre::set_hook(Box::new(|_| Box::new(FastestEyreHandlerInTheWest))).unwrap();
    let mut wrap_err_group = c.benchmark_group("WrapErr");
    wrap_err_group.bench_function("fastest:wrap_err(eyre!(...))", plain_bench);
    wrap_err_group.bench_function("fastest:wrap_err_with(|| eyre!(...))", with_bench);
    wrap_err_group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
