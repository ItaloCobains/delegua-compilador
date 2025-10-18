; ModuleID = 'program'
source_filename = "program"

@texto_literal = unnamed_addr constant [18 x i8] c"Digite seu nome: \00"
@formato_texto = unnamed_addr constant [6 x i8] c"%255s\00"
@texto_literal.1 = unnamed_addr constant [7 x i8] c"Ol\C3\A1, \00"
@texto_literal.2 = unnamed_addr constant [2 x i8] c"!\00"
@formato_texto.3 = unnamed_addr constant [4 x i8] c"%s\0A\00"
@texto_literal.4 = unnamed_addr constant [19 x i8] c"Digite sua idade: \00"
@texto_literal.5 = unnamed_addr constant [11 x i8] c"Voc\C3\AA tem \00"
@texto_literal.6 = unnamed_addr constant [7 x i8] c" anos.\00"
@texto_literal.7 = unnamed_addr constant [30 x i8] c"Digite sua altura em metros: \00"
@texto_literal.8 = unnamed_addr constant [15 x i8] c"Sua altura \C3\A9 \00"
@texto_literal.9 = unnamed_addr constant [9 x i8] c" metros.\00"
@texto_literal.10 = unnamed_addr constant [24 x i8] c"Digite seu peso em kg: \00"
@texto_literal.11 = unnamed_addr constant [13 x i8] c"Seu peso \C3\A9 \00"
@texto_literal.12 = unnamed_addr constant [5 x i8] c" kg.\00"

