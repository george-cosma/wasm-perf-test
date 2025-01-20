use criterion::*;
use wasm::{validate, RuntimeInstance, DEFAULT_MODULE};

const TEST_VALS: &[i64] = &[5, 50, 100, 1_000];

fn test_wasm_interpreter(c: &mut Criterion) {
    let binary = wat::parse_str(wasm_perf_test::WAT_PRIMES).unwrap();

    let validation_info = validate(&binary).unwrap();
    let mut instance = RuntimeInstance::new(&validation_info).unwrap();

    let primes = instance
        .get_function_by_name(DEFAULT_MODULE, "primes")
        .unwrap();

    for num_primes in TEST_VALS {
        let name = format!("wasm_interpeter_{}", num_primes);
        c.bench_function(&name, |b| {
            b.iter(|| {
                let _: i64 = instance.invoke(&primes, black_box(*num_primes)).unwrap();
            });
        });
    }
}

fn test_wasmtime(c: &mut Criterion) {
    let engine = wasmtime::Engine::default();
    let mut store = wasmtime::Store::new(&engine, ());

    let binary = wat::parse_str(wasm_perf_test::WAT_PRIMES).unwrap();
    let module = wasmtime::Module::from_binary(&engine, &binary).unwrap();

    let instance = wasmtime::Instance::new(&mut store, &module, &[]).unwrap();

    let primes = instance
        .get_typed_func::<i64, i64>(&mut store, "primes")
        .unwrap();

    for num_primes in TEST_VALS {
        let name = format!("wasmtime_{}", num_primes);
        c.bench_function(&name, |b| {
            b.iter(|| {
                primes.call(&mut store, black_box(*num_primes)).unwrap();
            });
        });
    }
}

fn test_tinywasm(c: &mut Criterion) {
    let binary = wat::parse_str(wasm_perf_test::WAT_PRIMES).unwrap();
    let module = tinywasm::Module::parse_bytes(&binary).unwrap();
    let mut store = tinywasm::Store::default();
    let instance = module.instantiate(&mut store, None).unwrap();

    let primes = instance
        .exported_func::<i64, i64>(&store, "primes")
        .unwrap();

    for num_primes in TEST_VALS {
        let name = format!("tinywasm_{}", num_primes);
        c.bench_function(&name, |b| {
            b.iter(|| {
                primes.call(&mut store, black_box(*num_primes)).unwrap();
            });
        });
    }
}

criterion_group!(benches, test_tinywasm, test_wasm_interpreter, test_wasmtime);
criterion_main!(benches);
