// Custom tailored serialization of message

use criterion::{criterion_group, criterion_main, Criterion, Throughput, BenchmarkId};
use ipc_channel::ipc::{self, IpcOneShotServer, IpcBytesSender, IpcBytesReceiver, IpcSender, IpcReceiver};
//use ipc_bench::channel::{Channel, Sender, Receiver};
use ipc_bench::process::{fork, Wait, Kill, Pid};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone)]
struct Message(Vec<u8>);
struct MsgTuple(pub u32, pub Vec<u8>);

trait MessageData: Clone + Into<Vec<u8>> + From<MsgTuple> + for<'de> Deserialize<'de> + Serialize  {
    fn get_topic(&self) -> u32;
    fn set_topic(&mut self, topic: u32);
    fn set_data(&mut self, data: &[u8]);
    fn extract_data(self) -> Vec<u8>;
}
impl MessageData for Message {
    fn get_topic(&self) -> u32 {
        self.0.get_topic()
    }
    fn set_topic(&mut self, topic: u32) {
        self.0.set_topic(topic);
    }
    fn set_data(&mut self, data: &[u8]) {
        // maybe not the best performant version
        self.0.set_data(data);
    }
    fn extract_data(self) -> Vec<u8> {
        self.0.extract_data()
    }
}
impl MessageData for Vec<u8> {
    fn get_topic(&self) -> u32 {
        unsafe { *(self.as_ptr() as *const u32) }
    }
    fn set_topic(&mut self, topic: u32) {
        // not portable version
        unsafe { *(self.as_mut_ptr() as *mut u32) = topic };
    }
    fn set_data(&mut self, data: &[u8]) {
        // maybe not the best performant version
        self.extend_from_slice(data);
    }
    fn extract_data(self) -> Vec<u8> {
        self[4..].to_vec()
    }
}
impl From<MsgTuple> for Vec<u8> {
    fn from(orig: MsgTuple) -> Self {
        let mut s = Vec::with_capacity(orig.1.len()+4);
        s.set_topic(orig.0);
        s.set_data(&orig.1);
        s
    }
}
impl From<MsgTuple> for Message {
    fn from(orig: MsgTuple) -> Self {
        Self(orig.into())
    }
}
impl Into<Vec<u8>> for Message {
    fn into(self) -> Vec<u8> {
        self.0
    }
}


// types boilerplace to abstract of Sender and Receiver
//trait SerDe: for<'de> Deserialize<'de> + Serialize {}
//impl<T> SerDe for T where T: for<'de> Deserialize<'de> + Serialize {}
trait ChanFn<IS,IR>: Fn()-> Result<(IS,IR),std::io::Error> {}
impl<F,IS,IR> ChanFn<IS,IR> for F where F: Fn()-> Result<(IS,IR),std::io::Error> {}
trait Sender<T>: for<'de> Deserialize<'de> + Serialize {
    fn send_msg(&self, data: T) -> Result<(),bincode::Error>;
}
trait Receiver<T>: for<'de> Deserialize<'de> + Serialize {
    fn recv_msg(&self) -> Result<T,bincode::Error>;
}
impl Sender<Vec<u8>> for IpcBytesSender {
    #[inline]
    fn send_msg(&self, msg: Vec<u8>) -> Result<(),bincode::Error> {
        self.send(&msg[..]).map_err(|err| err.into())
    }
}
impl<T> Sender<T> for IpcSender<T> where T: MessageData {
    #[inline]
    fn send_msg(&self, msg: T) -> Result<(),bincode::Error> {
        self.send(msg)
    }
}
impl Receiver<Vec<u8>> for IpcBytesReceiver {
    #[inline]
    fn recv_msg(&self) -> Result<Vec<u8>,bincode::Error> {
        self.recv()
    }
}
impl<T> Receiver<T> for IpcReceiver<T> where T: MessageData {
    #[inline]
    fn recv_msg(&self) -> Result<T,bincode::Error> {
        self.recv()
    }
}


