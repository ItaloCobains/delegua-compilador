; ModuleID = 'program'
source_filename = "program"

@string_literal = global [11 x i8] c"abs(-5) = \00"
@int_format = global [5 x i8] c"%lld\00"
@format_str = global [4 x i8] c"%s\0A\00"
@string_literal.1 = global [13 x i8] c"pow(2, 3) = \00"
@int_format.2 = global [5 x i8] c"%lld\00"
@format_str.3 = global [4 x i8] c"%s\0A\00"
@string_literal.4 = global [12 x i8] c"sqrt(16) = \00"
@int_format.5 = global [5 x i8] c"%lld\00"
@format_str.6 = global [4 x i8] c"%s\0A\00"

define i64 @main() {
entry:
  %x = alloca i64, align 8
  store i64 -5, ptr %x, align 4
  %x1 = load i64, ptr %x, align 4
  %module_call = call i64 @math_abs(i64 %x1)
  %abs_x = alloca i64, align 8
  store i64 %module_call, ptr %abs_x, align 4
  %module_call2 = call i64 @math_pow(i64 2, i64 3)
  %pow_result = alloca i64, align 8
  store i64 %module_call2, ptr %pow_result, align 4
  %module_call3 = call i64 @math_sqrt(i64 16)
  %sqrt_result = alloca i64, align 8
  store i64 %module_call3, ptr %sqrt_result, align 4
  %abs_x4 = load i64, ptr %abs_x, align 4
  %int_str_buffer = call ptr @malloc(i64 20)
  %sprintf_call = call i64 (ptr, ptr, ...) @sprintf(ptr %int_str_buffer, ptr @int_format, i64 %abs_x4)
  %left_len = call i64 @strlen(ptr @string_literal)
  %right_len = call i64 @strlen(ptr %int_str_buffer)
  %total_len = add i64 %left_len, %right_len
  %total_len_plus_one = add i64 %total_len, 1
  %concat_buffer = call ptr @malloc(i64 %total_len_plus_one)
  %strcpy_first = call ptr @strcpy(ptr %concat_buffer, ptr @string_literal)
  %strcat_second = call ptr @strcat(ptr %concat_buffer, ptr %int_str_buffer)
  %printf_call = call i64 (ptr, ...) @printf(ptr @format_str, ptr %concat_buffer)
  %pow_result5 = load i64, ptr %pow_result, align 4
  %int_str_buffer6 = call ptr @malloc(i64 20)
  %sprintf_call7 = call i64 (ptr, ptr, ...) @sprintf(ptr %int_str_buffer6, ptr @int_format.2, i64 %pow_result5)
  %left_len8 = call i64 @strlen(ptr @string_literal.1)
  %right_len9 = call i64 @strlen(ptr %int_str_buffer6)
  %total_len10 = add i64 %left_len8, %right_len9
  %total_len_plus_one11 = add i64 %total_len10, 1
  %concat_buffer12 = call ptr @malloc(i64 %total_len_plus_one11)
  %strcpy_first13 = call ptr @strcpy(ptr %concat_buffer12, ptr @string_literal.1)
  %strcat_second14 = call ptr @strcat(ptr %concat_buffer12, ptr %int_str_buffer6)
  %printf_call15 = call i64 (ptr, ...) @printf(ptr @format_str.3, ptr %concat_buffer12)
  %sqrt_result16 = load i64, ptr %sqrt_result, align 4
  %int_str_buffer17 = call ptr @malloc(i64 20)
  %sprintf_call18 = call i64 (ptr, ptr, ...) @sprintf(ptr %int_str_buffer17, ptr @int_format.5, i64 %sqrt_result16)
  %left_len19 = call i64 @strlen(ptr @string_literal.4)
  %right_len20 = call i64 @strlen(ptr %int_str_buffer17)
  %total_len21 = add i64 %left_len19, %right_len20
  %total_len_plus_one22 = add i64 %total_len21, 1
  %concat_buffer23 = call ptr @malloc(i64 %total_len_plus_one22)
  %strcpy_first24 = call ptr @strcpy(ptr %concat_buffer23, ptr @string_literal.4)
  %strcat_second25 = call ptr @strcat(ptr %concat_buffer23, ptr %int_str_buffer17)
  %printf_call26 = call i64 (ptr, ...) @printf(ptr @format_str.6, ptr %concat_buffer23)
  ret i64 0
}

define i64 @math_abs(i64 %0) {
entry:
  %is_negative = icmp slt i64 %0, 0
  %neg_x = sub i64 0, %0
  %abs_result = select i1 %is_negative, i64 %neg_x, i64 %0
  ret i64 %abs_result
}

define i64 @math_pow(i64 %0, i64 %1) {
entry:
  %result = alloca i64, align 8
  %counter = alloca i64, align 8
  store i64 1, ptr %result, align 4
  store i64 %1, ptr %counter, align 4
  br label %loop

loop:                                             ; preds = %loop_body, %entry
  %counter_val = load i64, ptr %counter, align 4
  %cond = icmp sgt i64 %counter_val, 0
  br i1 %cond, label %loop_body, label %after

loop_body:                                        ; preds = %loop
  %result_val = load i64, ptr %result, align 4
  %new_result = mul i64 %result_val, %0
  store i64 %new_result, ptr %result, align 4
  %new_counter = sub i64 %counter_val, 1
  store i64 %new_counter, ptr %counter, align 4
  br label %loop

after:                                            ; preds = %loop
  %final_result = load i64, ptr %result, align 4
  ret i64 %final_result
}

define i64 @math_sqrt(i64 %0) {
entry:
  ret i64 4
}

declare ptr @malloc(i64)

declare i64 @sprintf(ptr, ptr, ...)

declare i64 @strlen(ptr)

declare ptr @strcpy(ptr, ptr)

declare ptr @strcat(ptr, ptr)

declare i64 @printf(ptr, ...)