define i32 @main() {
entry:
  %puts_prompt = call i32 @puts(ptr @texto_literal)
  %input_buffer = call ptr @malloc(i64 256)
  %e_nulo = icmp eq ptr %input_buffer, null
  br i1 %e_nulo, label %alocacao_falhou, label %alocacao_sucesso

alocacao_falhou:                                  ; preds = %entry
  %exit_call = call i32 @exit(i32 1)
  unreachable

alocacao_sucesso:                                 ; preds = %entry
  %scanf_call = call i64 (ptr, ...) @scanf(ptr @formato_texto, ptr %input_buffer)
  %nome = alloca ptr, align 8
  store ptr %input_buffer, ptr %nome, align 8
  %nome1 = load ptr, ptr %nome, align 8
  %left_len = call i64 @strlen(ptr @texto_literal.1)
  %right_len = call i64 @strlen(ptr %nome1)
  %total_len = add i64 %left_len, %right_len
  %total_len_plus_one = add i64 %total_len, 1
  %concat_buffer = call ptr @malloc(i64 %total_len_plus_one)
  %strcpy_first = call ptr @strcpy(ptr %concat_buffer, ptr @texto_literal.1)
  %strcat_second = call ptr @strcat(ptr %concat_buffer, ptr %nome1)
  %left_len2 = call i64 @strlen(ptr %concat_buffer)
  %right_len3 = call i64 @strlen(ptr @texto_literal.2)
  %total_len4 = add i64 %left_len2, %right_len3
  %total_len_plus_one5 = add i64 %total_len4, 1
  %concat_buffer6 = call ptr @malloc(i64 %total_len_plus_one5)
  %strcpy_first7 = call ptr @strcpy(ptr %concat_buffer6, ptr %concat_buffer)
  %strcat_second8 = call ptr @strcat(ptr %concat_buffer6, ptr @texto_literal.2)
  %printf_call = call i32 (ptr, ...) @printf(ptr @formato_texto.3, ptr %concat_buffer6)
  %puts_prompt9 = call i32 @puts(ptr @texto_literal.4)
  %input_buffer10 = call ptr @malloc(i64 256)
  %e_nulo13 = icmp eq ptr %input_buffer10, null
  br i1 %e_nulo13, label %alocacao_falhou11, label %alocacao_sucesso12

alocacao_falhou11:                                ; preds = %alocacao_sucesso
  %exit_call14 = call i32 @exit(i32 1)
  unreachable

alocacao_sucesso12:                               ; preds = %alocacao_sucesso
  %scanf_call15 = call i64 (ptr, ...) @scanf(ptr @formato_texto, ptr %input_buffer10)
  %idade = alloca ptr, align 8
  store ptr %input_buffer10, ptr %idade, align 8
  %idade16 = load ptr, ptr %idade, align 8
  %left_len17 = call i64 @strlen(ptr @texto_literal.5)
  %right_len18 = call i64 @strlen(ptr %idade16)
  %total_len19 = add i64 %left_len17, %right_len18
  %total_len_plus_one20 = add i64 %total_len19, 1
  %concat_buffer21 = call ptr @malloc(i64 %total_len_plus_one20)
  %strcpy_first22 = call ptr @strcpy(ptr %concat_buffer21, ptr @texto_literal.5)
  %strcat_second23 = call ptr @strcat(ptr %concat_buffer21, ptr %idade16)
  %left_len24 = call i64 @strlen(ptr %concat_buffer21)
  %right_len25 = call i64 @strlen(ptr @texto_literal.6)
  %total_len26 = add i64 %left_len24, %right_len25
  %total_len_plus_one27 = add i64 %total_len26, 1
  %concat_buffer28 = call ptr @malloc(i64 %total_len_plus_one27)
  %strcpy_first29 = call ptr @strcpy(ptr %concat_buffer28, ptr %concat_buffer21)
  %strcat_second30 = call ptr @strcat(ptr %concat_buffer28, ptr @texto_literal.6)
  %printf_call31 = call i32 (ptr, ...) @printf(ptr @formato_texto.3, ptr %concat_buffer28)
  %puts_prompt32 = call i32 @puts(ptr @texto_literal.7)
  %input_buffer33 = call ptr @malloc(i64 256)
  %e_nulo36 = icmp eq ptr %input_buffer33, null
  br i1 %e_nulo36, label %alocacao_falhou34, label %alocacao_sucesso35

alocacao_falhou34:                                ; preds = %alocacao_sucesso12
  %exit_call37 = call i32 @exit(i32 1)
  unreachable

alocacao_sucesso35:                               ; preds = %alocacao_sucesso12
  %scanf_call38 = call i64 (ptr, ...) @scanf(ptr @formato_texto, ptr %input_buffer33)
  %altura = alloca ptr, align 8
  store ptr %input_buffer33, ptr %altura, align 8
  %altura39 = load ptr, ptr %altura, align 8
  %left_len40 = call i64 @strlen(ptr @texto_literal.8)
  %right_len41 = call i64 @strlen(ptr %altura39)
  %total_len42 = add i64 %left_len40, %right_len41
  %total_len_plus_one43 = add i64 %total_len42, 1
  %concat_buffer44 = call ptr @malloc(i64 %total_len_plus_one43)
  %strcpy_first45 = call ptr @strcpy(ptr %concat_buffer44, ptr @texto_literal.8)
  %strcat_second46 = call ptr @strcat(ptr %concat_buffer44, ptr %altura39)
  %left_len47 = call i64 @strlen(ptr %concat_buffer44)
  %right_len48 = call i64 @strlen(ptr @texto_literal.9)
  %total_len49 = add i64 %left_len47, %right_len48
  %total_len_plus_one50 = add i64 %total_len49, 1
  %concat_buffer51 = call ptr @malloc(i64 %total_len_plus_one50)
  %strcpy_first52 = call ptr @strcpy(ptr %concat_buffer51, ptr %concat_buffer44)
  %strcat_second53 = call ptr @strcat(ptr %concat_buffer51, ptr @texto_literal.9)
  %printf_call54 = call i32 (ptr, ...) @printf(ptr @formato_texto.3, ptr %concat_buffer51)
  %puts_prompt55 = call i32 @puts(ptr @texto_literal.10)
  %input_buffer56 = call ptr @malloc(i64 256)
  %e_nulo59 = icmp eq ptr %input_buffer56, null
  br i1 %e_nulo59, label %alocacao_falhou57, label %alocacao_sucesso58

alocacao_falhou57:                                ; preds = %alocacao_sucesso35
  %exit_call60 = call i32 @exit(i32 1)
  unreachable

alocacao_sucesso58:                               ; preds = %alocacao_sucesso35
  %scanf_call61 = call i64 (ptr, ...) @scanf(ptr @formato_texto, ptr %input_buffer56)
  %peso = alloca ptr, align 8
  store ptr %input_buffer56, ptr %peso, align 8
  %peso62 = load ptr, ptr %peso, align 8
  %left_len63 = call i64 @strlen(ptr @texto_literal.11)
  %right_len64 = call i64 @strlen(ptr %peso62)
  %total_len65 = add i64 %left_len63, %right_len64
  %total_len_plus_one66 = add i64 %total_len65, 1
  %concat_buffer67 = call ptr @malloc(i64 %total_len_plus_one66)
  %strcpy_first68 = call ptr @strcpy(ptr %concat_buffer67, ptr @texto_literal.11)
  %strcat_second69 = call ptr @strcat(ptr %concat_buffer67, ptr %peso62)
  %left_len70 = call i64 @strlen(ptr %concat_buffer67)
  %right_len71 = call i64 @strlen(ptr @texto_literal.12)
  %total_len72 = add i64 %left_len70, %right_len71
  %total_len_plus_one73 = add i64 %total_len72, 1
  %concat_buffer74 = call ptr @malloc(i64 %total_len_plus_one73)
  %strcpy_first75 = call ptr @strcpy(ptr %concat_buffer74, ptr %concat_buffer67)
  %strcat_second76 = call ptr @strcat(ptr %concat_buffer74, ptr @texto_literal.12)
  %printf_call77 = call i32 (ptr, ...) @printf(ptr @formato_texto.3, ptr %concat_buffer74)
  %liberar_buffer = call i32 @free(ptr %input_buffer)
  %liberar_buffer78 = call i32 @free(ptr %input_buffer10)
  %liberar_buffer79 = call i32 @free(ptr %input_buffer33)
  %liberar_buffer80 = call i32 @free(ptr %input_buffer56)
  ret i32 0
}

declare i64 @scanf(ptr, ...)

declare ptr @malloc(i64)

declare i32 @puts(ptr)

declare i32 @exit(i32)

declare i64 @strlen(ptr)

declare ptr @strcpy(ptr, ptr)

declare ptr @strcat(ptr, ptr)

declare i32 @printf(ptr, ...)

declare i32 @free(ptr)
