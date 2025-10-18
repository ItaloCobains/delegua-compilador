; ModuleID = 'program'
source_filename = "program"

@texto_literal = unnamed_addr constant [13 x i8] c"Ol\C3\A1, mundo!\00"
@formato_texto = unnamed_addr constant [4 x i8] c"%s\0A\00"

define i32 @main() {
entry:
  %printf_call = call i32 (ptr, ...) @printf(ptr @formato_texto, ptr @texto_literal)
  ret i32 0
}

declare i32 @printf(ptr, ...)
