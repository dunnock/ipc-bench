use criterion::{criterion_group, criterion_main, Criterion, Throughput, BenchmarkId};
use bincode::{serialize, deserialize};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone)]
struct Message {
	//pub topic: String,
	pub topic: u32,
        #[serde(with = "serde_bytes")]
	pub data: Vec<u8>
}

pub fn bincode_encode(c: &mut Criterion) {
    static KB: usize = 1024;
    let mut group = c.benchmark_group("bincode_encode");
    for size in [KB, 10 * KB, 50 * KB, 100 * KB].iter() {
        group.throughput(Throughput::Bytes(*size as u64));
        // prepare data
        let data = (0..*size).into_iter().map(|i| (i%255) as u8).collect();
        let msg = Message { topic: 0, data };
        // benchmark
        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, _| {
            b.iter(|| serialize(&msg).unwrap());
        });
    }
    group.finish();
}

pub fn bincode_decode(c: &mut Criterion) {
    static KB: usize = 1024;
    let mut group = c.benchmark_group("bincode_decode");
    for size in [KB, 10 * KB, 50 * KB, 100 * KB].iter() {
        group.throughput(Throughput::Bytes(*size as u64));
        // prepare data
        let data = (0..*size).into_iter().map(|i| (i%255) as u8).collect();
        let msg = Message { topic: 0, data };
        let msg_ser = serialize(&msg).unwrap();
        // benchmark
        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, _| {
            b.iter(|| deserialize::<Message>(&msg_ser[..]).unwrap());
        });
    }
    group.finish();
}


criterion_group!(benches_encode, bincode_encode);
criterion_group!(benches_decode, bincode_decode);
criterion_main!(benches_encode, benches_decode);
