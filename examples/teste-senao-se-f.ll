; ModuleID = 'program'
source_filename = "program"

@string_literal = global [18 x i8] c"Excelente! Nota A\00"
@format_str = global [4 x i8] c"%s\0A\00"
@string_literal.1 = global [2 x i8] c"a\00"
@format_str.2 = global [4 x i8] c"%s\0A\00"
@string_literal.3 = global [2 x i8] c"s\00"
@format_str.4 = global [4 x i8] c"%s\0A\00"

define i64 @main() {
entry:
  %nota = alloca i64, align 8
  store i64 180, ptr %nota, align 4
  %nota1 = load i64, ptr %nota, align 4
  %ge = icmp sge i64 %nota1, 90
  br i1 %ge, label %then, label %merge

then:                                             ; preds = %entry
  %printf_call = call i64 (ptr, ...) @printf(ptr @format_str, ptr @string_literal)
  %nota2 = load i64, ptr %nota, align 4
  %ge3 = icmp sge i64 %nota2, 80
  br i1 %ge3, label %then4, label %merge5

merge:                                            ; preds = %merge5, %entry
  ret i64 0

then4:                                            ; preds = %then
  %printf_call6 = call i64 (ptr, ...) @printf(ptr @format_str.2, ptr @string_literal.1)
  %nota7 = load i64, ptr %nota, align 4
  %ge8 = icmp sge i64 %nota7, 70
  br i1 %ge8, label %then9, label %merge10

merge5:                                           ; preds = %merge10, %then
  br label %merge

then9:                                            ; preds = %then4
  %printf_call11 = call i64 (ptr, ...) @printf(ptr @format_str.4, ptr @string_literal.3)
  br label %merge10

merge10:                                          ; preds = %then9, %then4
  br label %merge5
}

declare i64 @printf(ptr, ...)
