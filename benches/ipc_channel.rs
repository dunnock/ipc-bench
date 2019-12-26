use criterion::{criterion_group, criterion_main, Criterion, Throughput, BenchmarkId};
use ipc_channel::ipc::{IpcOneShotServer, IpcSender};
use ipc_bench::channel::{Message, Channel, Sender, Receiver};
use ipc_bench::process::{fork, Wait, Kill, Pid};

fn fork_receiver() -> (Pid, Sender) {
    let (server, server_name) = IpcOneShotServer::new().unwrap();
    let pid = unsafe { fork(|| {
        let (tx, rx) = Channel::simplex().unwrap().split().unwrap();
        let txs = IpcSender::connect(server_name).unwrap();
        txs.send(tx).unwrap();
        loop {
            receive(&rx)
        }
    }) };
    let (_, tx) = server.accept().unwrap();
    (pid, tx)
}

fn fork_sender(msg: Message) -> (Pid, Receiver) {
    let (server, server_name) = IpcOneShotServer::new().unwrap();
    let pid = unsafe { fork(|| {
        let (tx, rx) = Channel::simplex().unwrap().split().unwrap();
        let txs = IpcSender::connect(server_name).unwrap();
        txs.send(rx).unwrap();
        loop {
            send(&tx, &msg)
        }
    }) };
    let (_, tx) = server.accept().unwrap();
    (pid, tx)
}

fn send(tx: &Sender, msg: &Message) {
    tx.send(msg.clone()).unwrap()
}

fn receive(rx: &Receiver) {
    let msg = rx.recv().unwrap();
    if msg.data[0] == b'F' {
        std::process::exit(0);
    }
}

pub fn sends(c: &mut Criterion) {
    static KB: usize = 1024;
    let mut group = c.benchmark_group("sends");
    for size in [KB, 10 * KB, 50 * KB, 100 * KB].iter() {
        group.throughput(Throughput::Bytes(*size as u64));
        // prepare data
        let data = (0..*size).into_iter().map(|i| (i%255) as u8).collect();
        let mut msg = Message { topic: 0, data };
        // start receiving process in parallel
        let (pid, tx) = fork_receiver();
        // benchmark
        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, _| {
            b.iter(|| send(&tx, &msg));
        });
        // shutdown receiver
        msg.data[0] = b'F';
        send(&tx, &msg);
        pid.wait()
    }
    group.finish();
}

pub fn receives(c: &mut Criterion) {
    static KB: usize = 1024;
    let mut group = c.benchmark_group("receives");
    for size in [KB, 10 * KB, 50 * KB, 100 * KB].iter() {
        group.throughput(Throughput::Bytes(*size as u64));
        // prepare data
        let data = (0..*size).into_iter().map(|i| (i%255) as u8).collect();
        let msg = Message { topic: 0, data };
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


criterion_group!(benches_sends, sends);
criterion_group!(benches_receives, receives);
criterion_main!(benches_sends, benches_receives);
