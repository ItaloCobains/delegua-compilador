; ModuleID = 'program'
source_filename = "program"

@string_literal = global [21 x i8] c"Testando loop while:\00"
@format_str = global [4 x i8] c"%s\0A\00"
@string_literal.1 = global [11 x i8] c"Contador: \00"
@format_str.2 = global [6 x i8] c"%lld\0A\00"
@string_literal.3 = global [17 x i8] c"Loop completado!\00"

define i64 @main() {
entry:
  %printf_str_call = call i64 (ptr, ...) @printf(ptr @format_str, ptr @string_literal)
  %i = alloca i64, align 8
  store i64 0, ptr %i, align 4
  br label %loop

loop:                                             ; preds = %loop_body, %entry
  %i1 = load i64, ptr %i, align 4
  %lt = icmp slt i64 %i1, 3
  br i1 %lt, label %loop_body, label %after_loop

loop_body:                                        ; preds = %loop
  %printf_str_call2 = call i64 (ptr, ...) @printf(ptr @format_str, ptr @string_literal.1)
  %i3 = load i64, ptr %i, align 4
  %printf_int_call = call i64 (ptr, ...) @printf(ptr @format_str.2, i64 %i3)
  %i4 = load i64, ptr %i, align 4
  %add = add i64 %i4, 1
  store i64 %add, ptr %i, align 4
  br label %loop

after_loop:                                       ; preds = %loop
  %printf_str_call5 = call i64 (ptr, ...) @printf(ptr @format_str, ptr @string_literal.3)
  ret i64 0
}

declare i64 @printf(ptr, ...)
