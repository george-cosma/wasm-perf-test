use wasm::{validate, RuntimeInstance, DEFAULT_MODULE};

pub fn main() {
    let num_primes: u64 = std::env::args().collect::<Vec<_>>()[1].parse().unwrap();

    let binary = wat::parse_str(wasm_perf_test::WAT_PRIMES).unwrap();

    let validation_info = validate(&binary).unwrap();
    let mut instance = RuntimeInstance::new(&validation_info).unwrap();

    let primes = instance
        .get_function_by_name(DEFAULT_MODULE, "primes")
        .unwrap();

    let _: i64 = instance.invoke(&primes, num_primes).unwrap();
}