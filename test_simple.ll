; ModuleID = 'program'
source_filename = "program"

@string_literal = global [8 x i8] c"testing\00"
@format_str = global [4 x i8] c"%s\0A\00"

define i64 @main() {
entry:
  %printf_str_call = call i64 (ptr, ...) @printf(ptr @format_str, ptr @string_literal)
  ret i64 0
}

declare i64 @printf(ptr, ...)
