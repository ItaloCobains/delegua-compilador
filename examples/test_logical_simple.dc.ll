; ModuleID = 'program'
source_filename = "program"

@string_literal = global [30 x i8] c"Teste de operadores l\C3\B3gicos:\00"
@format_str = global [4 x i8] c"%s\0A\00"
@string_literal.1 = global [5 x i8] c"a = \00"
@format_str.2 = global [5 x i8] c"%lld\00"
@string_literal.3 = global [5 x i8] c"b = \00"
@string_literal.4 = global [9 x i8] c"a e b = \00"
@string_literal.5 = global [10 x i8] c"a ou b = \00"

define i64 @main() {
entry:
  %a = alloca i64, align 8
  store i64 1, ptr %a, align 4
  %b = alloca i64, align 8
  store i64 0, ptr %b, align 4
  %a1 = load i64, ptr %a, align 4
  %b2 = load i64, ptr %b, align 4
  %and = and i64 %a1, %b2
  %resultado_e = alloca i64, align 8
  store i64 %and, ptr %resultado_e, align 4
  %a3 = load i64, ptr %a, align 4
  %b4 = load i64, ptr %b, align 4
  %or = or i64 %a3, %b4
  %resultado_ou = alloca i64, align 8
  store i64 %or, ptr %resultado_ou, align 4
  %printf_str_call = call i64 (ptr, ...) @printf(ptr @format_str, ptr @string_literal)
  %a5 = load i64, ptr %a, align 4
  %int_str_buffer = call ptr @malloc(i64 20)
  %sprintf_int = call i64 (ptr, ptr, ...) @sprintf(ptr %int_str_buffer, ptr @format_str.2, i64 %a5)
  %left_len = call i64 @strlen(ptr @string_literal.1)
  %right_len = call i64 @strlen(ptr %int_str_buffer)
  %total_len = add i64 %left_len, %right_len
  %total_len_plus_one = add i64 %total_len, 1
  %concat_buffer = call ptr @malloc(i64 %total_len_plus_one)
  %strcpy_first = call ptr @strcpy(ptr %concat_buffer, ptr @string_literal.1)
  %strcat_second = call ptr @strcat(ptr %concat_buffer, ptr %int_str_buffer)
  %printf_str_call6 = call i64 (ptr, ...) @printf(ptr @format_str, ptr %concat_buffer)
  %b7 = load i64, ptr %b, align 4
  %int_str_buffer8 = call ptr @malloc(i64 20)
  %sprintf_int9 = call i64 (ptr, ptr, ...) @sprintf(ptr %int_str_buffer8, ptr @format_str.2, i64 %b7)
  %left_len10 = call i64 @strlen(ptr @string_literal.3)
  %right_len11 = call i64 @strlen(ptr %int_str_buffer8)
  %total_len12 = add i64 %left_len10, %right_len11
  %total_len_plus_one13 = add i64 %total_len12, 1
  %concat_buffer14 = call ptr @malloc(i64 %total_len_plus_one13)
  %strcpy_first15 = call ptr @strcpy(ptr %concat_buffer14, ptr @string_literal.3)
  %strcat_second16 = call ptr @strcat(ptr %concat_buffer14, ptr %int_str_buffer8)
  %printf_str_call17 = call i64 (ptr, ...) @printf(ptr @format_str, ptr %concat_buffer14)
  %resultado_e18 = load i64, ptr %resultado_e, align 4
  %int_str_buffer19 = call ptr @malloc(i64 20)
  %sprintf_int20 = call i64 (ptr, ptr, ...) @sprintf(ptr %int_str_buffer19, ptr @format_str.2, i64 %resultado_e18)
  %left_len21 = call i64 @strlen(ptr @string_literal.4)
  %right_len22 = call i64 @strlen(ptr %int_str_buffer19)
  %total_len23 = add i64 %left_len21, %right_len22
  %total_len_plus_one24 = add i64 %total_len23, 1
  %concat_buffer25 = call ptr @malloc(i64 %total_len_plus_one24)
  %strcpy_first26 = call ptr @strcpy(ptr %concat_buffer25, ptr @string_literal.4)
  %strcat_second27 = call ptr @strcat(ptr %concat_buffer25, ptr %int_str_buffer19)
  %printf_str_call28 = call i64 (ptr, ...) @printf(ptr @format_str, ptr %concat_buffer25)
  %resultado_ou29 = load i64, ptr %resultado_ou, align 4
  %int_str_buffer30 = call ptr @malloc(i64 20)
  %sprintf_int31 = call i64 (ptr, ptr, ...) @sprintf(ptr %int_str_buffer30, ptr @format_str.2, i64 %resultado_ou29)
  %left_len32 = call i64 @strlen(ptr @string_literal.5)
  %right_len33 = call i64 @strlen(ptr %int_str_buffer30)
  %total_len34 = add i64 %left_len32, %right_len33
  %total_len_plus_one35 = add i64 %total_len34, 1
  %concat_buffer36 = call ptr @malloc(i64 %total_len_plus_one35)
  %strcpy_first37 = call ptr @strcpy(ptr %concat_buffer36, ptr @string_literal.5)
  %strcat_second38 = call ptr @strcat(ptr %concat_buffer36, ptr %int_str_buffer30)
  %printf_str_call39 = call i64 (ptr, ...) @printf(ptr @format_str, ptr %concat_buffer36)
  ret i64 0
}

declare i64 @printf(ptr, ...)

declare ptr @malloc(i64)

declare i64 @sprintf(ptr, ptr, ...)

declare i64 @strlen(ptr)

declare ptr @strcpy(ptr, ptr)

declare ptr @strcat(ptr, ptr)
