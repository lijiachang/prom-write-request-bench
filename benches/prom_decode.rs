use bytes::Bytes;
use criterion::{criterion_group, criterion_main, Criterion};
use bench_prom::prom_write_request::WriteRequest;
use bench_prom::repeated_field::Clear;

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


// 火焰图

// 安装火焰图 cargo install flamegraph
// 参考文档：https://www.cnblogs.com/wmproxy/p/18253640


/*
添加调试信息：
在你的 Cargo.toml 文件中添加以下内容：
toml

[profile.bench]
debug = true

或者，你可以在运行命令前设置环境变量：

export CARGO_PROFILE_BENCH_DEBUG=true

*/


// 开始生成
// sudo cargo flamegraph --bench prom_decode

// 指定基准测试
// sudo cargo flamegraph --bench prom_decode -- pooled_write_request (好像不起作用）
//  sudo cargo flamegraph --bench prom_decode -- --bench pooled_write_request

// 生成的svg文件可以用浏览器打开，实现zoom in和zoom out