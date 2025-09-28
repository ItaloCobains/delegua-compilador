; ModuleID = 'program'
source_filename = "program"

@string_literal = global [17 x i8] c"x \C3\A9 maior que y\00"
@format_str = global [4 x i8] c"%s\0A\00"
@string_literal.1 = global [17 x i8] c"x \C3\A9 menor que y\00"
@string_literal.2 = global [22 x i8] c"x n\C3\A3o \C3\A9 menor que y\00"
@string_literal.3 = global [28 x i8] c"Resultado \C3\A9 maior que 10: \00"
@format_str.4 = global [6 x i8] c"%lld\0A\00"
@string_literal.5 = global [33 x i8] c"Resultado \C3\A9 menor ou igual a 10\00"

define i64 @main() {
entry:
  %x = alloca i64, align 8
  store i64 10, ptr %x, align 4
  %y = alloca i64, align 8
  store i64 5, ptr %y, align 4
  %x1 = load i64, ptr %x, align 4
  %y2 = load i64, ptr %y, align 4
  %gt = icmp sgt i64 %x1, %y2
  br i1 %gt, label %then, label %merge

then:                                             ; preds = %entry
  %printf_str_call = call i64 (ptr, ...) @printf(ptr @format_str, ptr @string_literal)
  br label %merge

merge:                                            ; preds = %then, %entry
  %x3 = load i64, ptr %x, align 4
  %y4 = load i64, ptr %y, align 4
  %lt = icmp slt i64 %x3, %y4
  br i1 %lt, label %then5, label %else

then5:                                            ; preds = %merge
  %printf_str_call7 = call i64 (ptr, ...) @printf(ptr @format_str, ptr @string_literal.1)
  br label %merge6

merge6:                                           ; preds = %else, %then5
  %x9 = load i64, ptr %x, align 4
  %y10 = load i64, ptr %y, align 4
  %add = add i64 %x9, %y10
  %resultado = alloca i64, align 8
  store i64 %add, ptr %resultado, align 4
  %resultado11 = load i64, ptr %resultado, align 4
  %gt12 = icmp sgt i64 %resultado11, 10
  br i1 %gt12, label %then13, label %else15

else:                                             ; preds = %merge
  %printf_str_call8 = call i64 (ptr, ...) @printf(ptr @format_str, ptr @string_literal.2)
  br label %merge6

then13:                                           ; preds = %merge6
  %printf_str_call16 = call i64 (ptr, ...) @printf(ptr @format_str, ptr @string_literal.3)
  %resultado17 = load i64, ptr %resultado, align 4
  %printf_int_call = call i64 (ptr, ...) @printf(ptr @format_str.4, i64 %resultado17)
  br label %merge14

merge14:                                          ; preds = %else15, %then13
  ret i64 0

else15:                                           ; preds = %merge6
  %printf_str_call18 = call i64 (ptr, ...) @printf(ptr @format_str, ptr @string_literal.5)
  br label %merge14
}

declare i64 @printf(ptr, ...)
