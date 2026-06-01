#[cfg(bench)]
criterion::criterion_main!(bench::benches);

#[cfg(not(bench))]
fn main() {}

#[cfg(bench)]
pub mod bench {
    use std::hint::black_box;

    use criterion::{Criterion, criterion_group};
    use railgun::bench_helpers::{
        groth16_prover::Groth16Prover, inputs::transact_inputs::TransactCircuitInputs,
    };

    const TRANSACT_DATA: &[u8] = include_bytes!("./fixtures/transact_inputs.json");

    pub fn bench_prove_transact(c: &mut Criterion) {
        let transact_inputs: TransactCircuitInputs = serde_json::from_slice(TRANSACT_DATA).unwrap();
        let prover = Groth16Prover::new();

        let mut group = c.benchmark_group("Prove Transact");
        group.sample_size(10);

        group.bench_function("prove_transact", |b| {
            b.to_async(
                tokio::runtime::Builder::new_current_thread()
                    .enable_all()
                    .build()
                    .unwrap(),
            )
            .iter(async || {
                prover
                    .prove_transact(black_box(&transact_inputs))
                    .await
                    .unwrap();
            })
        });
    }

    criterion_group!(benches, bench_prove_transact);
}
