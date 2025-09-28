; ModuleID = 'program'
source_filename = "program"

@string_literal = global [28 x i8] c"=== Programa Interativo ===\00"
@format_str = global [4 x i8] c"%s\0A\00"
@string_literal.1 = global [21 x i8] c"Qual \C3\A9 o seu nome? \00"
@format_str.2 = global [3 x i8] c"%s\00"
@format_str.3 = global [6 x i8] c"%255s\00"
@string_literal.4 = global [25 x i8] c"De que cidade voc\C3\AA \C3\A9? \00"
@string_literal.5 = global [24 x i8] c"Prazer em conhec\C3\AA-lo, \00"
@string_literal.6 = global [2 x i8] c"!\00"
@string_literal.7 = global [27 x i8] c"Que legal que voc\C3\AA \C3\A9 de \00"
@string_literal.8 = global [2 x i8] c"!\00"
@string_literal.9 = global [39 x i8] c"Voc\C3\AA gosta de programar? (sim/n\C3\A3o): \00"
@string_literal.10 = global [19 x i8] c"Sua resposta foi: \00"

define i64 @main() {
entry:
  %printf_str_call = call i64 (ptr, ...) @printf(ptr @format_str, ptr @string_literal)
  %printf_prompt = call i64 (ptr, ...) @printf(ptr @format_str.2, ptr @string_literal.1)
  %input_buffer = call ptr @malloc(i64 256)
  %scanf_call = call i64 (ptr, ...) @scanf(ptr @format_str.3, ptr %input_buffer)
  %nome = alloca ptr, align 8
  store ptr %input_buffer, ptr %nome, align 8
  %printf_prompt1 = call i64 (ptr, ...) @printf(ptr @format_str.2, ptr @string_literal.4)
  %input_buffer2 = call ptr @malloc(i64 256)
  %scanf_call3 = call i64 (ptr, ...) @scanf(ptr @format_str.3, ptr %input_buffer2)
  %cidade = alloca ptr, align 8
  store ptr %input_buffer2, ptr %cidade, align 8
  %nome4 = load ptr, ptr %nome, align 8
  %left_len = call i64 @strlen(ptr @string_literal.5)
  %right_len = call i64 @strlen(ptr %nome4)
  %total_len = add i64 %left_len, %right_len
  %total_len_plus_one = add i64 %total_len, 1
  %concat_buffer = call ptr @malloc(i64 %total_len_plus_one)
  %strcpy_first = call ptr @strcpy(ptr %concat_buffer, ptr @string_literal.5)
  %strcat_second = call ptr @strcat(ptr %concat_buffer, ptr %nome4)
  %left_len5 = call i64 @strlen(ptr %concat_buffer)
  %right_len6 = call i64 @strlen(ptr @string_literal.6)
  %total_len7 = add i64 %left_len5, %right_len6
  %total_len_plus_one8 = add i64 %total_len7, 1
  %concat_buffer9 = call ptr @malloc(i64 %total_len_plus_one8)
  %strcpy_first10 = call ptr @strcpy(ptr %concat_buffer9, ptr %concat_buffer)
  %strcat_second11 = call ptr @strcat(ptr %concat_buffer9, ptr @string_literal.6)
  %printf_str_call12 = call i64 (ptr, ...) @printf(ptr @format_str, ptr %concat_buffer9)
  %cidade13 = load ptr, ptr %cidade, align 8
  %left_len14 = call i64 @strlen(ptr @string_literal.7)
  %right_len15 = call i64 @strlen(ptr %cidade13)
  %total_len16 = add i64 %left_len14, %right_len15
  %total_len_plus_one17 = add i64 %total_len16, 1
  %concat_buffer18 = call ptr @malloc(i64 %total_len_plus_one17)
  %strcpy_first19 = call ptr @strcpy(ptr %concat_buffer18, ptr @string_literal.7)
  %strcat_second20 = call ptr @strcat(ptr %concat_buffer18, ptr %cidade13)
  %left_len21 = call i64 @strlen(ptr %concat_buffer18)
  %right_len22 = call i64 @strlen(ptr @string_literal.8)
  %total_len23 = add i64 %left_len21, %right_len22
  %total_len_plus_one24 = add i64 %total_len23, 1
  %concat_buffer25 = call ptr @malloc(i64 %total_len_plus_one24)
  %strcpy_first26 = call ptr @strcpy(ptr %concat_buffer25, ptr %concat_buffer18)
  %strcat_second27 = call ptr @strcat(ptr %concat_buffer25, ptr @string_literal.8)
  %printf_str_call28 = call i64 (ptr, ...) @printf(ptr @format_str, ptr %concat_buffer25)
  %printf_prompt29 = call i64 (ptr, ...) @printf(ptr @format_str.2, ptr @string_literal.9)
  %input_buffer30 = call ptr @malloc(i64 256)
  %scanf_call31 = call i64 (ptr, ...) @scanf(ptr @format_str.3, ptr %input_buffer30)
  %resposta = alloca ptr, align 8
  store ptr %input_buffer30, ptr %resposta, align 8
  %resposta32 = load ptr, ptr %resposta, align 8
  %left_len33 = call i64 @strlen(ptr @string_literal.10)
  %right_len34 = call i64 @strlen(ptr %resposta32)
  %total_len35 = add i64 %left_len33, %right_len34
  %total_len_plus_one36 = add i64 %total_len35, 1
  %concat_buffer37 = call ptr @malloc(i64 %total_len_plus_one36)
  %strcpy_first38 = call ptr @strcpy(ptr %concat_buffer37, ptr @string_literal.10)
  %strcat_second39 = call ptr @strcat(ptr %concat_buffer37, ptr %resposta32)
  %printf_str_call40 = call i64 (ptr, ...) @printf(ptr @format_str, ptr %concat_buffer37)
  ret i64 0
}

declare i64 @printf(ptr, ...)

declare i64 @scanf(ptr, ...)

declare ptr @malloc(i64)

declare i64 @strlen(ptr)

declare ptr @strcpy(ptr, ptr)

declare ptr @strcat(ptr, ptr)
