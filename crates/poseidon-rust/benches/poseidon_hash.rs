#[cfg(not(target_arch = "wasm32"))]
criterion::criterion_main!(bench::benches);

#[cfg(target_arch = "wasm32")]
fn main() {}

#[cfg(not(target_arch = "wasm32"))]
mod bench {
    use std::time::Duration;

    use ark_bn254::Fr;
    use criterion::criterion_group;
    use poseidon_rust::poseidon_hash;
    use rand::random;

    fn benchmark_poseidon(c: &mut criterion::Criterion) {
        let mut group = c.benchmark_group("Poseidon Hash");
        group
            .warm_up_time(Duration::from_secs(1))
            .measurement_time(Duration::from_secs(1));

        for t in 11..=13 {
            group.bench_function(&format!("poseidon_hash_{}", t), |b| {
                b.iter(|| {
                    let r: u128 = random();
                    let inputs = (0..t).map(|_| Fr::from(r)).collect::<Vec<_>>();
                    poseidon_hash(&inputs).unwrap();
                });
            });
        }
    }

    criterion_group!(benches, benchmark_poseidon);
}
