; ModuleID = 'program'
source_filename = "program"

@string_literal = global [18 x i8] c"x \C3\A9 maior que 10\00"
@format_str = global [4 x i8] c"%s\0A\00"
@string_literal.1 = global [25 x i8] c"x \C3\A9 menor ou igual a 10\00"
@format_str.2 = global [4 x i8] c"%s\0A\00"

define i64 @main() {
entry:
  %x = alloca i64, align 8
  store i64 15, ptr %x, align 4
  %x1 = load i64, ptr %x, align 4
  %gt = icmp sgt i64 %x1, 10
  br i1 %gt, label %then, label %else

then:                                             ; preds = %entry
  %printf_call = call i64 (ptr, ...) @printf(ptr @format_str, ptr @string_literal)
  br label %merge

merge:                                            ; preds = %else, %then
  ret i64 0

else:                                             ; preds = %entry
  %printf_call2 = call i64 (ptr, ...) @printf(ptr @format_str.2, ptr @string_literal.1)
  br label %merge
}

declare i64 @printf(ptr, ...)
