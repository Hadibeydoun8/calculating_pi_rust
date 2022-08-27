use calculating_pi_rust::pi_math::CalcPi;
use criterion::{criterion_group, criterion_main, Criterion};

fn calc_pi_with_write(c: &mut Criterion) {
    use std::fs::remove_dir_all;
    let ben_path = "./benchmarking";
    remove_dir_all(ben_path).unwrap_or(());
    c.bench_function("calc_pi_with_write", |b| {
        b.iter(|| {
            let mut calc_pi = CalcPi::new(0, 1000, 1, Some(ben_path));
            calc_pi.calc_pi().unwrap();
        })
    });
}

fn calc_pi_wo_write(c: &mut Criterion) {
    use std::fs::remove_dir_all;
    let ben_path = "./benchmarking";
    remove_dir_all(ben_path).unwrap_or(());
    c.bench_function("calc_pi_wo_write", |b| {
        b.iter(|| {
            let mut calc_pi = CalcPi::new(0, 1000, 1, Some(ben_path));
            calc_pi.calc_pi().unwrap();
        })
    });
}


criterion_group!(benches, calc_pi_with_write, calc_pi_wo_write);
criterion_main!(benches);