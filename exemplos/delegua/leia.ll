; ModuleID = 'program'
source_filename = "program"

@texto_literal = unnamed_addr constant [18 x i8] c"Digite seu nome: \00"
@formato_texto = unnamed_addr constant [3 x i8] c"%s\00"
@formato_texto.1 = unnamed_addr constant [6 x i8] c"%255s\00"

define i32 @main() {
entry:
  %printf_prompt = call i32 (ptr, ...) @printf(ptr @formato_texto, ptr @texto_literal)
  %input_buffer = call ptr @malloc(i64 256)
  %scanf_call = call i64 (ptr, ...) @scanf(ptr @formato_texto.1, ptr %input_buffer)
  ret i32 0
}

declare i64 @scanf(ptr, ...)

declare ptr @malloc(i64)

declare i32 @printf(ptr, ...)
