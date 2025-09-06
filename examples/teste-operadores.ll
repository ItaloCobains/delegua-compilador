; ModuleID = 'program'
source_filename = "program"

@string_literal = global [37 x i8] c"Testando operadores de compara\C3\A7\C3\A3o:\00"
@format_str = global [4 x i8] c"%s\0A\00"
@string_literal.1 = global [20 x i8] c"a < b \C3\A9 verdadeiro\00"
@format_str.2 = global [4 x i8] c"%s\0A\00"
@string_literal.3 = global [21 x i8] c"a <= 5 \C3\A9 verdadeiro\00"
@format_str.4 = global [4 x i8] c"%s\0A\00"
@string_literal.5 = global [20 x i8] c"b > a \C3\A9 verdadeiro\00"
@format_str.6 = global [4 x i8] c"%s\0A\00"
@string_literal.7 = global [22 x i8] c"b >= 10 \C3\A9 verdadeiro\00"
@format_str.8 = global [4 x i8] c"%s\0A\00"
@string_literal.9 = global [21 x i8] c"a == 5 \C3\A9 verdadeiro\00"
@format_str.10 = global [4 x i8] c"%s\0A\00"
@string_literal.11 = global [21 x i8] c"b != 5 \C3\A9 verdadeiro\00"
@format_str.12 = global [4 x i8] c"%s\0A\00"

define i64 @main() {
entry:
  %a = alloca i64, align 8
  store i64 5, ptr %a, align 4
  %b = alloca i64, align 8
  store i64 10, ptr %b, align 4
  %printf_call = call i64 (ptr, ...) @printf(ptr @format_str, ptr @string_literal)
  %a1 = load i64, ptr %a, align 4
  %b2 = load i64, ptr %b, align 4
  %lt = icmp slt i64 %a1, %b2
  br i1 %lt, label %then, label %merge

then:                                             ; preds = %entry
  %printf_call3 = call i64 (ptr, ...) @printf(ptr @format_str.2, ptr @string_literal.1)
  br label %merge

merge:                                            ; preds = %then, %entry
  %a4 = load i64, ptr %a, align 4
  %le = icmp sle i64 %a4, 5
  br i1 %le, label %then5, label %merge6

then5:                                            ; preds = %merge
  %printf_call7 = call i64 (ptr, ...) @printf(ptr @format_str.4, ptr @string_literal.3)
  br label %merge6

merge6:                                           ; preds = %then5, %merge
  %b8 = load i64, ptr %b, align 4
  %a9 = load i64, ptr %a, align 4
  %gt = icmp sgt i64 %b8, %a9
  br i1 %gt, label %then10, label %merge11

then10:                                           ; preds = %merge6
  %printf_call12 = call i64 (ptr, ...) @printf(ptr @format_str.6, ptr @string_literal.5)
  br label %merge11

merge11:                                          ; preds = %then10, %merge6
  %b13 = load i64, ptr %b, align 4
  %ge = icmp sge i64 %b13, 10
  br i1 %ge, label %then14, label %merge15

then14:                                           ; preds = %merge11
  %printf_call16 = call i64 (ptr, ...) @printf(ptr @format_str.8, ptr @string_literal.7)
  br label %merge15

merge15:                                          ; preds = %then14, %merge11
  %a17 = load i64, ptr %a, align 4
  %eq = icmp eq i64 %a17, 5
  br i1 %eq, label %then18, label %merge19

then18:                                           ; preds = %merge15
  %printf_call20 = call i64 (ptr, ...) @printf(ptr @format_str.10, ptr @string_literal.9)
  br label %merge19

merge19:                                          ; preds = %then18, %merge15
  %b21 = load i64, ptr %b, align 4
  %ne = icmp ne i64 %b21, 5
  br i1 %ne, label %then22, label %merge23

then22:                                           ; preds = %merge19
  %printf_call24 = call i64 (ptr, ...) @printf(ptr @format_str.12, ptr @string_literal.11)
  br label %merge23

merge23:                                          ; preds = %then22, %merge19
  ret i64 0
}

declare i64 @printf(ptr, ...)
