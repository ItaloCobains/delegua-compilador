; ModuleID = 'program'
source_filename = "program"

@string_literal = global [8 x i8] c"RESULT:\00"
@format_str = global [5 x i8] c"%lld\00"
@format_str.1 = global [4 x i8] c"%s\0A\00"

define i64 @main() {
entry:
  %limite = alloca i64, align 8
  store i64 1000000, ptr %limite, align 4
  %limite1 = load i64, ptr %limite, align 4
  %user_call = call i64 @contarPrimos(i64 %limite1)
  %resultado = alloca i64, align 8
  store i64 %user_call, ptr %resultado, align 4
  %resultado2 = load i64, ptr %resultado, align 4
  %int_str_buffer = call ptr @malloc(i64 20)
  %sprintf_int = call i64 (ptr, ptr, ...) @sprintf(ptr %int_str_buffer, ptr @format_str, i64 %resultado2)
  %left_len = call i64 @strlen(ptr @string_literal)
  %right_len = call i64 @strlen(ptr %int_str_buffer)
  %total_len = add i64 %left_len, %right_len
  %total_len_plus_one = add i64 %total_len, 1
  %concat_buffer = call ptr @malloc(i64 %total_len_plus_one)
  %strcpy_first = call ptr @strcpy(ptr %concat_buffer, ptr @string_literal)
  %strcat_second = call ptr @strcat(ptr %concat_buffer, ptr %int_str_buffer)
  %printf_str_call = call i64 (ptr, ...) @printf(ptr @format_str.1, ptr %concat_buffer)
  ret i64 0
}

define i64 @ehPrimo(i64 %0) {
entry:
  %n = alloca i64, align 8
  store i64 %0, ptr %n, align 4
  %n1 = load i64, ptr %n, align 4
  %lt = icmp slt i64 %n1, 2
  br i1 %lt, label %then, label %merge

then:                                             ; preds = %entry
  ret i64 0
  br label %merge

merge:                                            ; preds = %then, %entry
  %n2 = load i64, ptr %n, align 4
  %eq = icmp eq i64 %n2, 2
  br i1 %eq, label %then3, label %merge4

then3:                                            ; preds = %merge
  ret i64 1
  br label %merge4

merge4:                                           ; preds = %then3, %merge
  %n5 = load i64, ptr %n, align 4
  %mod = srem i64 %n5, 2
  %eq6 = icmp eq i64 %mod, 0
  br i1 %eq6, label %then7, label %merge8

then7:                                            ; preds = %merge4
  ret i64 0
  br label %merge8

merge8:                                           ; preds = %then7, %merge4
  %n9 = load i64, ptr %n, align 4
  %div = sdiv i64 %n9, 2
  %limite = alloca i64, align 8
  store i64 %div, ptr %limite, align 4
  br label %for_init

for_init:                                         ; preds = %merge8
  %i = alloca i64, align 8
  store i64 3, ptr %i, align 4
  br label %for_condition

for_condition:                                    ; preds = %for_increment, %for_init
  %i10 = load i64, ptr %i, align 4
  %limite11 = load i64, ptr %limite, align 4
  %le = icmp sle i64 %i10, %limite11
  br i1 %le, label %for_body, label %after_for

for_body:                                         ; preds = %for_condition
  %n12 = load i64, ptr %n, align 4
  %i13 = load i64, ptr %i, align 4
  %mod14 = srem i64 %n12, %i13
  %eq15 = icmp eq i64 %mod14, 0
  br i1 %eq15, label %then16, label %merge17

for_increment:                                    ; preds = %merge17
  %load_i = load i64, ptr %i, align 4
  %inc_i = add i64 %load_i, 1
  store i64 %inc_i, ptr %i, align 4
  br label %for_condition

after_for:                                        ; preds = %for_condition
  ret i64 1

then16:                                           ; preds = %for_body
  ret i64 0
  br label %merge17

merge17:                                          ; preds = %then16, %for_body
  br label %for_increment
}

define i64 @contarPrimos(i64 %0) {
entry:
  %limite = alloca i64, align 8
  store i64 %0, ptr %limite, align 4
  %contador = alloca i64, align 8
  store i64 0, ptr %contador, align 4
  br label %for_init

for_init:                                         ; preds = %entry
  %numero = alloca i64, align 8
  store i64 2, ptr %numero, align 4
  br label %for_condition

for_condition:                                    ; preds = %for_increment, %for_init
  %numero1 = load i64, ptr %numero, align 4
  %limite2 = load i64, ptr %limite, align 4
  %le = icmp sle i64 %numero1, %limite2
  br i1 %le, label %for_body, label %after_for

for_body:                                         ; preds = %for_condition
  %numero3 = load i64, ptr %numero, align 4
  %user_call = call i64 @ehPrimo(i64 %numero3)
  %if_condition = icmp ne i64 %user_call, 0
  br i1 %if_condition, label %then, label %merge

for_increment:                                    ; preds = %merge
  %load_numero = load i64, ptr %numero, align 4
  %inc_numero = add i64 %load_numero, 1
  store i64 %inc_numero, ptr %numero, align 4
  br label %for_condition

after_for:                                        ; preds = %for_condition
  %contador4 = load i64, ptr %contador, align 4
  ret i64 %contador4

then:                                             ; preds = %for_body
  %load_contador = load i64, ptr %contador, align 4
  %inc_contador = add i64 %load_contador, 1
  store i64 %inc_contador, ptr %contador, align 4
  br label %merge

merge:                                            ; preds = %then, %for_body
  br label %for_increment
}

declare ptr @malloc(i64)

declare i64 @sprintf(ptr, ptr, ...)

declare i64 @strlen(ptr)

declare ptr @strcpy(ptr, ptr)

declare ptr @strcat(ptr, ptr)

declare i64 @printf(ptr, ...)
