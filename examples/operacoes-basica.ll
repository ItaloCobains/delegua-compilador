; ModuleID = 'program'
source_filename = "program"

@string_literal = global [13 x i8] c"Valor de A: \00"
@int_format = global [5 x i8] c"%lld\00"
@format_str = global [4 x i8] c"%s\0A\00"
@string_literal.1 = global [13 x i8] c"Valor de B: \00"
@int_format.2 = global [5 x i8] c"%lld\00"
@format_str.3 = global [4 x i8] c"%s\0A\00"
@string_literal.4 = global [33 x i8] c"A soma dos n\C3\BAmeros \C3\A9 igual a: \00"
@int_format.5 = global [5 x i8] c"%lld\00"
@format_str.6 = global [4 x i8] c"%s\0A\00"
@string_literal.7 = global [40 x i8] c"A subtra\C3\A7\C3\A3o dos n\C3\BAmeros \C3\A9 igual a: \00"
@int_format.8 = global [5 x i8] c"%lld\00"
@format_str.9 = global [4 x i8] c"%s\0A\00"
@string_literal.10 = global [44 x i8] c"A multiplica\C3\A7\C3\A3o dos n\C3\BAmeros \C3\A9 igual a: \00"
@int_format.11 = global [5 x i8] c"%lld\00"
@format_str.12 = global [4 x i8] c"%s\0A\00"
@string_literal.13 = global [37 x i8] c"A divis\C3\A3o dos n\C3\BAmeros \C3\A9 igual a: \00"
@int_format.14 = global [5 x i8] c"%lld\00"
@format_str.15 = global [4 x i8] c"%s\0A\00"

