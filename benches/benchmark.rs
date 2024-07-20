use blocklist::{
    datablock::DataBlock, datablock2::DataBlock2, linkedlist::LinkedList, linkedlist2::LinkedList2,
    pool::Pool2, ptrbased::PtrBased, smallobjectpool::SmallObjectPool,
};
use criterion::{black_box, criterion_group, criterion_main, Criterion};

const BLOCK_SIZE: usize = 1024 * 10;
const ITERS: i32 = 100000;
type DTYPE = f64;

pub fn datablock_push_benchmark(c: &mut Criterion) {
    c.bench_function("array push", |b| {
        let mut list: DataBlock<f64, BLOCK_SIZE> = DataBlock::new();
        b.iter(|| {
            for i in 0..ITERS {
                list.insert(i as usize, black_box(i as DTYPE));
            }
        })
    });
}

pub fn datablock2_push_benchmark(c: &mut Criterion) {
    c.bench_function("array 2 push", |b| {
        let mut list: DataBlock2<f64, BLOCK_SIZE> = DataBlock2::new();
        b.iter(|| {
            for i in 0..ITERS {
                list.insert(i as usize, black_box(i as DTYPE));
            }
        })
    });
}

pub fn vec_push_benchmark(c: &mut Criterion) {
    c.bench_function("vec push", |b| {
        let mut vec: Vec<DTYPE> = Vec::new();
        b.iter(|| {
            for i in 0..ITERS {
                vec.push(i as DTYPE);
            }
        });
        black_box(vec);
    });
}

pub fn linked_list_push_benchmark(c: &mut Criterion) {
    c.bench_function("linked list push", |b| {
        let mut vec: LinkedList<DTYPE> = LinkedList::new();
        b.iter(|| {
            for i in 0..ITERS {
                vec.push_back(i as DTYPE);
            }
        });
        black_box(vec);
    });
}

pub fn linked_list_2_push_benchmark(c: &mut Criterion) {
    c.bench_function("linked list 2 push", |b| {
        let mut vec: LinkedList2<DTYPE> = LinkedList2::new();
        b.iter(|| {
            for i in 0..ITERS {
                vec.push_back(i as DTYPE);
            }
        });
        black_box(vec);
    });
}

pub fn sop_push_benchmark(c: &mut Criterion) {
    c.bench_function("sop push", |b| {
        let mut list: SmallObjectPool<DTYPE, BLOCK_SIZE> = SmallObjectPool::new();
        b.iter(|| {
            for i in 0..ITERS {
                list.push(i as DTYPE);
            }
        });
        black_box(list);
    });
}

pub fn pool_push_benchmark(c: &mut Criterion) {
    c.bench_function("pool push", |b| {
        let mut blocklist: Pool2<DTYPE, BLOCK_SIZE> = Pool2::new();
        b.iter(|| {
            for i in 0..ITERS {
                blocklist.push(i as DTYPE);
            }
        });
        black_box(blocklist);
    });
}

criterion_group!(
    benches,
    sop_push_benchmark,
    // datablock_push_benchmark,
    // datablock2_push_benchmark,
    vec_push_benchmark,
    // linked_list_push_benchmark,
    // linked_list_2_push_benchmark,
);
criterion_main!(benches);
