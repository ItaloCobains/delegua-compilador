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
  %right_len = call i64 @strlen(ptr %nome1)
  %total_len = add i64 6, %right_len
  %total_len_plus_one = add i64 %total_len, 1
  %concat_buffer = call ptr @malloc(i64 %total_len_plus_one)
  %e_nulo4 = icmp eq ptr %concat_buffer, null
  br i1 %e_nulo4, label %alocacao_falhou2, label %alocacao_sucesso3

alocacao_falhou2:                                 ; preds = %alocacao_sucesso
  %exit_call5 = call i32 @exit(i32 1)
  unreachable

alocacao_sucesso3:                                ; preds = %alocacao_sucesso
  %strcpy_first = call ptr @strcpy(ptr %concat_buffer, ptr @texto_literal.1)
  %strcat_second = call ptr @strcat(ptr %concat_buffer, ptr %nome1)
  %left_len = call i64 @strlen(ptr %concat_buffer)
  %total_len6 = add i64 %left_len, 1
  %total_len_plus_one7 = add i64 %total_len6, 1
  %concat_buffer8 = call ptr @malloc(i64 %total_len_plus_one7)
  %e_nulo11 = icmp eq ptr %concat_buffer8, null
  br i1 %e_nulo11, label %alocacao_falhou9, label %alocacao_sucesso10

alocacao_falhou9:                                 ; preds = %alocacao_sucesso3
  %exit_call12 = call i32 @exit(i32 1)
  unreachable

alocacao_sucesso10:                               ; preds = %alocacao_sucesso3
  %strcpy_first13 = call ptr @strcpy(ptr %concat_buffer8, ptr %concat_buffer)
  %strcat_second14 = call ptr @strcat(ptr %concat_buffer8, ptr @texto_literal.2)
  %printf_call = call i32 (ptr, ...) @printf(ptr @formato_texto.3, ptr %concat_buffer8)
  %liberar_temp = call i32 @free(ptr %concat_buffer)
  %liberar_temp15 = call i32 @free(ptr %concat_buffer8)
  %puts_prompt16 = call i32 @puts(ptr @texto_literal.4)
  %input_buffer17 = call ptr @malloc(i64 256)
  %e_nulo20 = icmp eq ptr %input_buffer17, null
  br i1 %e_nulo20, label %alocacao_falhou18, label %alocacao_sucesso19

alocacao_falhou18:                                ; preds = %alocacao_sucesso10
  %exit_call21 = call i32 @exit(i32 1)
  unreachable

alocacao_sucesso19:                               ; preds = %alocacao_sucesso10
  %scanf_call22 = call i64 (ptr, ...) @scanf(ptr @formato_texto, ptr %input_buffer17)
  %idade = alloca ptr, align 8
  store ptr %input_buffer17, ptr %idade, align 8
  %idade23 = load ptr, ptr %idade, align 8
  %right_len24 = call i64 @strlen(ptr %idade23)
  %total_len25 = add i64 10, %right_len24
  %total_len_plus_one26 = add i64 %total_len25, 1
  %concat_buffer27 = call ptr @malloc(i64 %total_len_plus_one26)
  %e_nulo30 = icmp eq ptr %concat_buffer27, null
  br i1 %e_nulo30, label %alocacao_falhou28, label %alocacao_sucesso29

alocacao_falhou28:                                ; preds = %alocacao_sucesso19
  %exit_call31 = call i32 @exit(i32 1)
  unreachable

alocacao_sucesso29:                               ; preds = %alocacao_sucesso19
  %strcpy_first32 = call ptr @strcpy(ptr %concat_buffer27, ptr @texto_literal.5)
  %strcat_second33 = call ptr @strcat(ptr %concat_buffer27, ptr %idade23)
  %left_len34 = call i64 @strlen(ptr %concat_buffer27)
  %total_len35 = add i64 %left_len34, 6
  %total_len_plus_one36 = add i64 %total_len35, 1
  %concat_buffer37 = call ptr @malloc(i64 %total_len_plus_one36)
  %e_nulo40 = icmp eq ptr %concat_buffer37, null
  br i1 %e_nulo40, label %alocacao_falhou38, label %alocacao_sucesso39

alocacao_falhou38:                                ; preds = %alocacao_sucesso29
  %exit_call41 = call i32 @exit(i32 1)
  unreachable

alocacao_sucesso39:                               ; preds = %alocacao_sucesso29
  %strcpy_first42 = call ptr @strcpy(ptr %concat_buffer37, ptr %concat_buffer27)
  %strcat_second43 = call ptr @strcat(ptr %concat_buffer37, ptr @texto_literal.6)
  %printf_call44 = call i32 (ptr, ...) @printf(ptr @formato_texto.3, ptr %concat_buffer37)
  %liberar_temp45 = call i32 @free(ptr %concat_buffer27)
  %liberar_temp46 = call i32 @free(ptr %concat_buffer37)
  %puts_prompt47 = call i32 @puts(ptr @texto_literal.7)
  %input_buffer48 = call ptr @malloc(i64 256)
  %e_nulo51 = icmp eq ptr %input_buffer48, null
  br i1 %e_nulo51, label %alocacao_falhou49, label %alocacao_sucesso50

alocacao_falhou49:                                ; preds = %alocacao_sucesso39
  %exit_call52 = call i32 @exit(i32 1)
  unreachable

