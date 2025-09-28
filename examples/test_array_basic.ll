; ModuleID = 'program'
source_filename = "program"

@string_literal = global [20 x i8] c"Primeiro elemento: \00"
@format_str = global [4 x i8] c"%s\0A\00"
@format_str.1 = global [6 x i8] c"%lld\0A\00"
@string_literal.2 = global [19 x i8] c"Segundo elemento: \00"
@string_literal.3 = global [18 x i8] c"Ultimo elemento: \00"

define i64 @main() {
entry:
  %array_alloc = call ptr @malloc(i64 48)
  store i64 5, ptr %array_alloc, align 4
  %array_elem_ptr_0 = getelementptr i64, ptr %array_alloc, i64 1
  store i64 1, ptr %array_elem_ptr_0, align 4
  %array_elem_ptr_1 = getelementptr i64, ptr %array_alloc, i64 2
  store i64 2, ptr %array_elem_ptr_1, align 4
  %array_elem_ptr_2 = getelementptr i64, ptr %array_alloc, i64 3
  store i64 3, ptr %array_elem_ptr_2, align 4
  %array_elem_ptr_3 = getelementptr i64, ptr %array_alloc, i64 4
  store i64 4, ptr %array_elem_ptr_3, align 4
  %array_elem_ptr_4 = getelementptr i64, ptr %array_alloc, i64 5
  store i64 5, ptr %array_elem_ptr_4, align 4
  %arr = alloca ptr, align 8
  store ptr %array_alloc, ptr %arr, align 8
  %arr1 = load ptr, ptr %arr, align 8
  %array_element_ptr = getelementptr i64, ptr %arr1, i64 1
  %array_element = load i64, ptr %array_element_ptr, align 4
  %primeiro = alloca i64, align 8
  store i64 %array_element, ptr %primeiro, align 4
  %arr2 = load ptr, ptr %arr, align 8
  %array_element_ptr3 = getelementptr i64, ptr %arr2, i64 2
  %array_element4 = load i64, ptr %array_element_ptr3, align 4
  %segundo = alloca i64, align 8
  store i64 %array_element4, ptr %segundo, align 4
  %arr5 = load ptr, ptr %arr, align 8
  %array_element_ptr6 = getelementptr i64, ptr %arr5, i64 5
  %array_element7 = load i64, ptr %array_element_ptr6, align 4
  %ultimo = alloca i64, align 8
  store i64 %array_element7, ptr %ultimo, align 4
  %printf_str_call = call i64 (ptr, ...) @printf(ptr @format_str, ptr @string_literal)
  %primeiro8 = load i64, ptr %primeiro, align 4
  %printf_int_call = call i64 (ptr, ...) @printf(ptr @format_str.1, i64 %primeiro8)
  %printf_str_call9 = call i64 (ptr, ...) @printf(ptr @format_str, ptr @string_literal.2)
  %segundo10 = load i64, ptr %segundo, align 4
  %printf_int_call11 = call i64 (ptr, ...) @printf(ptr @format_str.1, i64 %segundo10)
  %printf_str_call12 = call i64 (ptr, ...) @printf(ptr @format_str, ptr @string_literal.3)
  %ultimo13 = load i64, ptr %ultimo, align 4
  %printf_int_call14 = call i64 (ptr, ...) @printf(ptr @format_str.1, i64 %ultimo13)
  ret i64 0
}

declare ptr @malloc(i64)

declare i64 @printf(ptr, ...)
