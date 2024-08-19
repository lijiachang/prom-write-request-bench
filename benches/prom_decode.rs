use bytes::Bytes;
use criterion::{criterion_group, criterion_main, Criterion};
use greptime_proto::prometheus::remote::WriteRequest;
use prost::Message;

fn bench_decode_prom_request(c: &mut Criterion) {
    let mut d = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    d.push("assets");
    d.push("1709380533560664458.data");
    let data = Bytes::from(std::fs::read(d).unwrap());
    let mut request_pooled = WriteRequest::default();
    c.benchmark_group("decode")
        .bench_function("write_request", |b| {
            b.iter(|| {
                let mut request = WriteRequest::default();
                let data = data.clone();
                request.merge(data).unwrap();
            });
        })
        .bench_function("pooled_write_request", |b| {
            b.iter(|| {
                request_pooled.clear();
                let data = data.clone();
                request_pooled.merge(data).unwrap();
            });
        });
}

criterion_group!(benches, bench_decode_prom_request);
criterion_main!(benches);

// 运行 bench 命令，得到基线结果
// 未池化版本： cargo bench -- decode/write_request          测试结果是time:   [7.0951 ms 7.1130 ms 7.1330 ms]
// 池化版本：  cargo bench -- decode/pooled_write_request    测试结果是 time:   [7.1456 ms 7.1790 ms 7.2198 ms]
