(module
  (memory 1)

  ;; Initialize data
  ;; 0x0000: 01 02 03 04 05 06 07 08
  (data (i32.const 0x0000) "\01\02\03\04\05\06\07\08")
  ;; 0xFFF8: AA BB CC DD EE FF 11 22
  (data (i32.const 0xFFF8) "\aa\bb\cc\dd\ee\ff\11\22")

  ;; Load helpers
  (func (export "load8") (param i32) (result i32)
    (i32.load8_u (local.get 0))
  )
  (func (export "load16") (param i32) (result i32)
    (i32.load16_u (local.get 0))
  )
  (func (export "load32") (param i32) (result i32)
    (i32.load (local.get 0))
  )
  (func (export "load64") (param i32) (result i64)
    (i64.load (local.get 0))
  )

  ;; Store helpers
  (func (export "store32") (param i32) (param i32)
    (i32.store (local.get 0) (local.get 1))
  )
  (func (export "store64") (param i32) (param i64)
    (i64.store (local.get 0) (local.get 1))
  )
)

;; --------------------------
;; i8 / i16 / i32 LOAD TESTS
;; --------------------------

;; Wrap-around i8 load: 0x10000 → 0x0000
(assert_return (invoke "load8" (i32.const 0x10000)) (i32.const 0x01))

;; Wrap-around i16 load
(assert_return (invoke "load16" (i32.const 0x10000)) (i32.const 0x0201))

;; Wrap-around i32 load
(assert_return (invoke "load32" (i32.const 0x10000)) (i32.const 0x04030201))

;; i32 load at boundary: 0xFFFD (3 valid bytes)
;; Expected: 0x002211FF
(assert_return (invoke "load32" (i32.const 0xFFFD)) (i32.const 0x002211FF))

;; i32 load at 0xFFFE (2 valid bytes)
(assert_return (invoke "load32" (i32.const 0xFFFE)) (i32.const 0x00002211))

;; i32 load at 0xFFFF (1 valid byte)
(assert_return (invoke "load32" (i32.const 0xFFFF)) (i32.const 0x00000022))

;; i16 load at 0xFFFF
(assert_return (invoke "load16" (i32.const 0xFFFF)) (i32.const 0x0022))

;; i8 load at 0xFFFF
(assert_return (invoke "load8" (i32.const 0xFFFF)) (i32.const 0x22))

;; Wrap-around load using overflowed address: 0xFFFFFFFC → 0xFFFC
(assert_return (invoke "load32" (i32.const 0xFFFFFFFC)) (i32.const 0x2211FFEE))

;; --------------------------
;; i64 LOAD TESTS
;; --------------------------

;; Full i64 load at 0x0000: 01..08
(assert_return
  (invoke "load64" (i32.const 0x0000))
  (i64.const 0x0807060504030201)
)

;; Full i64 load at 0xFFF8: AA..22
(assert_return
  (invoke "load64" (i32.const 0xFFF8))
  (i64.const 0x2211FFEEDDCCBBAA)
)

;; i64 load at 0xFFF9 (7 valid bytes)
(assert_return
  (invoke "load64" (i32.const 0xFFF9))
  (i64.const 0x002211FFEEDDCCBB)
)

;; i64 load at 0xFFFE (2 valid bytes)
(assert_return
  (invoke "load64" (i32.const 0xFFFE))
  (i64.const 0x0000000000002211)
)

;; i64 load at 0xFFFF (1 valid byte)
(assert_return
  (invoke "load64" (i32.const 0xFFFF))
  (i64.const 0x0000000000000022)
)

;; i64 wrap-around load: 0x10000 → 0x0000
(assert_return
  (invoke "load64" (i32.const 0x10000))
  (i64.const 0x0807060504030201)
)

;; i64 wrap-around + boundary read: 0x1FFFE → 0xFFFE
(assert_return
  (invoke "load64" (i32.const 0x1FFFE))
  (i64.const 0x0000000000002211)
)

;; --------------------------
;; STORE TESTS
;; --------------------------

;; Store32 at 0x10000 → wraps to 0x0000
(invoke "store32" (i32.const 0x10000) (i32.const 0xDEADBEEF))
(assert_return (invoke "load32" (i32.const 0x0000)) (i32.const 0xDEADBEEF))

;; Store64 at 0xFFFE → only last 2 bytes should be written
(invoke "store64" (i32.const 0xFFFE) (i64.const 0x1122334455667788))
(assert_return (invoke "load16" (i32.const 0xFFFE)) (i32.const 0x7788))
(assert_return (invoke "load64" (i32.const 0xFFFE)) (i64.const 0x1122334455667788))

;; Store32 at 0xFFFD → last 3 bytes written
(invoke "store32" (i32.const 0xFFFD) (i32.const 0xAABBCCDD))
(assert_return (invoke "load32" (i32.const 0xFFFD)) (i32.const 0xAABBCCDD))
