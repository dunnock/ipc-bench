use criterion::{criterion_group, criterion_main, Criterion, Throughput, BenchmarkId};
use ipc_channel::ipc::{self, IpcOneShotServer, IpcBytesSender, IpcBytesReceiver, IpcSender};
use ipc_bench::process::{fork, Wait, Kill, Pid};

type Message = Vec<u8>;

fn fork_receiver() -> (Pid, IpcBytesSender) {
    let (server, server_name) = IpcOneShotServer::new().unwrap();
    let pid = unsafe { fork(|| {
        let (tx, rx) = ipc::bytes_channel().unwrap();
        let txs = IpcSender::connect(server_name).unwrap();
        txs.send(tx).unwrap();
        loop {
            receive(&rx)
        }
    }) };
    let (_, tx) = server.accept().unwrap();
    (pid, tx)
}

fn fork_sender(msg: Message) -> (Pid, IpcBytesReceiver) {
    let (server, server_name) = IpcOneShotServer::new().unwrap();
    let pid = unsafe { fork(|| {
        let (tx, rx) = ipc::bytes_channel().unwrap();
        let txs = IpcSender::connect(server_name).unwrap();
        txs.send(rx).unwrap();
        loop {
            send(&tx, &msg)
        }
    }) };
    let (_, tx) = server.accept().unwrap();
    (pid, tx)
}

fn send(tx: &IpcBytesSender, msg: &Message) {
    tx.send(msg).unwrap()
}

fn receive(rx: &IpcBytesReceiver) {
    let msg = rx.recv().unwrap();
    if msg[0] == b'F' {
        std::process::exit(0);
    }
}

pub fn bytes_sends(c: &mut Criterion) {
    static KB: usize = 1024;
    let mut group = c.benchmark_group("bytes_sends");
    for size in [KB, 10 * KB, 50 * KB, 100 * KB].iter() {
        group.throughput(Throughput::Bytes(*size as u64));
        // prepare data
        let mut msg: Vec<u8> = (0..*size).into_iter().map(|i| (i%255) as u8).collect();
        // start receiving process in parallel
        let (pid, tx) = fork_receiver();
        // benchmark
        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, _| {
            b.iter(|| send(&tx, &msg));
        });
        // shutdown receiver
        msg[0] = b'F';
        send(&tx, &msg);
        pid.wait()
    }
    group.finish();
}

pub fn bytes_sends_cloned(c: &mut Criterion) {
    static KB: usize = 1024;
    let mut group = c.benchmark_group("bytes_sends_cloned");
    for size in [KB, 10 * KB, 50 * KB, 100 * KB].iter() {
        group.throughput(Throughput::Bytes(*size as u64));
        // prepare data
        let mut msg: Vec<u8> = (0..*size).into_iter().map(|i| (i%255) as u8).collect();
        // start receiving process in parallel
        let (pid, tx) = fork_receiver();
        // benchmark
        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, _| {
            b.iter(|| { let msg = msg.clone(); send(&tx, &msg) });
        });
        // shutdown receiver
        msg[0] = b'F';
        send(&tx, &msg);
        pid.wait()
    }
    group.finish();
}

pub fn bytes_receives(c: &mut Criterion) {
    static KB: usize = 1024;
    let mut group = c.benchmark_group("bytes_receives");
    for size in [KB, 10 * KB, 50 * KB, 100 * KB].iter() {
        group.throughput(Throughput::Bytes(*size as u64));
        // prepare data
        let msg: Vec<u8> = (0..*size).into_iter().map(|i| (i%255) as u8).collect();
        // start receiving process in parallel
        let (pid, rx) = fork_sender(msg);
        // benchmark
        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, _| {
            b.iter(|| receive(&rx));
        });
        // kill sender
        pid.kill()
    }
    group.finish();
}


criterion_group!(benches_bytes_sends, bytes_sends);
criterion_group!(benches_bytes_sends_cloned, bytes_sends_cloned);
criterion_group!(benches_bytes_receives, bytes_receives);
criterion_main!(benches_bytes_sends, benches_bytes_sends_cloned, benches_bytes_receives);
