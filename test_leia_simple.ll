; ModuleID = 'program'
source_filename = "program"

@string_literal = global [24 x i8] c"Teste da fun\C3\A7\C3\A3o leia:\00"
@format_str = global [4 x i8] c"%s\0A\00"
@string_literal.1 = global [18 x i8] c"Digite seu nome: \00"
@format_str.2 = global [3 x i8] c"%s\00"
@format_str.3 = global [6 x i8] c"%255s\00"
@string_literal.4 = global [6 x i8] c"Ol\C3\A1 \00"
@string_literal.5 = global [2 x i8] c"!\00"
@string_literal.6 = global [19 x i8] c"Digite sua idade: \00"
@string_literal.7 = global [16 x i8] c"Voc\C3\AA digitou: \00"

define i64 @main() {
entry:
  %printf_str_call = call i64 (ptr, ...) @printf(ptr @format_str, ptr @string_literal)
  %printf_prompt = call i64 (ptr, ...) @printf(ptr @format_str.2, ptr @string_literal.1)
  %input_buffer = call ptr @malloc(i64 256)
  %scanf_call = call i64 (ptr, ...) @scanf(ptr @format_str.3, ptr %input_buffer)
  %nome = alloca ptr, align 8
  store ptr %input_buffer, ptr %nome, align 8
  %nome1 = load ptr, ptr %nome, align 8
  %left_len = call i64 @strlen(ptr @string_literal.4)
  %right_len = call i64 @strlen(ptr %nome1)
  %total_len = add i64 %left_len, %right_len
  %total_len_plus_one = add i64 %total_len, 1
  %concat_buffer = call ptr @malloc(i64 %total_len_plus_one)
  %strcpy_first = call ptr @strcpy(ptr %concat_buffer, ptr @string_literal.4)
  %strcat_second = call ptr @strcat(ptr %concat_buffer, ptr %nome1)
  %left_len2 = call i64 @strlen(ptr %concat_buffer)
  %right_len3 = call i64 @strlen(ptr @string_literal.5)
  %total_len4 = add i64 %left_len2, %right_len3
  %total_len_plus_one5 = add i64 %total_len4, 1
  %concat_buffer6 = call ptr @malloc(i64 %total_len_plus_one5)
  %strcpy_first7 = call ptr @strcpy(ptr %concat_buffer6, ptr %concat_buffer)
  %strcat_second8 = call ptr @strcat(ptr %concat_buffer6, ptr @string_literal.5)
  %printf_str_call9 = call i64 (ptr, ...) @printf(ptr @format_str, ptr %concat_buffer6)
  %printf_prompt10 = call i64 (ptr, ...) @printf(ptr @format_str.2, ptr @string_literal.6)
  %input_buffer11 = call ptr @malloc(i64 256)
  %scanf_call12 = call i64 (ptr, ...) @scanf(ptr @format_str.3, ptr %input_buffer11)
  %idade_str = alloca ptr, align 8
  store ptr %input_buffer11, ptr %idade_str, align 8
  %idade_str13 = load ptr, ptr %idade_str, align 8
  %left_len14 = call i64 @strlen(ptr @string_literal.7)
  %right_len15 = call i64 @strlen(ptr %idade_str13)
  %total_len16 = add i64 %left_len14, %right_len15
  %total_len_plus_one17 = add i64 %total_len16, 1
  %concat_buffer18 = call ptr @malloc(i64 %total_len_plus_one17)
  %strcpy_first19 = call ptr @strcpy(ptr %concat_buffer18, ptr @string_literal.7)
  %strcat_second20 = call ptr @strcat(ptr %concat_buffer18, ptr %idade_str13)
  %printf_str_call21 = call i64 (ptr, ...) @printf(ptr @format_str, ptr %concat_buffer18)
  ret i64 0
}

declare i64 @printf(ptr, ...)

declare i64 @scanf(ptr, ...)

declare ptr @malloc(i64)

declare i64 @strlen(ptr)

declare ptr @strcpy(ptr, ptr)

declare ptr @strcat(ptr, ptr)
