; ModuleID = 'program'
source_filename = "program"

@string_literal = global [14 x i8] c"Soma direta: \00"
@int_format = global [5 x i8] c"%lld\00"
@format_str = global [4 x i8] c"%s\0A\00"
@string_literal.1 = global [25 x i8] c"Multiplica\C3\A7\C3\A3o direta: \00"
@int_format.2 = global [5 x i8] c"%lld\00"
@format_str.3 = global [4 x i8] c"%s\0A\00"
@string_literal.4 = global [17 x i8] c"Soma novamente: \00"
@int_format.5 = global [5 x i8] c"%lld\00"
@format_str.6 = global [4 x i8] c"%s\0A\00"

define i64 @main() {
entry:
  %soma = alloca ptr, align 8
  store ptr @__anon_func_1, ptr %soma, align 8
  %multiplicar = alloca ptr, align 8
  store ptr @__anon_func_2, ptr %multiplicar, align 8
  %load_func_soma = load ptr, ptr %soma, align 8
  %call_soma = call i64 %load_func_soma(i64 10, i64 5)
  %int_str_buffer = call ptr @malloc(i64 20)
  %sprintf_call = call i64 (ptr, ptr, ...) @sprintf(ptr %int_str_buffer, ptr @int_format, i64 %call_soma)
  %left_len = call i64 @strlen(ptr @string_literal)
  %right_len = call i64 @strlen(ptr %int_str_buffer)
  %total_len = add i64 %left_len, %right_len
  %total_len_plus_one = add i64 %total_len, 1
  %concat_buffer = call ptr @malloc(i64 %total_len_plus_one)
  %strcpy_first = call ptr @strcpy(ptr %concat_buffer, ptr @string_literal)
  %strcat_second = call ptr @strcat(ptr %concat_buffer, ptr %int_str_buffer)
  %printf_call = call i64 (ptr, ...) @printf(ptr @format_str, ptr %concat_buffer)
  %load_func_multiplicar = load ptr, ptr %multiplicar, align 8
  %call_multiplicar = call i64 %load_func_multiplicar(i64 10, i64 5)
  %int_str_buffer1 = call ptr @malloc(i64 20)
  %sprintf_call2 = call i64 (ptr, ptr, ...) @sprintf(ptr %int_str_buffer1, ptr @int_format.2, i64 %call_multiplicar)
  %left_len3 = call i64 @strlen(ptr @string_literal.1)
  %right_len4 = call i64 @strlen(ptr %int_str_buffer1)
  %total_len5 = add i64 %left_len3, %right_len4
  %total_len_plus_one6 = add i64 %total_len5, 1
  %concat_buffer7 = call ptr @malloc(i64 %total_len_plus_one6)
  %strcpy_first8 = call ptr @strcpy(ptr %concat_buffer7, ptr @string_literal.1)
  %strcat_second9 = call ptr @strcat(ptr %concat_buffer7, ptr %int_str_buffer1)
  %printf_call10 = call i64 (ptr, ...) @printf(ptr @format_str.3, ptr %concat_buffer7)
  %load_func_soma11 = load ptr, ptr %soma, align 8
  %call_soma12 = call i64 %load_func_soma11(i64 20, i64 3)
  %int_str_buffer13 = call ptr @malloc(i64 20)
  %sprintf_call14 = call i64 (ptr, ptr, ...) @sprintf(ptr %int_str_buffer13, ptr @int_format.5, i64 %call_soma12)
  %left_len15 = call i64 @strlen(ptr @string_literal.4)
  %right_len16 = call i64 @strlen(ptr %int_str_buffer13)
  %total_len17 = add i64 %left_len15, %right_len16
  %total_len_plus_one18 = add i64 %total_len17, 1
  %concat_buffer19 = call ptr @malloc(i64 %total_len_plus_one18)
  %strcpy_first20 = call ptr @strcpy(ptr %concat_buffer19, ptr @string_literal.4)
  %strcat_second21 = call ptr @strcat(ptr %concat_buffer19, ptr %int_str_buffer13)
  %printf_call22 = call i64 (ptr, ...) @printf(ptr @format_str.6, ptr %concat_buffer19)
  ret i64 0
}

define i64 @__anon_func_1(i64 %0, i64 %1) {
entry:
  %a = alloca i64, align 8
  store i64 %0, ptr %a, align 4
  %b = alloca i64, align 8
  store i64 %1, ptr %b, align 4
  %a1 = load i64, ptr %a, align 4
  %b2 = load i64, ptr %b, align 4
  %add = add i64 %a1, %b2
  ret i64 %add
}

define i64 @__anon_func_2(i64 %0, i64 %1) {
entry:
  %a = alloca i64, align 8
  store i64 %0, ptr %a, align 4
  %b = alloca i64, align 8
  store i64 %1, ptr %b, align 4
  %a1 = load i64, ptr %a, align 4
  %b2 = load i64, ptr %b, align 4
  %mul = mul i64 %a1, %b2
  ret i64 %mul
}

declare ptr @malloc(i64)

declare i64 @sprintf(ptr, ptr, ...)

declare i64 @strlen(ptr)

declare ptr @strcpy(ptr, ptr)

declare ptr @strcat(ptr, ptr)

declare i64 @printf(ptr, ...)
