use criterion::{black_box, criterion_group, criterion_main, Criterion};
use smallobjectpool::{
    arraylike::ArrayLike, linkedlist::LinkedList, ptrbased::PtrBased,
    smallobjectpool::SmallObjectPool,
};

const BLOCK_SIZE: usize = 1024;
const ITERS: i32 = 1024 * 10;
type DTYPE = f64;

pub fn array_like_direct_insert_benchmark(c: &mut Criterion) {
    c.bench_function("array direct insert", |b| {
        b.iter(|| {
            let list: ArrayLike<DTYPE, BLOCK_SIZE> = ArrayLike::new();
            let mut ptr = list.begin().unwrap();
            for i in 0..BLOCK_SIZE {
                unsafe {
                    ptr.as_ptr().write(i as DTYPE);
                    ptr = list.next(ptr).unwrap();
                }
            }
            black_box(list);
        })
    });
}

pub fn vec_push_benchmark(c: &mut Criterion) {
    c.bench_function("vec push", |b| {
        b.iter(|| {
            let mut vec: Vec<DTYPE> = Vec::new();
            for i in 0..BLOCK_SIZE {
                vec.push(i as DTYPE);
            }
            black_box(vec);
        });
    });
}

pub fn linked_list_push_benchmark(c: &mut Criterion) {
    c.bench_function("linked list push", |b| {
        b.iter(|| {
            let mut vec: LinkedList<DTYPE> = LinkedList::new();
            for i in 0..ITERS {
                vec.push_back(i as DTYPE);
            }
            black_box(vec);
        });
    });
}

pub fn sop_push_benchmark(c: &mut Criterion) {
    c.bench_function("sop push", |b| {
        b.iter(|| {
            let mut list: SmallObjectPool<DTYPE, BLOCK_SIZE> = SmallObjectPool::new();
            for i in 0..ITERS {
                list.push(i as DTYPE);
            }
            black_box(list);
        });
    });
}

pub fn vec_high_vol_push_benchmark(c: &mut Criterion) {
    c.bench_function("vec high volume push", |b| {
        b.iter(|| {
            let mut vec: Vec<DTYPE> = Vec::new();
            for i in 0..ITERS {
                vec.push(i as DTYPE);
            }
            black_box(vec);
        });
    });
}

criterion_group!(
    benches,
    vec_push_benchmark,
    array_like_direct_insert_benchmark,
    sop_push_benchmark,
    linked_list_push_benchmark,
    vec_high_vol_push_benchmark
);
criterion_main!(benches);
