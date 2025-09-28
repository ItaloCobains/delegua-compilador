; ModuleID = 'program'
source_filename = "program"

@string_literal = global [12 x i8] c"Hello World\00"
@string_literal.1 = global [11 x i8] c"Original: \00"
@format_str = global [4 x i8] c"%s\0A\00"
@string_literal.2 = global [14 x i8] c"Comprimento: \00"
@format_str.3 = global [6 x i8] c"%lld\0A\00"
@string_literal.4 = global [12 x i8] c"Maiuscula: \00"
@string_literal.5 = global [12 x i8] c"Minuscula: \00"
@string_literal.6 = global [8 x i8] c"DeLegUa\00"
@string_literal.7 = global [17 x i8] c"Misto original: \00"
@string_literal.8 = global [18 x i8] c"Misto maiuscula: \00"
@string_literal.9 = global [18 x i8] c"Misto minuscula: \00"

define i64 @main() {
entry:
  %texto = alloca ptr, align 8
  store ptr @string_literal, ptr %texto, align 8
  %printf_str_call = call i64 (ptr, ...) @printf(ptr @format_str, ptr @string_literal.1)
  %texto1 = load ptr, ptr %texto, align 8
  %printf_str_call2 = call i64 (ptr, ...) @printf(ptr @format_str, ptr %texto1)
  %texto3 = load ptr, ptr %texto, align 8
  %strlen_call = call i64 @strlen(ptr %texto3)
  %tamanho = alloca i64, align 8
  store i64 %strlen_call, ptr %tamanho, align 4
  %printf_str_call4 = call i64 (ptr, ...) @printf(ptr @format_str, ptr @string_literal.2)
  %tamanho5 = load i64, ptr %tamanho, align 4
  %printf_int_call = call i64 (ptr, ...) @printf(ptr @format_str.3, i64 %tamanho5)
  %texto6 = load ptr, ptr %texto, align 8
  %strlen_call7 = call i64 @strlen(ptr %texto6)
  %buffer_size = add i64 %strlen_call7, 1
  %upper_buffer = call ptr @malloc(i64 %buffer_size)
  %strcpy_call = call ptr @strcpy(ptr %upper_buffer, ptr %texto6)
  %upper_counter = alloca i64, align 8
  store i64 0, ptr %upper_counter, align 4
  br label %upper_loop

upper_loop:                                       ; preds = %upper_body, %entry
  %load_counter = load i64, ptr %upper_counter, align 4
  %is_end = icmp eq i64 %load_counter, %strlen_call7
  br i1 %is_end, label %upper_done, label %upper_body

upper_body:                                       ; preds = %upper_loop
  %char_ptr = getelementptr i8, ptr %upper_buffer, i64 %load_counter
  %char_val = load i8, ptr %char_ptr, align 1
  %char_ext = zext i8 %char_val to i64
  %is_ge_a = icmp sge i64 %char_ext, 97
  %is_le_z = icmp sle i64 %char_ext, 122
  %is_lowercase = and i1 %is_ge_a, %is_le_z
  %upper_char = sub i64 %char_ext, 32
  %final_char = select i1 %is_lowercase, i64 %upper_char, i64 %char_ext
  %final_char_i8 = trunc i64 %final_char to i8
  store i8 %final_char_i8, ptr %char_ptr, align 1
  %increment = add i64 %load_counter, 1
  store i64 %increment, ptr %upper_counter, align 4
  br label %upper_loop

upper_done:                                       ; preds = %upper_loop
  %maiusculo = alloca ptr, align 8
  store ptr %upper_buffer, ptr %maiusculo, align 8
  %printf_str_call8 = call i64 (ptr, ...) @printf(ptr @format_str, ptr @string_literal.4)
  %maiusculo9 = load ptr, ptr %maiusculo, align 8
  %printf_str_call10 = call i64 (ptr, ...) @printf(ptr @format_str, ptr %maiusculo9)
  %texto11 = load ptr, ptr %texto, align 8
  %strlen_call12 = call i64 @strlen(ptr %texto11)
  %buffer_size13 = add i64 %strlen_call12, 1
  %lower_buffer = call ptr @malloc(i64 %buffer_size13)
  %strcpy_call14 = call ptr @strcpy(ptr %lower_buffer, ptr %texto11)
  %lower_counter = alloca i64, align 8
  store i64 0, ptr %lower_counter, align 4
  br label %lower_loop

lower_loop:                                       ; preds = %lower_body, %upper_done
  %load_counter15 = load i64, ptr %lower_counter, align 4
  %is_end16 = icmp eq i64 %load_counter15, %strlen_call12
  br i1 %is_end16, label %lower_done, label %lower_body

lower_body:                                       ; preds = %lower_loop
  %char_ptr17 = getelementptr i8, ptr %lower_buffer, i64 %load_counter15
  %char_val18 = load i8, ptr %char_ptr17, align 1
  %char_ext19 = zext i8 %char_val18 to i64
  %is_ge_A = icmp sge i64 %char_ext19, 65
  %is_le_Z = icmp sle i64 %char_ext19, 90
  %is_uppercase = and i1 %is_ge_A, %is_le_Z
  %lower_char = add i64 %char_ext19, 32
  %final_char20 = select i1 %is_uppercase, i64 %lower_char, i64 %char_ext19
  %final_char_i821 = trunc i64 %final_char20 to i8
  store i8 %final_char_i821, ptr %char_ptr17, align 1
  %increment22 = add i64 %load_counter15, 1
  store i64 %increment22, ptr %lower_counter, align 4
  br label %lower_loop

lower_done:                                       ; preds = %lower_loop
  %minusculo = alloca ptr, align 8
  store ptr %lower_buffer, ptr %minusculo, align 8
  %printf_str_call23 = call i64 (ptr, ...) @printf(ptr @format_str, ptr @string_literal.5)
  %minusculo24 = load ptr, ptr %minusculo, align 8
  %printf_str_call25 = call i64 (ptr, ...) @printf(ptr @format_str, ptr %minusculo24)
  %misto = alloca ptr, align 8
  store ptr @string_literal.6, ptr %misto, align 8
  %printf_str_call26 = call i64 (ptr, ...) @printf(ptr @format_str, ptr @string_literal.7)
  %misto27 = load ptr, ptr %misto, align 8
  %printf_str_call28 = call i64 (ptr, ...) @printf(ptr @format_str, ptr %misto27)
  %printf_str_call29 = call i64 (ptr, ...) @printf(ptr @format_str, ptr @string_literal.8)
  %misto30 = load ptr, ptr %misto, align 8
  %strlen_call31 = call i64 @strlen(ptr %misto30)
  %buffer_size32 = add i64 %strlen_call31, 1
  %upper_buffer33 = call ptr @malloc(i64 %buffer_size32)
  %strcpy_call34 = call ptr @strcpy(ptr %upper_buffer33, ptr %misto30)
  %upper_counter38 = alloca i64, align 8
  store i64 0, ptr %upper_counter38, align 4
  br label %upper_loop35

upper_loop35:                                     ; preds = %upper_body36, %lower_done
  %load_counter39 = load i64, ptr %upper_counter38, align 4
  %is_end40 = icmp eq i64 %load_counter39, %strlen_call31
  br i1 %is_end40, label %upper_done37, label %upper_body36

upper_body36:                                     ; preds = %upper_loop35
  %char_ptr41 = getelementptr i8, ptr %upper_buffer33, i64 %load_counter39
  %char_val42 = load i8, ptr %char_ptr41, align 1
  %char_ext43 = zext i8 %char_val42 to i64
  %is_ge_a44 = icmp sge i64 %char_ext43, 97
  %is_le_z45 = icmp sle i64 %char_ext43, 122
  %is_lowercase46 = and i1 %is_ge_a44, %is_le_z45
  %upper_char47 = sub i64 %char_ext43, 32
  %final_char48 = select i1 %is_lowercase46, i64 %upper_char47, i64 %char_ext43
  %final_char_i849 = trunc i64 %final_char48 to i8
  store i8 %final_char_i849, ptr %char_ptr41, align 1
  %increment50 = add i64 %load_counter39, 1
  store i64 %increment50, ptr %upper_counter38, align 4
  br label %upper_loop35

upper_done37:                                     ; preds = %upper_loop35
  %printf_str_call51 = call i64 (ptr, ...) @printf(ptr @format_str, ptr %upper_buffer33)
  %printf_str_call52 = call i64 (ptr, ...) @printf(ptr @format_str, ptr @string_literal.9)
  %misto53 = load ptr, ptr %misto, align 8
  %strlen_call54 = call i64 @strlen(ptr %misto53)
  %buffer_size55 = add i64 %strlen_call54, 1
  %lower_buffer56 = call ptr @malloc(i64 %buffer_size55)
  %strcpy_call57 = call ptr @strcpy(ptr %lower_buffer56, ptr %misto53)
  %lower_counter61 = alloca i64, align 8
  store i64 0, ptr %lower_counter61, align 4
  br label %lower_loop58

lower_loop58:                                     ; preds = %lower_body59, %upper_done37
  %load_counter62 = load i64, ptr %lower_counter61, align 4
  %is_end63 = icmp eq i64 %load_counter62, %strlen_call54
  br i1 %is_end63, label %lower_done60, label %lower_body59

lower_body59:                                     ; preds = %lower_loop58
  %char_ptr64 = getelementptr i8, ptr %lower_buffer56, i64 %load_counter62
  %char_val65 = load i8, ptr %char_ptr64, align 1
  %char_ext66 = zext i8 %char_val65 to i64
  %is_ge_A67 = icmp sge i64 %char_ext66, 65
  %is_le_Z68 = icmp sle i64 %char_ext66, 90
  %is_uppercase69 = and i1 %is_ge_A67, %is_le_Z68
  %lower_char70 = add i64 %char_ext66, 32
  %final_char71 = select i1 %is_uppercase69, i64 %lower_char70, i64 %char_ext66
  %final_char_i872 = trunc i64 %final_char71 to i8
  store i8 %final_char_i872, ptr %char_ptr64, align 1
  %increment73 = add i64 %load_counter62, 1
  store i64 %increment73, ptr %lower_counter61, align 4
  br label %lower_loop58

lower_done60:                                     ; preds = %lower_loop58
  %printf_str_call74 = call i64 (ptr, ...) @printf(ptr @format_str, ptr %lower_buffer56)
  ret i64 0
}

declare i64 @printf(ptr, ...)

declare i64 @strlen(ptr)

declare ptr @malloc(i64)

declare ptr @strcpy(ptr, ptr)
