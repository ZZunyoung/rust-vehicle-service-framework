use criterion::{criterion_group, criterion_main, Criterion};
use framework::event::Event;
use framework::registry::ServiceRegistry;
use std::hint::black_box;
use std::sync::{Arc, Mutex, RwLock};
use std::thread;

const THREADS: usize = 8;
const READS_PER_THREAD: usize = 2_000;

fn build_registry() -> ServiceRegistry {
    let mut registry = ServiceRegistry::new();
    for i in 0..10 {
        registry.register(
            &format!("service-{i}"),
            vec![Event::DoorOpened],
            &format!("/tmp/service-{i}.sock"),
        );
    }
    registry
}

fn bench_mutex(c: &mut Criterion) {
    c.bench_function("registry read-heavy / Mutex", |b| {
        let registry = Arc::new(Mutex::new(build_registry()));

        b.iter(|| {
            let mut handles = Vec::new();

            for _ in 0..THREADS {
                let registry = Arc::clone(&registry);
                handles.push(thread::spawn(move || {
                    for _ in 0..READS_PER_THREAD {
                        let subs = registry
                            .lock()
                            .unwrap()
                            .subscribers_for(black_box(&Event::DoorOpened));
                        black_box(subs);
                    }
                }));
            }

            for handle in handles {
                handle.join().unwrap();
            }
        });
    });
}

fn bench_rwlock(c: &mut Criterion) {
    c.bench_function("registry read-heavy / RwLock", |b| {
        let registry = Arc::new(RwLock::new(build_registry()));

        b.iter(|| {
            let mut handles = Vec::new();

            for _ in 0..THREADS {
                let registry = Arc::clone(&registry);
                handles.push(thread::spawn(move || {
                    for _ in 0..READS_PER_THREAD {
                        let subs = registry
                            .read()
                            .unwrap()
                            .subscribers_for(black_box(&Event::DoorOpened));
                        black_box(subs);
                    }
                }));
            }

            for handle in handles {
                handle.join().unwrap();
            }
        });
    });
}

const WRITES_PER_THREAD: usize = 2_000;

fn bench_mutex_write(c: &mut Criterion) {
    c.bench_function("registry write-heavy / Mutex", |b| {
        let registry = Arc::new(Mutex::new(build_registry()));

        b.iter(|| {
            let mut handles = Vec::new();

            for t in 0..THREADS {
                let registry = Arc::clone(&registry);
                handles.push(thread::spawn(move || {
                    let name = format!("service-{}", t % 10);
                    for _ in 0..WRITES_PER_THREAD {
                        let ok = registry.lock().unwrap().mark_heartbeat(black_box(&name));
                        black_box(ok);
                    }
                }));
            }

            for handle in handles {
                handle.join().unwrap();
            }
        });
    });
}

fn bench_rwlock_write(c: &mut Criterion) {
    c.bench_function("registry write-heavy / RwLock", |b| {
        let registry = Arc::new(RwLock::new(build_registry()));

        b.iter(|| {
            let mut handles = Vec::new();

            for t in 0..THREADS {
                let registry = Arc::clone(&registry);
                handles.push(thread::spawn(move || {
                    let name = format!("service-{}", t % 10);
                    for _ in 0..WRITES_PER_THREAD {
                        let ok = registry.write().unwrap().mark_heartbeat(black_box(&name));
                        black_box(ok);
                    }
                }));
            }

            for handle in handles {
                handle.join().unwrap();
            }
        });
    });
}

criterion_group!(
    benches,
    bench_mutex,
    bench_rwlock,
    bench_mutex_write,
    bench_rwlock_write
);
criterion_main!(benches);
