/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */

#![cfg_attr(feature = "fatal-warnings", deny(warnings))]

use criterion::{criterion_group, criterion_main, Criterion};
use std::hint::black_box;

#[cfg(not(feature = "triomphe-bench"))]
use rpds::ListSync;
#[cfg(feature = "triomphe-bench")]
type ListSync<T> = rpds::List<T, archery::ArcTK>;

fn rpds_list_sync_push_front(c: &mut Criterion) {
    let limit = 10_000;

    c.bench_function("rpds list sync push front", move |b| {
        b.iter(|| {
            let mut list: ListSync<usize> = ListSync::new_with_ptr_kind();

            for i in 0..limit {
                list = list.push_front(i);
            }

            list
        })
    });
}

fn rpds_list_sync_push_front_mut(c: &mut Criterion) {
    let limit = 10_000;

    c.bench_function("rpds list sync push front mut", move |b| {
        b.iter(|| {
            let mut list: ListSync<usize> = ListSync::new_with_ptr_kind();

            for i in 0..limit {
                list.push_front_mut(i);
            }

            list
        })
    });
}

fn rpds_list_sync_drop_first(c: &mut Criterion) {
    let limit = 10_000;

    c.bench_function("rpds list sync drop first", move |b| {
        b.iter_with_setup(
            || {
                let mut list: ListSync<usize> = ListSync::new_with_ptr_kind();

                for i in 0..limit {
                    list.push_front_mut(i);
                }

                list
            },
            |mut list| {
                for _ in 0..limit {
                    list = list.drop_first().unwrap();
                }

                list
            },
        );
    });
}

fn rpds_list_sync_drop_first_mut(c: &mut Criterion) {
    let limit = 10_000;

    c.bench_function("rpds list sync drop first mut", move |b| {
        b.iter_with_setup(
            || {
                let mut list: ListSync<usize> = ListSync::new_with_ptr_kind();

                for i in 0..limit {
                    list.push_front_mut(i);
                }

                list
            },
            |mut list| {
                for _ in 0..limit {
                    list.drop_first_mut();
                }

                list
            },
        );
    });
}

fn rpds_list_sync_reverse(c: &mut Criterion) {
    let limit = 1_000;

    c.bench_function("rpds list sync reverse", move |b| {
        b.iter_with_setup(
            || {
                let mut list: ListSync<usize> = ListSync::new_with_ptr_kind();

                for i in 0..limit {
                    list.push_front_mut(i);
                }

                list
            },
            |mut list| {
                for _ in 0..limit {
                    list = list.reverse();
                }

                list
            },
        );
    });
}

fn rpds_list_sync_reverse_mut(c: &mut Criterion) {
    let limit = 1_000;

    c.bench_function("rpds list sync reverse mut", move |b| {
        b.iter_with_setup(
            || {
                let mut list: ListSync<usize> = ListSync::new_with_ptr_kind();

                for i in 0..limit {
                    list.push_front_mut(i);
                }

                list
            },
            |mut list| {
                for _ in 0..limit {
                    list.reverse_mut();
                }

                list
            },
        );
    });
}

fn rpds_list_sync_iterate(c: &mut Criterion) {
    let limit = 10_000;
    let mut list: ListSync<usize> = ListSync::new_with_ptr_kind();

    for i in 0..limit {
        list.push_front_mut(i);
    }

    c.bench_function("rpds list sync iterate", move |b| {
        b.iter(|| {
            for i in list.iter() {
                black_box(i);
            }
        })
    });
}

criterion_group!(
    benches,
    rpds_list_sync_push_front,
    rpds_list_sync_push_front_mut,
    rpds_list_sync_drop_first,
    rpds_list_sync_drop_first_mut,
    rpds_list_sync_reverse,
    rpds_list_sync_reverse_mut,
    rpds_list_sync_iterate
);
criterion_main!(benches);
