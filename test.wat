1111
-12.123123
+12.123123
12.2234545324652
(module
    (import "env" "print_string" (func $print_string (param i32)))
    (import "env" "buffer" (memory 1))
    (global $start_string (import "env" "start_string") i32)
    (global $string_len i32 (i32.const 12))
    (global $test1 i32 (i32.const -12))
    (global $test2 i32 (i32.const +12))
    (global $test3 i32 (i32.const nan))
    (global $test3 i32 (i32.const nan:0x7ff80000))
    (global $test4 i32 (i32.const inf))
    (global $test5 i32 (i32.const -inf))
    (data (global.get $start_string) "hello world!")
    (func (export "helloworld")
        (call $print_string (global.get $string_len))
    )
)
