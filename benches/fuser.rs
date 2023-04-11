use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use micam::fuser::get_pid_using_file;

fn bench_get_pid_using_file(c: &mut Criterion) {
    let file_path = "/dev/video1"; // Update with your file path
    let mut group = c.benchmark_group("get_pid_using_file");
    group.bench_function(BenchmarkId::new("file1", file_path), |b| {
        b.iter(|| get_pid_using_file(file_path))
    });
    group.finish();
}
criterion_group!(benches, bench_get_pid_using_file);
criterion_main!(benches);
