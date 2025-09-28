; ModuleID = 'program'
source_filename = "program"

@string_literal = global [7 x i8] c"Soma: \00"
@format_str = global [4 x i8] c"%s\0A\00"
@format_str.1 = global [6 x i8] c"%lld\0A\00"
@string_literal.2 = global [16 x i8] c"Multiplicacao: \00"
@string_literal.3 = global [12 x i8] c"Subtracao: \00"

define i64 @main() {
entry:
  %a = alloca i64, align 8
  store i64 10, ptr %a, align 4
  %b = alloca i64, align 8
  store i64 20, ptr %b, align 4
  %array_alloc = call ptr @malloc(i64 32)
  store i64 3, ptr %array_alloc, align 4
  %a1 = load i64, ptr %a, align 4
  %b2 = load i64, ptr %b, align 4
  %add = add i64 %a1, %b2
  %array_elem_ptr_0 = getelementptr i64, ptr %array_alloc, i64 1
  store i64 %add, ptr %array_elem_ptr_0, align 4
  %a3 = load i64, ptr %a, align 4
  %b4 = load i64, ptr %b, align 4
  %mul = mul i64 %a3, %b4
  %array_elem_ptr_1 = getelementptr i64, ptr %array_alloc, i64 2
  store i64 %mul, ptr %array_elem_ptr_1, align 4
  %a5 = load i64, ptr %a, align 4
  %b6 = load i64, ptr %b, align 4
  %sub = sub i64 %a5, %b6
  %array_elem_ptr_2 = getelementptr i64, ptr %array_alloc, i64 3
  store i64 %sub, ptr %array_elem_ptr_2, align 4
  %arr = alloca ptr, align 8
  store ptr %array_alloc, ptr %arr, align 8
  %printf_str_call = call i64 (ptr, ...) @printf(ptr @format_str, ptr @string_literal)
  %arr7 = load ptr, ptr %arr, align 8
  %array_element_ptr = getelementptr i64, ptr %arr7, i64 1
  %array_element = load i64, ptr %array_element_ptr, align 4
  %printf_int_call = call i64 (ptr, ...) @printf(ptr @format_str.1, i64 %array_element)
  %printf_str_call8 = call i64 (ptr, ...) @printf(ptr @format_str, ptr @string_literal.2)
  %arr9 = load ptr, ptr %arr, align 8
  %array_element_ptr10 = getelementptr i64, ptr %arr9, i64 2
  %array_element11 = load i64, ptr %array_element_ptr10, align 4
  %printf_int_call12 = call i64 (ptr, ...) @printf(ptr @format_str.1, i64 %array_element11)
  %printf_str_call13 = call i64 (ptr, ...) @printf(ptr @format_str, ptr @string_literal.3)
  %arr14 = load ptr, ptr %arr, align 8
  %array_element_ptr15 = getelementptr i64, ptr %arr14, i64 3
  %array_element16 = load i64, ptr %array_element_ptr15, align 4
  %printf_int_call17 = call i64 (ptr, ...) @printf(ptr @format_str.1, i64 %array_element16)
  ret i64 0
}

declare ptr @malloc(i64)

declare i64 @printf(ptr, ...)