alocacao_sucesso50:                               ; preds = %alocacao_sucesso39
  %scanf_call53 = call i64 (ptr, ...) @scanf(ptr @formato_texto, ptr %input_buffer48)
  %altura = alloca ptr, align 8
  store ptr %input_buffer48, ptr %altura, align 8
  %altura54 = load ptr, ptr %altura, align 8
  %right_len55 = call i64 @strlen(ptr %altura54)
  %total_len56 = add i64 14, %right_len55
  %total_len_plus_one57 = add i64 %total_len56, 1
  %concat_buffer58 = call ptr @malloc(i64 %total_len_plus_one57)
  %e_nulo61 = icmp eq ptr %concat_buffer58, null
  br i1 %e_nulo61, label %alocacao_falhou59, label %alocacao_sucesso60

alocacao_falhou59:                                ; preds = %alocacao_sucesso50
  %exit_call62 = call i32 @exit(i32 1)
  unreachable

alocacao_sucesso60:                               ; preds = %alocacao_sucesso50
  %strcpy_first63 = call ptr @strcpy(ptr %concat_buffer58, ptr @texto_literal.8)
  %strcat_second64 = call ptr @strcat(ptr %concat_buffer58, ptr %altura54)
  %left_len65 = call i64 @strlen(ptr %concat_buffer58)
  %total_len66 = add i64 %left_len65, 8
  %total_len_plus_one67 = add i64 %total_len66, 1
  %concat_buffer68 = call ptr @malloc(i64 %total_len_plus_one67)
  %e_nulo71 = icmp eq ptr %concat_buffer68, null
  br i1 %e_nulo71, label %alocacao_falhou69, label %alocacao_sucesso70

alocacao_falhou69:                                ; preds = %alocacao_sucesso60
  %exit_call72 = call i32 @exit(i32 1)
  unreachable

alocacao_sucesso70:                               ; preds = %alocacao_sucesso60
  %strcpy_first73 = call ptr @strcpy(ptr %concat_buffer68, ptr %concat_buffer58)
  %strcat_second74 = call ptr @strcat(ptr %concat_buffer68, ptr @texto_literal.9)
  %printf_call75 = call i32 (ptr, ...) @printf(ptr @formato_texto.3, ptr %concat_buffer68)
  %liberar_temp76 = call i32 @free(ptr %concat_buffer58)
  %liberar_temp77 = call i32 @free(ptr %concat_buffer68)
  %puts_prompt78 = call i32 @puts(ptr @texto_literal.10)
  %input_buffer79 = call ptr @malloc(i64 256)
  %e_nulo82 = icmp eq ptr %input_buffer79, null
  br i1 %e_nulo82, label %alocacao_falhou80, label %alocacao_sucesso81

alocacao_falhou80:                                ; preds = %alocacao_sucesso70
  %exit_call83 = call i32 @exit(i32 1)
  unreachable

alocacao_sucesso81:                               ; preds = %alocacao_sucesso70
  %scanf_call84 = call i64 (ptr, ...) @scanf(ptr @formato_texto, ptr %input_buffer79)
  %peso = alloca ptr, align 8
  store ptr %input_buffer79, ptr %peso, align 8
  %peso85 = load ptr, ptr %peso, align 8
  %right_len86 = call i64 @strlen(ptr %peso85)
  %total_len87 = add i64 12, %right_len86
  %total_len_plus_one88 = add i64 %total_len87, 1
  %concat_buffer89 = call ptr @malloc(i64 %total_len_plus_one88)
  %e_nulo92 = icmp eq ptr %concat_buffer89, null
  br i1 %e_nulo92, label %alocacao_falhou90, label %alocacao_sucesso91

alocacao_falhou90:                                ; preds = %alocacao_sucesso81
  %exit_call93 = call i32 @exit(i32 1)
  unreachable

alocacao_sucesso91:                               ; preds = %alocacao_sucesso81
  %strcpy_first94 = call ptr @strcpy(ptr %concat_buffer89, ptr @texto_literal.11)
  %strcat_second95 = call ptr @strcat(ptr %concat_buffer89, ptr %peso85)
  %left_len96 = call i64 @strlen(ptr %concat_buffer89)
  %total_len97 = add i64 %left_len96, 4
  %total_len_plus_one98 = add i64 %total_len97, 1
  %concat_buffer99 = call ptr @malloc(i64 %total_len_plus_one98)
  %e_nulo102 = icmp eq ptr %concat_buffer99, null
  br i1 %e_nulo102, label %alocacao_falhou100, label %alocacao_sucesso101

alocacao_falhou100:                               ; preds = %alocacao_sucesso91
  %exit_call103 = call i32 @exit(i32 1)
  unreachable

alocacao_sucesso101:                              ; preds = %alocacao_sucesso91
  %strcpy_first104 = call ptr @strcpy(ptr %concat_buffer99, ptr %concat_buffer89)
  %strcat_second105 = call ptr @strcat(ptr %concat_buffer99, ptr @texto_literal.12)
  %printf_call106 = call i32 (ptr, ...) @printf(ptr @formato_texto.3, ptr %concat_buffer99)
  %liberar_temp107 = call i32 @free(ptr %concat_buffer89)
  %liberar_temp108 = call i32 @free(ptr %concat_buffer99)
  %liberar_buffer = call i32 @free(ptr %input_buffer)
  %liberar_buffer109 = call i32 @free(ptr %input_buffer17)
  %liberar_buffer110 = call i32 @free(ptr %input_buffer48)
  %liberar_buffer111 = call i32 @free(ptr %input_buffer79)
  ret i32 0
}

declare i64 @scanf(ptr, ...)

declare ptr @malloc(i64)

declare i32 @puts(ptr)

declare i32 @exit(i32)

declare ptr @strcpy(ptr, ptr)

declare ptr @strcat(ptr, ptr)

declare i64 @strlen(ptr)

declare i32 @printf(ptr, ...)

declare i32 @free(ptr)
