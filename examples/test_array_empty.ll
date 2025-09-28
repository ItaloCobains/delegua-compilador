; ModuleID = 'program'
source_filename = "program"

@string_literal = global [32 x i8] c"Array vazio criado com sucesso!\00"
@format_str = global [4 x i8] c"%s\0A\00"

define i64 @main() {
entry:
  %array_alloc = call ptr @malloc(i64 8)
  store i64 0, ptr %array_alloc, align 4
  %vazio = alloca ptr, align 8
  store ptr %array_alloc, ptr %vazio, align 8
  %printf_str_call = call i64 (ptr, ...) @printf(ptr @format_str, ptr @string_literal)
  ret i64 0
}

declare ptr @malloc(i64)

declare i64 @printf(ptr, ...)
