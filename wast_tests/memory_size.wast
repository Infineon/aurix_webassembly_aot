(module
  (memory 0)
  (func (export "size") (result i32) (memory.size))
  (func (export "grow") (param $sz i32) (drop (memory.grow (local.get $sz))))
)

(assert_return (invoke "size") (i32.const 0))
(assert_return (invoke "grow" (i32.const 1)))
(assert_return (invoke "size") (i32.const 1))
(assert_return (invoke "grow" (i32.const 4)))
(assert_return (invoke "size") (i32.const 1)) ;; :MODIFIED: Test environment supports only two memory pages
(assert_return (invoke "grow" (i32.const 0)))
(assert_return (invoke "size") (i32.const 1)) ;; :MODIFIED: Test environment supports only two memory pages

(module
  (memory 1)
  (func (export "size") (result i32) (memory.size))
  (func (export "grow") (param $sz i32) (drop (memory.grow (local.get $sz))))
)

(assert_return (invoke "size") (i32.const 1))
(assert_return (invoke "grow" (i32.const 1)))
(assert_return (invoke "size") (i32.const 2))
(assert_return (invoke "grow" (i32.const 4)))
(assert_return (invoke "size") (i32.const 2)) ;; :MODIFIED: Test environment supports only two memory pages
(assert_return (invoke "grow" (i32.const 0)))
(assert_return (invoke "size") (i32.const 2)) ;; :MODIFIED: Test environment supports only two memory pages

(module
  (memory 0 2)
  (func (export "size") (result i32) (memory.size))
  (func (export "grow") (param $sz i32) (drop (memory.grow (local.get $sz))))
)

(assert_return (invoke "size") (i32.const 0))
(assert_return (invoke "grow" (i32.const 3)))
(assert_return (invoke "size") (i32.const 0))
(assert_return (invoke "grow" (i32.const 1)))
(assert_return (invoke "size") (i32.const 1))
(assert_return (invoke "grow" (i32.const 0)))
(assert_return (invoke "size") (i32.const 1))
(assert_return (invoke "grow" (i32.const 4)))
(assert_return (invoke "size") (i32.const 1))
(assert_return (invoke "grow" (i32.const 1)))
(assert_return (invoke "size") (i32.const 2))
;; :MODIFIED: Test environment supports only two memory pages
;;   (memory 3 8)
;;   (func (export "size") (result i32) (memory.size))
;;   (func (export "grow") (param $sz i32) (drop (memory.grow (local.get $sz))))
;; )

;; (assert_return (invoke "size") (i32.const 3))
;; (assert_return (invoke "grow" (i32.const 1)))
;; (assert_return (invoke "size") (i32.const 4))
;; (assert_return (invoke "grow" (i32.const 3)))
;; (assert_return (invoke "size") (i32.const 7))
;; (assert_return (invoke "grow" (i32.const 0)))
;; (assert_return (invoke "size") (i32.const 7))
;; (assert_return (invoke "grow" (i32.const 2)))
;; (assert_return (invoke "size") (i32.const 7))
;; (assert_return (invoke "grow" (i32.const 1)))
;; (assert_return (invoke "size") (i32.const 8))


;; Type errors

(assert_invalid
  (module
    (memory 1)
    (func $type-result-i32-vs-empty
      (memory.size)
    )
  )
  "type mismatch"
)
(assert_invalid
  (module
    (memory 1)
    (func $type-result-i32-vs-f32 (result f32)
      (memory.size)
    )
  )
  "type mismatch"
)
