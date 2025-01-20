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

pub const WAT_PRIMES: &str = r#"
(module
  ;; Function declaration with one parameter and one result
  (func $primes (param $max_num i64) (result i64)
    ;; Local variables
    (local $result i64)    ;; Counter for prime numbers
    (local $num i64)       ;; Current number being checked
    (local $k i64)         ;; Divisor
    (local $prime i32)     ;; Boolean flag for primality

    ;; Initialize result to 0
    (local.set $result (i64.const 0))
    
    ;; Main loop: for num in 0..=max_num
    (local.set $num (i64.const 0))
    (block $outer_break
      (loop $outer_loop
        ;; Check if num <= max_num
        (br_if $outer_break
          (i64.gt_u
            (local.get $num)
            (local.get $max_num)
          )
        )
        
        ;; Set prime flag to true (1)
        (local.set $prime (i32.const 1))
        
        ;; Inner loop: for k in 2..num
        (local.set $k (i64.const 2))
        (block $inner_break
          (loop $inner_loop
            ;; Check if k < num
            (br_if $inner_break
              (i64.ge_u
                (local.get $k)
                (local.get $num)
              )
            )
            
            ;; Check if num % k == 0
            (if
              (i64.eqz
                (i64.rem_u
                  (local.get $num)
                  (local.get $k)
                )
              )
              (then
                ;; Set prime to false (0) and break inner loop
                (local.set $prime (i32.const 0))
                (br $inner_break)
              )
            )
            
            ;; Increment k
            (local.set $k
              (i64.add
                (local.get $k)
                (i64.const 1)
              )
            )
            (br $inner_loop)
          )
        )
        
        ;; If prime is true, increment result
        (if
          (local.get $prime)
          (then
            (local.set $result
              (i64.add
                (local.get $result)
                (i64.const 1)
              )
            )
          )
        )
        
        ;; Increment num
        (local.set $num
          (i64.add
            (local.get $num)
            (i64.const 1)
          )
        )
        (br $outer_loop)
      )
    )
    
    ;; Return result
    (local.get $result)
  )
  
  ;; Export the function
  (export "primes" (func $primes))
)
"#;