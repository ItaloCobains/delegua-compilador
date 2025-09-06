; ModuleID = 'program'
source_filename = "program"

@string_literal = global [18 x i8] c"Excelente! Nota A\00"
@format_str = global [4 x i8] c"%s\0A\00"
@string_literal.1 = global [18 x i8] c"Muito bom! Nota B\00"
@format_str.2 = global [4 x i8] c"%s\0A\00"
@string_literal.3 = global [12 x i8] c"Bom! Nota C\00"
@format_str.4 = global [4 x i8] c"%s\0A\00"
@string_literal.5 = global [30 x i8] c"Precisa melhorar. Nota D ou F\00"
@format_str.6 = global [4 x i8] c"%s\0A\00"

define i64 @main() {
entry:
  %nota = alloca i64, align 8
  store i64 95, ptr %nota, align 4
  %nota1 = load i64, ptr %nota, align 4
  %ge = icmp sge i64 %nota1, 90
  br i1 %ge, label %then, label %else_if_start

then:                                             ; preds = %entry
  %printf_call = call i64 (ptr, ...) @printf(ptr @format_str, ptr @string_literal)
  br label %merge

else_if_start:                                    ; preds = %entry
  %nota2 = load i64, ptr %nota, align 4
  %ge3 = icmp sge i64 %nota2, 80
  br i1 %ge3, label %else_if_then_0, label %else_if_1

merge:                                            ; preds = %final_else, %else_if_then_1, %else_if_then_0, %then
  ret i64 0

else_if_then_0:                                   ; preds = %else_if_start
  %printf_call4 = call i64 (ptr, ...) @printf(ptr @format_str.2, ptr @string_literal.1)
  br label %merge

else_if_1:                                        ; preds = %else_if_start
  %nota5 = load i64, ptr %nota, align 4
  %ge6 = icmp sge i64 %nota5, 70
  br i1 %ge6, label %else_if_then_1, label %final_else

else_if_then_1:                                   ; preds = %else_if_1
  %printf_call7 = call i64 (ptr, ...) @printf(ptr @format_str.4, ptr @string_literal.3)
  br label %merge

final_else:                                       ; preds = %else_if_1
  %printf_call8 = call i64 (ptr, ...) @printf(ptr @format_str.6, ptr @string_literal.5)
  br label %merge
}

declare i64 @printf(ptr, ...)
