; ModuleID = 'program'
source_filename = "program"

@format_int = global [6 x i8] c"%lld\0A\00"

define i64 @main() {
entry:
  %x = alloca i64, align 8
  store i64 3, ptr %x, align 4
  %x1 = load i64, ptr %x, align 4
  %printf_call = call i64 (ptr, ...) @printf(ptr @format_int, i64 %x1)
  ret i64 0
}

declare i64 @printf(ptr, ...)