define i64 @main() {
entry:
  %a = alloca i64, align 8
  store i64 10, ptr %a, align 4
  %b = alloca i64, align 8
  store i64 4, ptr %b, align 4
  %a1 = load i64, ptr %a, align 4
  %int_str_buffer = call ptr @malloc(i64 20)
  %sprintf_call = call i64 (ptr, ptr, ...) @sprintf(ptr %int_str_buffer, ptr @int_format, i64 %a1)
  %left_len = call i64 @strlen(ptr @string_literal)
  %right_len = call i64 @strlen(ptr %int_str_buffer)
  %total_len = add i64 %left_len, %right_len
  %total_len_plus_one = add i64 %total_len, 1
  %concat_buffer = call ptr @malloc(i64 %total_len_plus_one)
  %strcpy_first = call ptr @strcpy(ptr %concat_buffer, ptr @string_literal)
  %strcat_second = call ptr @strcat(ptr %concat_buffer, ptr %int_str_buffer)
  %printf_call = call i64 (ptr, ...) @printf(ptr @format_str, ptr %concat_buffer)
  %b2 = load i64, ptr %b, align 4
  %int_str_buffer3 = call ptr @malloc(i64 20)
  %sprintf_call4 = call i64 (ptr, ptr, ...) @sprintf(ptr %int_str_buffer3, ptr @int_format.2, i64 %b2)
  %left_len5 = call i64 @strlen(ptr @string_literal.1)
  %right_len6 = call i64 @strlen(ptr %int_str_buffer3)
  %total_len7 = add i64 %left_len5, %right_len6
  %total_len_plus_one8 = add i64 %total_len7, 1
  %concat_buffer9 = call ptr @malloc(i64 %total_len_plus_one8)
  %strcpy_first10 = call ptr @strcpy(ptr %concat_buffer9, ptr @string_literal.1)
  %strcat_second11 = call ptr @strcat(ptr %concat_buffer9, ptr %int_str_buffer3)
  %printf_call12 = call i64 (ptr, ...) @printf(ptr @format_str.3, ptr %concat_buffer9)
  %a13 = load i64, ptr %a, align 4
  %b14 = load i64, ptr %b, align 4
  %add = add i64 %a13, %b14
  %soma = alloca i64, align 8
  store i64 %add, ptr %soma, align 4
  %a15 = load i64, ptr %a, align 4
  %b16 = load i64, ptr %b, align 4
  %sub = sub i64 %a15, %b16
  %sub17 = alloca i64, align 8
  store i64 %sub, ptr %sub17, align 4
  %a18 = load i64, ptr %a, align 4
  %b19 = load i64, ptr %b, align 4
  %mul = mul i64 %a18, %b19
  %mult = alloca i64, align 8
  store i64 %mul, ptr %mult, align 4
  %a20 = load i64, ptr %a, align 4
  %b21 = load i64, ptr %b, align 4
  %div = sdiv i64 %a20, %b21
  %div22 = alloca i64, align 8
  store i64 %div, ptr %div22, align 4
  %soma23 = load i64, ptr %soma, align 4
  %int_str_buffer24 = call ptr @malloc(i64 20)
  %sprintf_call25 = call i64 (ptr, ptr, ...) @sprintf(ptr %int_str_buffer24, ptr @int_format.5, i64 %soma23)
  %left_len26 = call i64 @strlen(ptr @string_literal.4)
  %right_len27 = call i64 @strlen(ptr %int_str_buffer24)
  %total_len28 = add i64 %left_len26, %right_len27
  %total_len_plus_one29 = add i64 %total_len28, 1
  %concat_buffer30 = call ptr @malloc(i64 %total_len_plus_one29)
  %strcpy_first31 = call ptr @strcpy(ptr %concat_buffer30, ptr @string_literal.4)
  %strcat_second32 = call ptr @strcat(ptr %concat_buffer30, ptr %int_str_buffer24)
  %printf_call33 = call i64 (ptr, ...) @printf(ptr @format_str.6, ptr %concat_buffer30)
  %sub34 = load i64, ptr %sub17, align 4
  %int_str_buffer35 = call ptr @malloc(i64 20)
  %sprintf_call36 = call i64 (ptr, ptr, ...) @sprintf(ptr %int_str_buffer35, ptr @int_format.8, i64 %sub34)
  %left_len37 = call i64 @strlen(ptr @string_literal.7)
  %right_len38 = call i64 @strlen(ptr %int_str_buffer35)
  %total_len39 = add i64 %left_len37, %right_len38
  %total_len_plus_one40 = add i64 %total_len39, 1
  %concat_buffer41 = call ptr @malloc(i64 %total_len_plus_one40)
  %strcpy_first42 = call ptr @strcpy(ptr %concat_buffer41, ptr @string_literal.7)
  %strcat_second43 = call ptr @strcat(ptr %concat_buffer41, ptr %int_str_buffer35)
  %printf_call44 = call i64 (ptr, ...) @printf(ptr @format_str.9, ptr %concat_buffer41)
  %mult45 = load i64, ptr %mult, align 4
  %int_str_buffer46 = call ptr @malloc(i64 20)
  %sprintf_call47 = call i64 (ptr, ptr, ...) @sprintf(ptr %int_str_buffer46, ptr @int_format.11, i64 %mult45)
  %left_len48 = call i64 @strlen(ptr @string_literal.10)
  %right_len49 = call i64 @strlen(ptr %int_str_buffer46)
  %total_len50 = add i64 %left_len48, %right_len49
  %total_len_plus_one51 = add i64 %total_len50, 1
  %concat_buffer52 = call ptr @malloc(i64 %total_len_plus_one51)
  %strcpy_first53 = call ptr @strcpy(ptr %concat_buffer52, ptr @string_literal.10)
  %strcat_second54 = call ptr @strcat(ptr %concat_buffer52, ptr %int_str_buffer46)
  %printf_call55 = call i64 (ptr, ...) @printf(ptr @format_str.12, ptr %concat_buffer52)
  %div56 = load i64, ptr %div22, align 4
  %int_str_buffer57 = call ptr @malloc(i64 20)
  %sprintf_call58 = call i64 (ptr, ptr, ...) @sprintf(ptr %int_str_buffer57, ptr @int_format.14, i64 %div56)
  %left_len59 = call i64 @strlen(ptr @string_literal.13)
  %right_len60 = call i64 @strlen(ptr %int_str_buffer57)
  %total_len61 = add i64 %left_len59, %right_len60
  %total_len_plus_one62 = add i64 %total_len61, 1
  %concat_buffer63 = call ptr @malloc(i64 %total_len_plus_one62)
  %strcpy_first64 = call ptr @strcpy(ptr %concat_buffer63, ptr @string_literal.13)
  %strcat_second65 = call ptr @strcat(ptr %concat_buffer63, ptr %int_str_buffer57)
  %printf_call66 = call i64 (ptr, ...) @printf(ptr @format_str.15, ptr %concat_buffer63)
  ret i64 0
}

declare ptr @malloc(i64)

declare i64 @sprintf(ptr, ptr, ...)

declare i64 @strlen(ptr)

declare ptr @strcpy(ptr, ptr)

declare ptr @strcat(ptr, ptr)

declare i64 @printf(ptr, ...)
