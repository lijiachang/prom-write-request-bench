use bytes::Bytes;  // 用于高效处理二进制数据
use criterion::{criterion_group, criterion_main, Criterion};
use greptime_proto::prometheus::remote::WriteRequest;  //一个 Protocol Buffers 生成的结构
use prost::Message; //用于处理 Protocol Buffers 消息

fn bench_decode_prom_request(c: &mut Criterion) {

    // 构建测试数据文件的路径
    let mut d = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    d.push("assets");
    d.push("1709380533560664458.data");
    // 读取文件内容并转换为 Bytes 类型
    let data = Bytes::from(std::fs::read(d).unwrap());

    let mut request_pooled = WriteRequest::default();

    c.benchmark_group("decode") //设置基准测试组
        .bench_function("write_request", |b| {  //第一个基准测试：非池化版本. 测试每次创建新的 WriteRequest 并解析数据的性能
            b.iter(|| {
                let mut request = WriteRequest::default();
                let data = data.clone();
                request.merge(data).unwrap();  //在每次迭代中创建新的 request 实例
            });
        })
        .bench_function("pooled_write_request", |b| { //第二个基准测试：池化版本, 测试重用同一个 WriteRequest 实例的性能
            b.iter(|| {
                request_pooled.clear();
                let data = data.clone();
                request_pooled.merge(data).unwrap();  //每次迭代前清除 request_pooled，然后重新解析数据
            });
        });
}

//创建一个包含 bench_decode_prom_request 函数的基准测试组
criterion_group!(benches, bench_decode_prom_request);
//设置主函数来运行这个基准测试组
criterion_main!(benches);
