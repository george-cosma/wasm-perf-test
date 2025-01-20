```c
tinywasm_5              time:   [1.7935 µs 1.8726 µs 1.9638 µs]
                        change: [+15.095% +19.583% +24.551%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 1 outliers among 100 measurements (1.00%)
  1 (1.00%) high mild

tinywasm_50             time:   [29.750 µs 30.339 µs 31.039 µs]
                        change: [+1.4190% +3.6450% +5.8826%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 17 outliers among 100 measurements (17.00%)
  7 (7.00%) high mild
  10 (10.00%) high severe

tinywasm_100            time:   [85.857 µs 86.228 µs 86.682 µs]
                        change: [-0.7678% +0.9304% +3.0062%] (p = 0.33 > 0.05)
                        No change in performance detected.
Found 13 outliers among 100 measurements (13.00%)
  1 (1.00%) low mild
  4 (4.00%) high mild
  8 (8.00%) high severe

tinywasm_1000           time:   [6.7082 ms 6.9719 ms 7.2461 ms]
                        change: [+26.976% +31.868% +37.034%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 1 outliers among 100 measurements (1.00%)
  1 (1.00%) high mild

wasm_interpeter_5       time:   [3.7558 µs 3.7948 µs 3.8459 µs]
                        change: [+2.0955% +4.1261% +6.4752%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 12 outliers among 100 measurements (12.00%)
  10 (10.00%) high mild
  2 (2.00%) high severe

wasm_interpeter_50      time:   [104.78 µs 110.19 µs 116.31 µs]
                        change: [+7.6295% +12.080% +16.262%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 7 outliers among 100 measurements (7.00%)
  4 (4.00%) high mild
  3 (3.00%) high severe

wasm_interpeter_100     time:   [302.98 µs 310.24 µs 317.75 µs]
                        change: [+4.8944% +6.8661% +8.9544%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 3 outliers among 100 measurements (3.00%)
  3 (3.00%) high mild

wasm_interpeter_1000    time:   [19.576 ms 20.254 ms 20.986 ms]
                        change: [+10.828% +14.835% +19.688%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 6 outliers among 100 measurements (6.00%)
  6 (6.00%) high mild

wasmtime_5              time:   [53.060 ns 53.379 ns 53.752 ns]
                        change: [-1.6497% +0.1524% +2.0533%] (p = 0.87 > 0.05)
                        No change in performance detected.
Found 10 outliers among 100 measurements (10.00%)
  1 (1.00%) high mild
  9 (9.00%) high severe

wasmtime_50             time:   [1.7596 µs 1.7726 µs 1.7857 µs]
                        change: [-16.941% -14.356% -11.832%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 20 outliers among 100 measurements (20.00%)
  2 (2.00%) low severe
  5 (5.00%) high mild
  13 (13.00%) high severe

wasmtime_100            time:   [5.7733 µs 5.8316 µs 5.9030 µs]
                        change: [-0.1364% +1.9318% +4.2826%] (p = 0.08 > 0.05)
                        No change in performance detected.
Found 18 outliers among 100 measurements (18.00%)
  5 (5.00%) high mild
  13 (13.00%) high severe

wasmtime_1000           time:   [396.10 µs 397.98 µs 400.09 µs]
                        change: [-1.4542% -0.2326% +0.9424%] (p = 0.71 > 0.05)
                        No change in performance detected.
Found 12 outliers among 100 measurements (12.00%)
  5 (5.00%) high mild
  7 (7.00%) high severe
```

| num_primes | wasm_interpreter | tinywasm  | wasmtime (JIT) |
|------------|------------------|-----------|----------------|
| 5          | 3.7948 µs        | 1.8726 µs | 53.379 ns      |
| 50         | 110.19 µs        | 30.339 µs | 1.7726 µs      |
| 100        | 310.24 µs        | 86.228 µs | 5.8316 µs      |
| 1000       | 20.254 ms        | 6.9719 ms | 397.98 µs      |


## How?

The WASM code used for working out these test can be found in `/src/lib.rs/`. In
short, we count how many prime numbers there are between 0 and `num_primes`,
each number being tested by checking divisibility for each number up to the
tested number. Here is the rust-equivelent of it:

```rs
pub fn primes(max_num: u64) -> u64 {
    let mut result = 0;
    for num in 0..=max_num {
        let mut prime = true;
        for k in 2..num {
            if num % k == 0 {
                prime = false;
                break;
            }
        }

        if prime {
            result += 1;
        }
    }

    return result;
}
```

All of the interpreters were used using their default settings. This means that,
for wasmtime, the WASM bytecode was compiled down to machine code and ran using
JIT. I've tried to make it use the `pulley` runtime, wasmtime's built-in
interpreter, instead of `cranelift` or `winch`, the machine-code compilers.
However, I failed in this endeavour.

The more important comparison is between `wasm-interpreter` and `tinywasm`. The
~ x3 performance difference is yet to be explained, but here are some potential
candidates:
- Stack efficiency - a lot of computation time is spent on `Stack::push` and
  `Stack::pop`
- Sidetable transfer efficiency (`do_sidetable_transfer`) 
- Using parsing functions inside execution e.g. `::parse_u32`


