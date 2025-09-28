; ModuleID = 'program'
source_filename = "program"

@string_literal = global [7 x i8] c"Ol\C3\A1, \00"
@format_str = global [4 x i8] c"%s\0A\00"
@format_str.1 = global [6 x i8] c"%lld\0A\00"
@string_literal.2 = global [2 x i8] c"!\00"
@string_literal.3 = global [20 x i8] c"Resultado da soma: \00"
@string_literal.4 = global [6 x i8] c"Maria\00"
@string_literal.5 = global [15 x i8] c"Maior numero: \00"

define i64 @main() {
entry:
  %user_call = call i64 @somar(i64 10, i64 5)
  %resultado = alloca i64, align 8
  store i64 %user_call, ptr %resultado, align 4
  %printf_str_call = call i64 (ptr, ...) @printf(ptr @format_str, ptr @string_literal.3)
  %resultado1 = load i64, ptr %resultado, align 4
  %printf_int_call = call i64 (ptr, ...) @printf(ptr @format_str.1, i64 %resultado1)
  %user_call2 = call i64 @saudacao(ptr @string_literal.4)
  %user_call3 = call i64 @maiorQue(i64 8, i64 12)
  %maior = alloca i64, align 8
  store i64 %user_call3, ptr %maior, align 4
  %printf_str_call4 = call i64 (ptr, ...) @printf(ptr @format_str, ptr @string_literal.5)
  %maior5 = load i64, ptr %maior, align 4
  %printf_int_call6 = call i64 (ptr, ...) @printf(ptr @format_str.1, i64 %maior5)
  ret i64 0
}

define i64 @somar(i64 %0, i64 %1) {
entry:
  %a = alloca i64, align 8
  store i64 %0, ptr %a, align 4
  %b = alloca i64, align 8
  store i64 %1, ptr %b, align 4
  %a1 = load i64, ptr %a, align 4
  %b2 = load i64, ptr %b, align 4
  %add = add i64 %a1, %b2
  ret i64 %add
}

define i64 @saudacao(i64 %0) {
entry:
  %nome = alloca i64, align 8
  store i64 %0, ptr %nome, align 4
  %printf_str_call = call i64 (ptr, ...) @printf(ptr @format_str, ptr @string_literal)
  %nome1 = load i64, ptr %nome, align 4
  %printf_int_call = call i64 (ptr, ...) @printf(ptr @format_str.1, i64 %nome1)
  %printf_str_call2 = call i64 (ptr, ...) @printf(ptr @format_str, ptr @string_literal.2)
  ret i64 0
}

declare i64 @printf(ptr, ...)

define i64 @maiorQue(i64 %0, i64 %1) {
entry:
  %x = alloca i64, align 8
  store i64 %0, ptr %x, align 4
  %y = alloca i64, align 8
  store i64 %1, ptr %y, align 4
  %x1 = load i64, ptr %x, align 4
  %y2 = load i64, ptr %y, align 4
  %gt = icmp sgt i64 %x1, %y2
  br i1 %gt, label %then, label %else

then:                                             ; preds = %entry
  %x3 = load i64, ptr %x, align 4
  ret i64 %x3
  br label %merge

merge:                                            ; preds = %else, %then
  ret i64 0

else:                                             ; preds = %entry
  %y4 = load i64, ptr %y, align 4
  ret i64 %y4
  br label %merge
}
