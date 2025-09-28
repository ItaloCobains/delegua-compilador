; ModuleID = 'program'
source_filename = "program"

@string_literal = global [12 x i8] c"Hello World\00"
@string_literal.1 = global [8 x i8] c"Texto: \00"
@format_str = global [4 x i8] c"%s\0A\00"
@string_literal.2 = global [10 x i8] c"Tamanho: \00"
@format_str.3 = global [6 x i8] c"%lld\0A\00"
@string_literal.4 = global [8 x i8] c"Delegua\00"
@string_literal.5 = global [23 x i8] c"Tamanho de 'Delegua': \00"

define i64 @main() {
entry:
  %texto1 = alloca ptr, align 8
  store ptr @string_literal, ptr %texto1, align 8
  %texto11 = load ptr, ptr %texto1, align 8
  %strlen_call = call i64 @strlen(ptr %texto11)
  %tamanho = alloca i64, align 8
  store i64 %strlen_call, ptr %tamanho, align 4
  %printf_str_call = call i64 (ptr, ...) @printf(ptr @format_str, ptr @string_literal.1)
  %texto12 = load ptr, ptr %texto1, align 8
  %printf_str_call3 = call i64 (ptr, ...) @printf(ptr @format_str, ptr %texto12)
  %printf_str_call4 = call i64 (ptr, ...) @printf(ptr @format_str, ptr @string_literal.2)
  %tamanho5 = load i64, ptr %tamanho, align 4
  %printf_int_call = call i64 (ptr, ...) @printf(ptr @format_str.3, i64 %tamanho5)
  %strlen_call6 = call i64 @strlen(ptr @string_literal.4)
  %tamanho2 = alloca i64, align 8
  store i64 %strlen_call6, ptr %tamanho2, align 4
  %printf_str_call7 = call i64 (ptr, ...) @printf(ptr @format_str, ptr @string_literal.5)
  %tamanho28 = load i64, ptr %tamanho2, align 4
  %printf_int_call9 = call i64 (ptr, ...) @printf(ptr @format_str.3, i64 %tamanho28)
  ret i64 0
}

declare i64 @strlen(ptr)

declare i64 @printf(ptr, ...)
