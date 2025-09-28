; ModuleID = 'program'
source_filename = "program"

@string_literal = global [23 x i8] c"Elemento no indice 0: \00"
@format_str = global [4 x i8] c"%s\0A\00"
@format_str.1 = global [6 x i8] c"%lld\0A\00"
@string_literal.2 = global [23 x i8] c"Elemento no indice 1: \00"
@string_literal.3 = global [23 x i8] c"Elemento no indice 2: \00"
@string_literal.4 = global [23 x i8] c"Elemento no indice 4: \00"

define i64 @main() {
entry:
  %array_alloc = call ptr @malloc(i64 48)
  store i64 5, ptr %array_alloc, align 4
  %array_elem_ptr_0 = getelementptr i64, ptr %array_alloc, i64 1
  store i64 10, ptr %array_elem_ptr_0, align 4
  %array_elem_ptr_1 = getelementptr i64, ptr %array_alloc, i64 2
  store i64 20, ptr %array_elem_ptr_1, align 4
  %array_elem_ptr_2 = getelementptr i64, ptr %array_alloc, i64 3
  store i64 30, ptr %array_elem_ptr_2, align 4
  %array_elem_ptr_3 = getelementptr i64, ptr %array_alloc, i64 4
  store i64 40, ptr %array_elem_ptr_3, align 4
  %array_elem_ptr_4 = getelementptr i64, ptr %array_alloc, i64 5
  store i64 50, ptr %array_elem_ptr_4, align 4
  %numeros = alloca ptr, align 8
  store ptr %array_alloc, ptr %numeros, align 8
  %array_alloc1 = call ptr @malloc(i64 40)
  store i64 4, ptr %array_alloc1, align 4
  %array_elem_ptr_02 = getelementptr i64, ptr %array_alloc1, i64 1
  store i64 0, ptr %array_elem_ptr_02, align 4
  %array_elem_ptr_13 = getelementptr i64, ptr %array_alloc1, i64 2
  store i64 1, ptr %array_elem_ptr_13, align 4
  %array_elem_ptr_24 = getelementptr i64, ptr %array_alloc1, i64 3
  store i64 2, ptr %array_elem_ptr_24, align 4
  %array_elem_ptr_35 = getelementptr i64, ptr %array_alloc1, i64 4
  store i64 4, ptr %array_elem_ptr_35, align 4
  %indices = alloca ptr, align 8
  store ptr %array_alloc1, ptr %indices, align 8
  %printf_str_call = call i64 (ptr, ...) @printf(ptr @format_str, ptr @string_literal)
  %numeros6 = load ptr, ptr %numeros, align 8
  %indices7 = load ptr, ptr %indices, align 8
  %array_element_ptr = getelementptr i64, ptr %indices7, i64 1
  %array_element = load i64, ptr %array_element_ptr, align 4
  %adjusted_index = add i64 %array_element, 1
  %array_element_ptr8 = getelementptr i64, ptr %numeros6, i64 %adjusted_index
  %array_element9 = load i64, ptr %array_element_ptr8, align 4
  %printf_int_call = call i64 (ptr, ...) @printf(ptr @format_str.1, i64 %array_element9)
  %printf_str_call10 = call i64 (ptr, ...) @printf(ptr @format_str, ptr @string_literal.2)
  %numeros11 = load ptr, ptr %numeros, align 8
  %indices12 = load ptr, ptr %indices, align 8
  %array_element_ptr13 = getelementptr i64, ptr %indices12, i64 2
  %array_element14 = load i64, ptr %array_element_ptr13, align 4
  %adjusted_index15 = add i64 %array_element14, 1
  %array_element_ptr16 = getelementptr i64, ptr %numeros11, i64 %adjusted_index15
  %array_element17 = load i64, ptr %array_element_ptr16, align 4
  %printf_int_call18 = call i64 (ptr, ...) @printf(ptr @format_str.1, i64 %array_element17)
  %printf_str_call19 = call i64 (ptr, ...) @printf(ptr @format_str, ptr @string_literal.3)
  %numeros20 = load ptr, ptr %numeros, align 8
  %indices21 = load ptr, ptr %indices, align 8
  %array_element_ptr22 = getelementptr i64, ptr %indices21, i64 3
  %array_element23 = load i64, ptr %array_element_ptr22, align 4
  %adjusted_index24 = add i64 %array_element23, 1
  %array_element_ptr25 = getelementptr i64, ptr %numeros20, i64 %adjusted_index24
  %array_element26 = load i64, ptr %array_element_ptr25, align 4
  %printf_int_call27 = call i64 (ptr, ...) @printf(ptr @format_str.1, i64 %array_element26)
  %printf_str_call28 = call i64 (ptr, ...) @printf(ptr @format_str, ptr @string_literal.4)
  %numeros29 = load ptr, ptr %numeros, align 8
  %indices30 = load ptr, ptr %indices, align 8
  %array_element_ptr31 = getelementptr i64, ptr %indices30, i64 4
  %array_element32 = load i64, ptr %array_element_ptr31, align 4
  %adjusted_index33 = add i64 %array_element32, 1
  %array_element_ptr34 = getelementptr i64, ptr %numeros29, i64 %adjusted_index33
  %array_element35 = load i64, ptr %array_element_ptr34, align 4
  %printf_int_call36 = call i64 (ptr, ...) @printf(ptr @format_str.1, i64 %array_element35)
  ret i64 0
}

declare ptr @malloc(i64)

declare i64 @printf(ptr, ...)
