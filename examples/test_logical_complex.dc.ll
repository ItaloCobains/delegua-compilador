; ModuleID = 'program'
source_filename = "program"

@string_literal = global [23 x i8] c"Teste de preced\C3\AAncia:\00"
@format_str = global [4 x i8] c"%s\0A\00"
@string_literal.1 = global [5 x i8] c"x = \00"
@format_str.2 = global [5 x i8] c"%lld\00"
@string_literal.3 = global [7 x i8] c", y = \00"
@string_literal.4 = global [7 x i8] c", z = \00"
@string_literal.5 = global [14 x i8] c"x e y ou z = \00"
@string_literal.6 = global [14 x i8] c"x ou y e z = \00"
@string_literal.7 = global [16 x i8] c"x e (y ou z) = \00"

define i64 @main() {
entry:
  %x = alloca i64, align 8
  store i64 1, ptr %x, align 4
  %y = alloca i64, align 8
  store i64 0, ptr %y, align 4
  %z = alloca i64, align 8
  store i64 1, ptr %z, align 4
  %x1 = load i64, ptr %x, align 4
  %y2 = load i64, ptr %y, align 4
  %and = and i64 %x1, %y2
  %z3 = load i64, ptr %z, align 4
  %or = or i64 %and, %z3
  %teste1 = alloca i64, align 8
  store i64 %or, ptr %teste1, align 4
  %x4 = load i64, ptr %x, align 4
  %y5 = load i64, ptr %y, align 4
  %z6 = load i64, ptr %z, align 4
  %and7 = and i64 %y5, %z6
  %or8 = or i64 %x4, %and7
  %teste2 = alloca i64, align 8
  store i64 %or8, ptr %teste2, align 4
  %x9 = load i64, ptr %x, align 4
  %y10 = load i64, ptr %y, align 4
  %z11 = load i64, ptr %z, align 4
  %or12 = or i64 %y10, %z11
  %and13 = and i64 %x9, %or12
  %teste3 = alloca i64, align 8
  store i64 %and13, ptr %teste3, align 4
  %printf_str_call = call i64 (ptr, ...) @printf(ptr @format_str, ptr @string_literal)
  %x14 = load i64, ptr %x, align 4
  %int_str_buffer = call ptr @malloc(i64 20)
  %sprintf_int = call i64 (ptr, ptr, ...) @sprintf(ptr %int_str_buffer, ptr @format_str.2, i64 %x14)
  %left_len = call i64 @strlen(ptr @string_literal.1)
  %right_len = call i64 @strlen(ptr %int_str_buffer)
  %total_len = add i64 %left_len, %right_len
  %total_len_plus_one = add i64 %total_len, 1
  %concat_buffer = call ptr @malloc(i64 %total_len_plus_one)
  %strcpy_first = call ptr @strcpy(ptr %concat_buffer, ptr @string_literal.1)
  %strcat_second = call ptr @strcat(ptr %concat_buffer, ptr %int_str_buffer)
  %left_len15 = call i64 @strlen(ptr %concat_buffer)
  %right_len16 = call i64 @strlen(ptr @string_literal.3)
  %total_len17 = add i64 %left_len15, %right_len16
  %total_len_plus_one18 = add i64 %total_len17, 1
  %concat_buffer19 = call ptr @malloc(i64 %total_len_plus_one18)
  %strcpy_first20 = call ptr @strcpy(ptr %concat_buffer19, ptr %concat_buffer)
  %strcat_second21 = call ptr @strcat(ptr %concat_buffer19, ptr @string_literal.3)
  %y22 = load i64, ptr %y, align 4
  %int_str_buffer23 = call ptr @malloc(i64 20)
  %sprintf_int24 = call i64 (ptr, ptr, ...) @sprintf(ptr %int_str_buffer23, ptr @format_str.2, i64 %y22)
  %left_len25 = call i64 @strlen(ptr %concat_buffer19)
  %right_len26 = call i64 @strlen(ptr %int_str_buffer23)
  %total_len27 = add i64 %left_len25, %right_len26
  %total_len_plus_one28 = add i64 %total_len27, 1
  %concat_buffer29 = call ptr @malloc(i64 %total_len_plus_one28)
  %strcpy_first30 = call ptr @strcpy(ptr %concat_buffer29, ptr %concat_buffer19)
  %strcat_second31 = call ptr @strcat(ptr %concat_buffer29, ptr %int_str_buffer23)
  %left_len32 = call i64 @strlen(ptr %concat_buffer29)
  %right_len33 = call i64 @strlen(ptr @string_literal.4)
  %total_len34 = add i64 %left_len32, %right_len33
  %total_len_plus_one35 = add i64 %total_len34, 1
  %concat_buffer36 = call ptr @malloc(i64 %total_len_plus_one35)
  %strcpy_first37 = call ptr @strcpy(ptr %concat_buffer36, ptr %concat_buffer29)
  %strcat_second38 = call ptr @strcat(ptr %concat_buffer36, ptr @string_literal.4)
  %z39 = load i64, ptr %z, align 4
  %int_str_buffer40 = call ptr @malloc(i64 20)
  %sprintf_int41 = call i64 (ptr, ptr, ...) @sprintf(ptr %int_str_buffer40, ptr @format_str.2, i64 %z39)
  %left_len42 = call i64 @strlen(ptr %concat_buffer36)
  %right_len43 = call i64 @strlen(ptr %int_str_buffer40)
  %total_len44 = add i64 %left_len42, %right_len43
  %total_len_plus_one45 = add i64 %total_len44, 1
  %concat_buffer46 = call ptr @malloc(i64 %total_len_plus_one45)
  %strcpy_first47 = call ptr @strcpy(ptr %concat_buffer46, ptr %concat_buffer36)
  %strcat_second48 = call ptr @strcat(ptr %concat_buffer46, ptr %int_str_buffer40)
  %printf_str_call49 = call i64 (ptr, ...) @printf(ptr @format_str, ptr %concat_buffer46)
  %teste150 = load i64, ptr %teste1, align 4
  %int_str_buffer51 = call ptr @malloc(i64 20)
  %sprintf_int52 = call i64 (ptr, ptr, ...) @sprintf(ptr %int_str_buffer51, ptr @format_str.2, i64 %teste150)
  %left_len53 = call i64 @strlen(ptr @string_literal.5)
  %right_len54 = call i64 @strlen(ptr %int_str_buffer51)
  %total_len55 = add i64 %left_len53, %right_len54
  %total_len_plus_one56 = add i64 %total_len55, 1
  %concat_buffer57 = call ptr @malloc(i64 %total_len_plus_one56)
  %strcpy_first58 = call ptr @strcpy(ptr %concat_buffer57, ptr @string_literal.5)
  %strcat_second59 = call ptr @strcat(ptr %concat_buffer57, ptr %int_str_buffer51)
  %printf_str_call60 = call i64 (ptr, ...) @printf(ptr @format_str, ptr %concat_buffer57)
  %teste261 = load i64, ptr %teste2, align 4
  %int_str_buffer62 = call ptr @malloc(i64 20)
  %sprintf_int63 = call i64 (ptr, ptr, ...) @sprintf(ptr %int_str_buffer62, ptr @format_str.2, i64 %teste261)
  %left_len64 = call i64 @strlen(ptr @string_literal.6)
  %right_len65 = call i64 @strlen(ptr %int_str_buffer62)
  %total_len66 = add i64 %left_len64, %right_len65
  %total_len_plus_one67 = add i64 %total_len66, 1
  %concat_buffer68 = call ptr @malloc(i64 %total_len_plus_one67)
  %strcpy_first69 = call ptr @strcpy(ptr %concat_buffer68, ptr @string_literal.6)
  %strcat_second70 = call ptr @strcat(ptr %concat_buffer68, ptr %int_str_buffer62)
  %printf_str_call71 = call i64 (ptr, ...) @printf(ptr @format_str, ptr %concat_buffer68)
  %teste372 = load i64, ptr %teste3, align 4
  %int_str_buffer73 = call ptr @malloc(i64 20)
  %sprintf_int74 = call i64 (ptr, ptr, ...) @sprintf(ptr %int_str_buffer73, ptr @format_str.2, i64 %teste372)
  %left_len75 = call i64 @strlen(ptr @string_literal.7)
  %right_len76 = call i64 @strlen(ptr %int_str_buffer73)
  %total_len77 = add i64 %left_len75, %right_len76
  %total_len_plus_one78 = add i64 %total_len77, 1
  %concat_buffer79 = call ptr @malloc(i64 %total_len_plus_one78)
  %strcpy_first80 = call ptr @strcpy(ptr %concat_buffer79, ptr @string_literal.7)
  %strcat_second81 = call ptr @strcat(ptr %concat_buffer79, ptr %int_str_buffer73)
  %printf_str_call82 = call i64 (ptr, ...) @printf(ptr @format_str, ptr %concat_buffer79)
  ret i64 0
}

declare i64 @printf(ptr, ...)

declare ptr @malloc(i64)

declare i64 @sprintf(ptr, ptr, ...)

declare i64 @strlen(ptr)

declare ptr @strcpy(ptr, ptr)

declare ptr @strcat(ptr, ptr)
