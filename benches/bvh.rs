use criterion::black_box;
use criterion::*;
use rust_bvh::*;

fn setup_2d() -> Bvh2<Bounds2, 2> {
    let objects: Vec<Bounds2> = (0..1000)
        .map(|i| ([i as f32; 2], [(i + 1) as f32; 2]).into())
        .collect();

    Bvh2::build(objects)
}

fn setup_3d() -> Bvh3A<Bounds3A, 2> {
    let objects: Vec<Bounds3A> = (0..1000)
        .map(|i| ([i as f32; 3], [(i + 1) as f32; 3]).into())
        .collect();

    Bvh3A::build(objects)
}

fn bvh_build(c: &mut Criterion) {
    let objects_2d: Vec<Bounds2> = (0..1000)
        .map(|i| ([i as f32; 2], [(i + 1) as f32; 2]).into())
        .collect();
    let objects_3d: Vec<Bounds3A> = (0..1000)
        .map(|i| ([i as f32; 3], [(i + 1) as f32; 3]).into())
        .collect();

    c.bench_function("Bvh2d build", move |b| {
        // This will avoid timing the to_vec call.
        b.iter_batched(
            || objects_2d.clone(),
            |data| Bvh2::<_, 2>::build(data),
            BatchSize::SmallInput,
        )
    });

    c.bench_function("Bvh3d build", move |b| {
        // This will avoid timing the to_vec call.
        b.iter_batched(
            || objects_3d.clone(),
            |data| Bvh3A::<_, 2>::build(data),
            BatchSize::SmallInput,
        )
    });
}

fn bvh_iter(c: &mut Criterion) {
    let bvh2d = setup_2d();
    let bvh3d = setup_3d();

    c.bench_function("Bvh2d iter", move |b| {
        b.iter(|| bvh2d.iter_objects().map(|obj| black_box(obj)));
    });
    c.bench_function("Bvh3d iter", move |b| {
        b.iter(|| bvh3d.iter_objects().map(|obj| black_box(obj)));
    });
}

fn bvh_ray(c: &mut Criterion) {
    let bvh2d = setup_2d();
    let bvh3d = setup_3d();
    let ray2d = ([0.0; 2], [1.0, 0.9]).into();
    let ray3d = ([0.0; 3], [1.0, 1.0, 0.9]).into();

    c.bench_function("Bvh2d query_ray", move |b| {
        b.iter(|| {
            bvh2d
                .query_ray(&ray2d, 0.0, f32::MAX)
                .map(|obj| black_box(obj))
        });
    });
    c.bench_function("Bvh3d query_ray", move |b| {
        b.iter(|| {
            bvh3d
                .query_ray(&ray3d, 0.0, f32::MAX)
                .map(|obj| black_box(obj))
        });
    });

    let bvh2d = setup_2d();
    let bvh3d = setup_3d();
    let ray2d = ([0.0; 2], [1.0, 0.9]).into();
    let ray3d = ([0.0; 3], [1.0, 1.0, 0.9]).into();

    c.bench_function("Bvh2d query_ray_exact", move |b| {
        b.iter(|| {
            bvh2d
                .query_ray_exact(&ray2d, 0.0, f32::MAX)
                .map(|obj| black_box(obj))
        });
    });
    c.bench_function("Bvh3d query_ray_exact", move |b| {
        b.iter(|| {
            bvh3d
                .query_ray_exact(&ray3d, 0.0, f32::MAX)
                .map(|obj| black_box(obj))
        });
    });
}

fn bvh_bounds(c: &mut Criterion) {
    let bvh2d = setup_2d();
    let bvh3d = setup_3d();
    let b2d = ([0.0; 2], [100.0; 2]).into();
    let b3d = ([0.0; 3], [100.0; 3]).into();

    c.bench_function("Bvh2d query_bounds", move |b| {
        b.iter(|| bvh2d.query_bounds(&b2d).map(|obj| black_box(obj)));
    });
    c.bench_function("Bvh3d query_bounds", move |b| {
        b.iter(|| bvh3d.query_bounds(&b3d).map(|obj| black_box(obj)));
    });

    let bvh2d = setup_2d();
    let bvh3d = setup_3d();
    let b2d = ([0.0; 2], [100.0; 2]).into();
    let b3d = ([0.0; 3], [100.0; 3]).into();

    c.bench_function("Bvh2d query_bounds_exact", move |b| {
        b.iter(|| bvh2d.query_bounds_exact(&b2d).map(|obj| black_box(obj)));
    });
    c.bench_function("Bvh3d query_bounds_exact", move |b| {
        b.iter(|| bvh3d.query_bounds_exact(&b3d).map(|obj| black_box(obj)));
    });
}

fn bvh_point(c: &mut Criterion) {
    let bvh2d = setup_2d();
    let bvh3d = setup_3d();
    let p2d = [100.5; 2].into();
    let p3d = [100.5; 3].into();

    c.bench_function("Bvh2d query_point", move |b| {
        b.iter(|| bvh2d.query_point(&p2d).map(|obj| black_box(obj)));
    });
    c.bench_function("Bvh3d query_point", move |b| {
        b.iter(|| bvh3d.query_point(&p3d).map(|obj| black_box(obj)));
    });

    let bvh2d = setup_2d();
    let bvh3d = setup_3d();
    let p2d = [100.5; 2].into();
    let p3d = [100.5; 3].into();

    c.bench_function("Bvh2d query_point_exact", move |b| {
        b.iter(|| bvh2d.query_point_exact(&p2d).map(|obj| black_box(obj)));
    });
    c.bench_function("Bvh3d query_point_exact", move |b| {
        b.iter(|| bvh3d.query_point_exact(&p3d).map(|obj| black_box(obj)));
    });
}

criterion_group!(benches, bvh_build, bvh_iter, bvh_ray, bvh_bounds, bvh_point);
criterion_main!(benches);
