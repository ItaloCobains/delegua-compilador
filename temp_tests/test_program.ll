; ModuleID = 'program'
source_filename = "program"

@string_literal = global [12 x i8] c"A soma \C3\A9: \00"
@int_format = global [5 x i8] c"%lld\00"
@format_str = global [4 x i8] c"%s\0A\00"
@string_literal.1 = global [8 x i8] c"Delegua\00"
@string_literal.2 = global [6 x i8] c"Ol\C3\A1 \00"
@string_literal.3 = global [2 x i8] c"!\00"
@format_str.4 = global [4 x i8] c"%s\0A\00"

define i64 @main() {
entry:
  %x = alloca i64, align 8
  store i64 10, ptr %x, align 4
  %y = alloca i64, align 8
  store i64 5, ptr %y, align 4
  %x1 = load i64, ptr %x, align 4
  %y2 = load i64, ptr %y, align 4
  %add = add i64 %x1, %y2
  %soma = alloca i64, align 8
  store i64 %add, ptr %soma, align 4
  %soma3 = load i64, ptr %soma, align 4
  %int_str_buffer = call ptr @malloc(i64 20)
  %sprintf_call = call i64 (ptr, ptr, ...) @sprintf(ptr %int_str_buffer, ptr @int_format, i64 %soma3)
  %left_len = call i64 @strlen(ptr @string_literal)
  %right_len = call i64 @strlen(ptr %int_str_buffer)
  %total_len = add i64 %left_len, %right_len
  %total_len_plus_one = add i64 %total_len, 1
  %concat_buffer = call ptr @malloc(i64 %total_len_plus_one)
  %strcpy_first = call ptr @strcpy(ptr %concat_buffer, ptr @string_literal)
  %strcat_second = call ptr @strcat(ptr %concat_buffer, ptr %int_str_buffer)
  %printf_call = call i64 (ptr, ...) @printf(ptr @format_str, ptr %concat_buffer)
  %nome = alloca ptr, align 8
  store ptr @string_literal.1, ptr %nome, align 8
  %nome4 = load ptr, ptr %nome, align 8
  %left_len5 = call i64 @strlen(ptr @string_literal.2)
  %right_len6 = call i64 @strlen(ptr %nome4)
  %total_len7 = add i64 %left_len5, %right_len6
  %total_len_plus_one8 = add i64 %total_len7, 1
  %concat_buffer9 = call ptr @malloc(i64 %total_len_plus_one8)
  %strcpy_first10 = call ptr @strcpy(ptr %concat_buffer9, ptr @string_literal.2)
  %strcat_second11 = call ptr @strcat(ptr %concat_buffer9, ptr %nome4)
  %left_len12 = call i64 @strlen(ptr %concat_buffer9)
  %right_len13 = call i64 @strlen(ptr @string_literal.3)
  %total_len14 = add i64 %left_len12, %right_len13
  %total_len_plus_one15 = add i64 %total_len14, 1
  %concat_buffer16 = call ptr @malloc(i64 %total_len_plus_one15)
  %strcpy_first17 = call ptr @strcpy(ptr %concat_buffer16, ptr %concat_buffer9)
  %strcat_second18 = call ptr @strcat(ptr %concat_buffer16, ptr @string_literal.3)
  %printf_call19 = call i64 (ptr, ...) @printf(ptr @format_str.4, ptr %concat_buffer16)
  ret i64 0
}

declare ptr @malloc(i64)

declare i64 @sprintf(ptr, ptr, ...)

declare i64 @strlen(ptr)

declare ptr @strcpy(ptr, ptr)

declare ptr @strcat(ptr, ptr)

declare i64 @printf(ptr, ...)
