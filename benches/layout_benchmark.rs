use criterion::{Criterion, criterion_group, criterion_main};
use std::hint::black_box;

// 1. Định nghĩa kiểu AoS
#[derive(Clone, Copy)]
struct PointAoS {
    x: u64,
    y: u64,
    z: u64,
}

// 2. Định nghĩa kiểu SoA
struct PointSoA {
    xs: Vec<u64>,
    ys: Vec<u64>,
    zs: Vec<u64>,
}

// Hàm tạo dữ liệu
fn create_data(size: usize) -> (Vec<PointAoS>, PointSoA) {
    let mut vec_aos = Vec::with_capacity(size);
    let mut soa = PointSoA {
        xs: Vec::with_capacity(size),
        ys: Vec::with_capacity(size),
        zs: Vec::with_capacity(size),
    };

    for i in 0..size {
        let p = PointAoS {
            x: i as u64,
            y: i as u64 * 2,
            z: i as u64 * 3,
        };
        vec_aos.push(p);
        soa.xs.push(p.x);
        soa.ys.push(p.y);
        soa.zs.push(p.z);
    }
    (vec_aos, soa)
}

// 3. Hàm benchmark AoS
// Chỉ truy cập 1/3 dữ liệu chúng ta tải lên
fn sum_x_aos(points: &[PointAoS]) -> u64 {
    let mut sum = 0;
    for p in points {
        sum += p.x;
    }
    sum
}

// 4. Hàm benchmark SoA
// Mọi byte tải lên đều được dùng
fn sum_x_soa(points: &PointSoA) -> u64 {
    let mut sum = 0;
    for x in &points.xs {
        sum += x;
    }
    sum
}

// 5. Thiết lập benchmark
fn layout_benchmark(c: &mut Criterion) {
    let (aos_data, soa_data) = create_data(10_000);

    let mut group = c.benchmark_group("AoS vs SoA (Sum X)");

    group.bench_function("AoS", |b| b.iter(|| sum_x_aos(black_box(&aos_data))));

    group.bench_function("SoA", |b| b.iter(|| sum_x_soa(black_box(&soa_data))));

    group.finish();
}

criterion_group!(benches, layout_benchmark);
criterion_main!(benches);
