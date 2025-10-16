;; own tests to verify that a break to the outermost block is equivalent to a return from the function
;; this is supposed to be valid as per the spec but is not checked by any of the official test files br.wast, br_if.wast, br_table.wast
(module
  (func (export "br_exit_function") (result i32)
    (block
      (i32.const 10)
      (br 1)
    )
    (i32.const 11)
  )

  (func (export "br_if_exit_function") (param i32) (result i32)
    (block (result i32)
      (i32.const 20)
      (local.get 0)
      (br_if 1)
    )
    (drop)
    (i32.const 21)
  )

  (func (export "br_table_exit_function") (param i32) (result i32)
    (block (result i32)
      (i32.const 31)
      (local.get 0)
      (br_table 0 1)
    )
    (drop)
    (i32.const 30)
  )
)

(assert_return (invoke "br_exit_function") (i32.const 10))

(assert_return (invoke "br_if_exit_function" (i32.const 0)) (i32.const 21))
(assert_return (invoke "br_if_exit_function" (i32.const 1)) (i32.const 20))

(assert_return (invoke "br_table_exit_function" (i32.const 0)) (i32.const 30))
(assert_return (invoke "br_table_exit_function" (i32.const 1)) (i32.const 31))
(assert_return (invoke "br_table_exit_function" (i32.const 2)) (i32.const 31))
(assert_return (invoke "br_table_exit_function" (i32.const -1)) (i32.const 31))