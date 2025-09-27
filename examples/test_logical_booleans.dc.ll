; ModuleID = 'program'
source_filename = "program"

@string_literal = global [21 x i8] c"Teste com booleanos:\00"
@format_str = global [4 x i8] c"%s\0A\00"
@string_literal.1 = global [22 x i8] c"verdadeiro e falso = \00"
@format_str.2 = global [5 x i8] c"%lld\00"
@string_literal.3 = global [23 x i8] c"verdadeiro ou falso = \00"
@string_literal.4 = global [27 x i8] c"verdadeiro e verdadeiro = \00"
@string_literal.5 = global [17 x i8] c"falso e falso = \00"

define i64 @main() {
entry:
  %verdade = alloca i64, align 8
  store i64 1, ptr %verdade, align 4
  %mentira = alloca i64, align 8
  store i64 0, ptr %mentira, align 4
  %verdade1 = load i64, ptr %verdade, align 4
  %mentira2 = load i64, ptr %mentira, align 4
  %and = and i64 %verdade1, %mentira2
  %and_test = alloca i64, align 8
  store i64 %and, ptr %and_test, align 4
  %verdade3 = load i64, ptr %verdade, align 4
  %mentira4 = load i64, ptr %mentira, align 4
  %or = or i64 %verdade3, %mentira4
  %or_test = alloca i64, align 8
  store i64 %or, ptr %or_test, align 4
  %verdade5 = load i64, ptr %verdade, align 4
  %verdade6 = load i64, ptr %verdade, align 4
  %and7 = and i64 %verdade5, %verdade6
  %both_true = alloca i64, align 8
  store i64 %and7, ptr %both_true, align 4
  %mentira8 = load i64, ptr %mentira, align 4
  %mentira9 = load i64, ptr %mentira, align 4
  %and10 = and i64 %mentira8, %mentira9
  %both_false = alloca i64, align 8
  store i64 %and10, ptr %both_false, align 4
  %printf_str_call = call i64 (ptr, ...) @printf(ptr @format_str, ptr @string_literal)
  %and_test11 = load i64, ptr %and_test, align 4
  %int_str_buffer = call ptr @malloc(i64 20)
  %sprintf_int = call i64 (ptr, ptr, ...) @sprintf(ptr %int_str_buffer, ptr @format_str.2, i64 %and_test11)
  %left_len = call i64 @strlen(ptr @string_literal.1)
  %right_len = call i64 @strlen(ptr %int_str_buffer)
  %total_len = add i64 %left_len, %right_len
  %total_len_plus_one = add i64 %total_len, 1
  %concat_buffer = call ptr @malloc(i64 %total_len_plus_one)
  %strcpy_first = call ptr @strcpy(ptr %concat_buffer, ptr @string_literal.1)
  %strcat_second = call ptr @strcat(ptr %concat_buffer, ptr %int_str_buffer)
  %printf_str_call12 = call i64 (ptr, ...) @printf(ptr @format_str, ptr %concat_buffer)
  %or_test13 = load i64, ptr %or_test, align 4
  %int_str_buffer14 = call ptr @malloc(i64 20)
  %sprintf_int15 = call i64 (ptr, ptr, ...) @sprintf(ptr %int_str_buffer14, ptr @format_str.2, i64 %or_test13)
  %left_len16 = call i64 @strlen(ptr @string_literal.3)
  %right_len17 = call i64 @strlen(ptr %int_str_buffer14)
  %total_len18 = add i64 %left_len16, %right_len17
  %total_len_plus_one19 = add i64 %total_len18, 1
  %concat_buffer20 = call ptr @malloc(i64 %total_len_plus_one19)
  %strcpy_first21 = call ptr @strcpy(ptr %concat_buffer20, ptr @string_literal.3)
  %strcat_second22 = call ptr @strcat(ptr %concat_buffer20, ptr %int_str_buffer14)
  %printf_str_call23 = call i64 (ptr, ...) @printf(ptr @format_str, ptr %concat_buffer20)
  %both_true24 = load i64, ptr %both_true, align 4
  %int_str_buffer25 = call ptr @malloc(i64 20)
  %sprintf_int26 = call i64 (ptr, ptr, ...) @sprintf(ptr %int_str_buffer25, ptr @format_str.2, i64 %both_true24)
  %left_len27 = call i64 @strlen(ptr @string_literal.4)
  %right_len28 = call i64 @strlen(ptr %int_str_buffer25)
  %total_len29 = add i64 %left_len27, %right_len28
  %total_len_plus_one30 = add i64 %total_len29, 1
  %concat_buffer31 = call ptr @malloc(i64 %total_len_plus_one30)
  %strcpy_first32 = call ptr @strcpy(ptr %concat_buffer31, ptr @string_literal.4)
  %strcat_second33 = call ptr @strcat(ptr %concat_buffer31, ptr %int_str_buffer25)
  %printf_str_call34 = call i64 (ptr, ...) @printf(ptr @format_str, ptr %concat_buffer31)
  %both_false35 = load i64, ptr %both_false, align 4
  %int_str_buffer36 = call ptr @malloc(i64 20)
  %sprintf_int37 = call i64 (ptr, ptr, ...) @sprintf(ptr %int_str_buffer36, ptr @format_str.2, i64 %both_false35)
  %left_len38 = call i64 @strlen(ptr @string_literal.5)
  %right_len39 = call i64 @strlen(ptr %int_str_buffer36)
  %total_len40 = add i64 %left_len38, %right_len39
  %total_len_plus_one41 = add i64 %total_len40, 1
  %concat_buffer42 = call ptr @malloc(i64 %total_len_plus_one41)
  %strcpy_first43 = call ptr @strcpy(ptr %concat_buffer42, ptr @string_literal.5)
  %strcat_second44 = call ptr @strcat(ptr %concat_buffer42, ptr %int_str_buffer36)
  %printf_str_call45 = call i64 (ptr, ...) @printf(ptr @format_str, ptr %concat_buffer42)
  ret i64 0
}

declare i64 @printf(ptr, ...)

declare ptr @malloc(i64)

declare i64 @sprintf(ptr, ptr, ...)

declare i64 @strlen(ptr)

declare ptr @strcpy(ptr, ptr)

declare ptr @strcat(ptr, ptr)
