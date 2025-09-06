; ModuleID = 'program'
source_filename = "program"

@string_literal = global [22 x i8] c"Condi\C3\A7\C3\A3o verdadeira\00"
@format_str = global [4 x i8] c"%s\0A\00"

define i64 @main() {
entry:
  br i1 true, label %then, label %merge

then:                                             ; preds = %entry
  %printf_call = call i64 (ptr, ...) @printf(ptr @format_str, ptr @string_literal)
  br label %merge

merge:                                            ; preds = %then, %entry
  ret i64 0
}

declare i64 @printf(ptr, ...)
