; ModuleID = 'program'
source_filename = "program"

@string_literal = global [6 x i8] c"hello\00"
@format_str = global [4 x i8] c"%s\0A\00"

define i64 @main() {
entry:
  %texto = alloca ptr, align 8
  store ptr @string_literal, ptr %texto, align 8
  %texto1 = load ptr, ptr %texto, align 8
  %strlen_call = call i64 @strlen(ptr %texto1)
  %buffer_size = add i64 %strlen_call, 1
  %upper_buffer = call ptr @malloc(i64 %buffer_size)
  %strcpy_call = call ptr @strcpy(ptr %upper_buffer, ptr %texto1)
  %upper_counter = alloca i64, align 8
  store i64 0, ptr %upper_counter, align 4
  br label %upper_loop

upper_loop:                                       ; preds = %upper_body, %entry
  %load_counter = load i64, ptr %upper_counter, align 4
  %is_end = icmp eq i64 %load_counter, %strlen_call
  br i1 %is_end, label %upper_done, label %upper_body

upper_body:                                       ; preds = %upper_loop
  %char_ptr = getelementptr i8, ptr %upper_buffer, i64 %load_counter
  %char_val = load i8, ptr %char_ptr, align 1
  %char_ext = zext i8 %char_val to i64
  %is_ge_a = icmp sge i64 %char_ext, 97
  %is_le_z = icmp sle i64 %char_ext, 122
  %is_lowercase = and i1 %is_ge_a, %is_le_z
  %upper_char = sub i64 %char_ext, 32
  %final_char = select i1 %is_lowercase, i64 %upper_char, i64 %char_ext
  %final_char_i8 = trunc i64 %final_char to i8
  store i8 %final_char_i8, ptr %char_ptr, align 1
  %increment = add i64 %load_counter, 1
  store i64 %increment, ptr %upper_counter, align 4
  br label %upper_loop

upper_done:                                       ; preds = %upper_loop
  %resultado = alloca ptr, align 8
  store ptr %upper_buffer, ptr %resultado, align 8
  %resultado2 = load ptr, ptr %resultado, align 8
  %printf_str_call = call i64 (ptr, ...) @printf(ptr @format_str, ptr %resultado2)
  ret i64 0
}

declare ptr @malloc(i64)

declare i64 @strlen(ptr)

declare ptr @strcpy(ptr, ptr)

declare i64 @printf(ptr, ...)
