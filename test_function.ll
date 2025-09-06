; ModuleID = 'program'
source_filename = "program"

@string_literal = global [12 x i8] c"resultado: \00"
@format_str = global [5 x i8] c"%lld\00"
@format_str.1 = global [4 x i8] c"%s\0A\00"

define i64 @main() {
entry:
  %user_call = call i64 @teste(i64 5)
  %resultado = alloca i64, align 8
  store i64 %user_call, ptr %resultado, align 4
  %resultado1 = load i64, ptr %resultado, align 4
  %int_str_buffer = call ptr @malloc(i64 20)
  %sprintf_int = call i64 (ptr, ptr, ...) @sprintf(ptr %int_str_buffer, ptr @format_str, i64 %resultado1)
  %left_len = call i64 @strlen(ptr @string_literal)
  %right_len = call i64 @strlen(ptr %int_str_buffer)
  %total_len = add i64 %left_len, %right_len
  %total_len_plus_one = add i64 %total_len, 1
  %concat_buffer = call ptr @malloc(i64 %total_len_plus_one)
  %strcpy_first = call ptr @strcpy(ptr %concat_buffer, ptr @string_literal)
  %strcat_second = call ptr @strcat(ptr %concat_buffer, ptr %int_str_buffer)
  %printf_str_call = call i64 (ptr, ...) @printf(ptr @format_str.1, ptr %concat_buffer)
  ret i64 0
}

define i64 @teste(i64 %0) {
entry:
  %n = alloca i64, align 8
  store i64 %0, ptr %n, align 4
  %n1 = load i64, ptr %n, align 4
  %add = add i64 %n1, 1
  ret i64 %add
}

declare ptr @malloc(i64)

declare i64 @sprintf(ptr, ptr, ...)

declare i64 @strlen(ptr)

declare ptr @strcpy(ptr, ptr)

declare ptr @strcat(ptr, ptr)

declare i64 @printf(ptr, ...)
