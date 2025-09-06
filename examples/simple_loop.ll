; ModuleID = 'program'
source_filename = "program"

@format_int = global [6 x i8] c"%lld\0A\00"
@string_literal = global [6 x i8] c"For: \00"
@int_format = global [5 x i8] c"%lld\00"
@format_str = global [4 x i8] c"%s\0A\00"
@string_literal.1 = global [9 x i8] c"Do-while\00"
@format_str.2 = global [4 x i8] c"%s\0A\00"
@string_literal.3 = global [13 x i8] c"Verdadeiro: \00"
@int_format.4 = global [5 x i8] c"%lld\00"
@format_str.5 = global [4 x i8] c"%s\0A\00"
@string_literal.6 = global [8 x i8] c"Falso: \00"
@int_format.7 = global [5 x i8] c"%lld\00"
@format_str.8 = global [4 x i8] c"%s\0A\00"

define i64 @main() {
entry:
  %contador = alloca i64, align 8
  store i64 0, ptr %contador, align 4
  br label %loop

loop:                                             ; preds = %loop_body, %entry
  %contador1 = load i64, ptr %contador, align 4
  %lt = icmp slt i64 %contador1, 5
  br i1 %lt, label %loop_body, label %after_loop

loop_body:                                        ; preds = %loop
  %contador2 = load i64, ptr %contador, align 4
  %printf_call = call i64 (ptr, ...) @printf(ptr @format_int, i64 %contador2)
  %contador3 = load i64, ptr %contador, align 4
  %add = add i64 %contador3, 1
  store i64 %add, ptr %contador, align 4
  br label %loop

after_loop:                                       ; preds = %loop
  br label %for_init

for_init:                                         ; preds = %after_loop
  %i = alloca i64, align 8
  store i64 0, ptr %i, align 4
  br label %for_condition

for_condition:                                    ; preds = %for_increment, %for_init
  %i4 = load i64, ptr %i, align 4
  %lt5 = icmp slt i64 %i4, 5
  br i1 %lt5, label %for_body, label %after_for

for_body:                                         ; preds = %for_condition
  %i6 = load i64, ptr %i, align 4
  %int_str_buffer = call ptr @malloc(i64 20)
  %sprintf_call = call i64 (ptr, ptr, ...) @sprintf(ptr %int_str_buffer, ptr @int_format, i64 %i6)
  %left_len = call i64 @strlen(ptr @string_literal)
  %right_len = call i64 @strlen(ptr %int_str_buffer)
  %total_len = add i64 %left_len, %right_len
  %total_len_plus_one = add i64 %total_len, 1
  %concat_buffer = call ptr @malloc(i64 %total_len_plus_one)
  %strcpy_first = call ptr @strcpy(ptr %concat_buffer, ptr @string_literal)
  %strcat_second = call ptr @strcat(ptr %concat_buffer, ptr %int_str_buffer)
  %printf_call7 = call i64 (ptr, ...) @printf(ptr @format_str, ptr %concat_buffer)
  %i8 = load i64, ptr %i, align 4
  %add9 = add i64 %i8, 1
  store i64 %add9, ptr %i, align 4
  br label %for_increment

for_increment:                                    ; preds = %for_body
  br label %for_condition

after_for:                                        ; preds = %for_condition
  br label %do_body

do_body:                                          ; preds = %do_condition, %after_for
  %printf_call10 = call i64 (ptr, ...) @printf(ptr @format_str.2, ptr @string_literal.1)
  br label %do_condition

do_condition:                                     ; preds = %do_body
  br i1 false, label %do_body, label %after_do

after_do:                                         ; preds = %do_condition
  %teste_verdadeiro = alloca i64, align 8
  store i64 1, ptr %teste_verdadeiro, align 4
  %teste_verdadeiro11 = load i64, ptr %teste_verdadeiro, align 4
  %int_str_buffer12 = call ptr @malloc(i64 20)
  %sprintf_call13 = call i64 (ptr, ptr, ...) @sprintf(ptr %int_str_buffer12, ptr @int_format.4, i64 %teste_verdadeiro11)
  %left_len14 = call i64 @strlen(ptr @string_literal.3)
  %right_len15 = call i64 @strlen(ptr %int_str_buffer12)
  %total_len16 = add i64 %left_len14, %right_len15
  %total_len_plus_one17 = add i64 %total_len16, 1
  %concat_buffer18 = call ptr @malloc(i64 %total_len_plus_one17)
  %strcpy_first19 = call ptr @strcpy(ptr %concat_buffer18, ptr @string_literal.3)
  %strcat_second20 = call ptr @strcat(ptr %concat_buffer18, ptr %int_str_buffer12)
  %printf_call21 = call i64 (ptr, ...) @printf(ptr @format_str.5, ptr %concat_buffer18)
  %teste_falso = alloca i64, align 8
  store i64 0, ptr %teste_falso, align 4
  %teste_falso22 = load i64, ptr %teste_falso, align 4
  %int_str_buffer23 = call ptr @malloc(i64 20)
  %sprintf_call24 = call i64 (ptr, ptr, ...) @sprintf(ptr %int_str_buffer23, ptr @int_format.7, i64 %teste_falso22)
  %left_len25 = call i64 @strlen(ptr @string_literal.6)
  %right_len26 = call i64 @strlen(ptr %int_str_buffer23)
  %total_len27 = add i64 %left_len25, %right_len26
  %total_len_plus_one28 = add i64 %total_len27, 1
  %concat_buffer29 = call ptr @malloc(i64 %total_len_plus_one28)
  %strcpy_first30 = call ptr @strcpy(ptr %concat_buffer29, ptr @string_literal.6)
  %strcat_second31 = call ptr @strcat(ptr %concat_buffer29, ptr %int_str_buffer23)
  %printf_call32 = call i64 (ptr, ...) @printf(ptr @format_str.8, ptr %concat_buffer29)
  ret i64 0
}

declare i64 @printf(ptr, ...)

declare ptr @malloc(i64)

declare i64 @sprintf(ptr, ptr, ...)

declare i64 @strlen(ptr)

declare ptr @strcpy(ptr, ptr)

declare ptr @strcat(ptr, ptr)