fn fork_receiver<TS, TR, IS, IR>(channel: impl ChanFn<IS,IR>) -> (Pid, IS)
    where TS: MessageData, TR: MessageData, IS: Sender<TS>, IR: Receiver<TR> 
{
    let (server, server_name) = IpcOneShotServer::new().unwrap();
    let pid = unsafe { fork(|| {
        let (tx, rx) = channel().unwrap();
        let txs = IpcSender::connect(server_name).unwrap();
        txs.send(tx).unwrap();
        loop {
            receive(&rx)
        }
    }) };
    let (_, tx) = server.accept().unwrap();
    (pid, tx)
}

fn fork_sender<TS, TR, IS, IR>(msg: TS, channel: impl ChanFn<IS,IR>) -> (Pid, IR)
    where TS: MessageData, TR: MessageData, IS: Sender<TS>, IR: Receiver<TR> 
{
    let (server, server_name) = IpcOneShotServer::new().unwrap();
    let pid = unsafe { fork(|| {
        let (tx, rx) = channel().unwrap();
        let txs = IpcSender::connect(server_name).unwrap();
        txs.send(rx).unwrap();
        loop {
            send(&tx, msg.clone())
        }
    }) };
    let (_, rx) = server.accept().unwrap();
    (pid, rx)
}

fn send<T: MessageData>(tx: &impl Sender<T>, msg: T) {
    tx.send_msg(msg).unwrap()
}

fn receive<T: MessageData>(rx: &impl Receiver<T>) {
    let msg = rx.recv_msg().unwrap();
    if msg.get_topic() == 1 {
        std::process::exit(0);
    }
}

fn sends_custom_template<TS,TR,IS,IR>(c: &mut Criterion, channel: impl ChanFn<IS,IR>, group: &str)
    where TS: MessageData, TR: MessageData,  IS: Sender<TS>, IR: Receiver<TR> 
{
    static KB: usize = 1024;
    let mut group = c.benchmark_group(group);
    for size in [KB, 10 * KB, 50 * KB, 100 * KB].iter() {
        group.throughput(Throughput::Bytes(*size as u64));
        // prepare data
        let data = (0..*size).into_iter().map(|i| (i%255) as u8).collect();
        let mut msg = TS::from(MsgTuple(0u32, data));
        // start receiving process in parallel
        let (pid, tx) = fork_receiver(&channel);
        // benchmark
        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, _| {
            b.iter(|| send(&tx, msg.clone()));
        });
        // shutdown receiver
        msg.set_topic(1);
        send(&tx, msg);
        pid.wait()
    }
    group.finish();
}

fn receives_custom_template<TS,TR,IS,IR>(c: &mut Criterion, channel: impl ChanFn<IS,IR>, group: &str) 
    where TS: MessageData, TR: MessageData,  IS: Sender<TS>, IR: Receiver<TR> 
{
    static KB: usize = 1024;
    let mut group = c.benchmark_group(group);
    for size in [KB, 10 * KB, 50 * KB, 100 * KB].iter() {
        group.throughput(Throughput::Bytes(*size as u64));
        // prepare data
        let data = (0..*size).into_iter().map(|i| (i%255) as u8).collect();
        let msg = TS::from(MsgTuple(0u32, data));
        // start receiving process in parallel
        let (pid, rx) = fork_sender(msg, &channel);
        // benchmark
        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, _| {
            b.iter(|| receive(&rx));
        });
        // kill sender
        pid.kill()
    }
    group.finish();
}

pub fn sends_custom(c: &mut Criterion) {
    sends_custom_template(c, ipc::channel::<Message>, "sends_custom")
}
pub fn sends_custom_bytes(c: &mut Criterion) {
    sends_custom_template(c, ipc::bytes_channel, "sends_custom_bytes")
}
pub fn receives_custom(c: &mut Criterion) {
    receives_custom_template(c, ipc::channel::<Message>, "receives_custom")
}
pub fn receives_custom_bytes(c: &mut Criterion) {
    receives_custom_template(c, ipc::bytes_channel, "receives_custom_bytes")
}


criterion_group!(benches_sends_custom, sends_custom);
criterion_group!(benches_receives_custom, receives_custom);
criterion_group!(benches_sends_custom_bytes, sends_custom_bytes);
criterion_group!(benches_receives_custom_bytes, receives_custom_bytes);
criterion_main!(benches_sends_custom, benches_receives_custom, benches_sends_custom_bytes, benches_receives_custom_bytes);
