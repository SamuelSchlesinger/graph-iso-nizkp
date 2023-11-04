use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use graph_iso_pfs::{graph::Graph, prove};
use perm::{Action, Table};

fn benchmark<const N: usize>(c: &mut Criterion) {
    let a: Box<Graph<N>> = Box::new(rand::random());
    let t: Box<Table<N>> = Box::new(rand::random());
    let b: Box<Graph<N>> = Box::new(t.act(&a));

    c.bench_with_input(
        BenchmarkId::new("NIZK graph iso proof", N),
        black_box(&(&a, &b, &t)),
        |bencher, &(a, b, t)| {
            bencher.iter(|| Box::new(prove(*a.clone(), *b.clone(), *t.clone(), 32)));
        },
    );
}

criterion_group!(benches, benchmark<20>, benchmark<100>, benchmark<500>,);
criterion_main!(benches);
