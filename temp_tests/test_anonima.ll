; ModuleID = 'program'
source_filename = "program"

@string_literal = global [12 x i8] c"Resultado: \00"
@int_format = global [5 x i8] c"%lld\00"
@format_str = global [4 x i8] c"%s\0A\00"

define i64 @main() {
entry:
  %minha_funcao = alloca ptr, align 8
  store ptr @__anon_func_1, ptr %minha_funcao, align 8
  %load_func_minha_funcao = load ptr, ptr %minha_funcao, align 8
  %call_minha_funcao = call i64 %load_func_minha_funcao(i64 5, i64 3)
  %int_str_buffer = call ptr @malloc(i64 20)
  %sprintf_call = call i64 (ptr, ptr, ...) @sprintf(ptr %int_str_buffer, ptr @int_format, i64 %call_minha_funcao)
  %left_len = call i64 @strlen(ptr @string_literal)
  %right_len = call i64 @strlen(ptr %int_str_buffer)
  %total_len = add i64 %left_len, %right_len
  %total_len_plus_one = add i64 %total_len, 1
  %concat_buffer = call ptr @malloc(i64 %total_len_plus_one)
  %strcpy_first = call ptr @strcpy(ptr %concat_buffer, ptr @string_literal)
  %strcat_second = call ptr @strcat(ptr %concat_buffer, ptr %int_str_buffer)
  %printf_call = call i64 (ptr, ...) @printf(ptr @format_str, ptr %concat_buffer)
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

declare ptr @malloc(i64)

declare i64 @sprintf(ptr, ptr, ...)

declare i64 @strlen(ptr)

declare ptr @strcpy(ptr, ptr)

declare ptr @strcat(ptr, ptr)

declare i64 @printf(ptr, ...)
