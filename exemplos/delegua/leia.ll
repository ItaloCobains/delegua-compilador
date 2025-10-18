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
  %right_len = call i64 @strlen(ptr %input_buffer)
  %total_len = add i64 6, %right_len
  %total_len_plus_one = add i64 %total_len, 1
  %concat_buffer = call ptr @malloc(i64 %total_len_plus_one)
  %e_nulo3 = icmp eq ptr %concat_buffer, null
  br i1 %e_nulo3, label %alocacao_falhou1, label %alocacao_sucesso2

alocacao_falhou1:                                 ; preds = %alocacao_sucesso
  %exit_call4 = call i32 @exit(i32 1)
  unreachable

alocacao_sucesso2:                                ; preds = %alocacao_sucesso
  %strcpy_first = call ptr @strcpy(ptr %concat_buffer, ptr @texto_literal.1)
  %strcat_second = call ptr @strcat(ptr %concat_buffer, ptr %input_buffer)
  %total_len5 = add i64 %total_len, 1
  %total_len_plus_one6 = add i64 %total_len5, 1
  %concat_buffer7 = call ptr @malloc(i64 %total_len_plus_one6)
  %e_nulo10 = icmp eq ptr %concat_buffer7, null
  br i1 %e_nulo10, label %alocacao_falhou8, label %alocacao_sucesso9

alocacao_falhou8:                                 ; preds = %alocacao_sucesso2
  %exit_call11 = call i32 @exit(i32 1)
  unreachable

alocacao_sucesso9:                                ; preds = %alocacao_sucesso2
  %strcpy_first12 = call ptr @strcpy(ptr %concat_buffer7, ptr %concat_buffer)
  %strcat_second13 = call ptr @strcat(ptr %concat_buffer7, ptr @texto_literal.2)
  %printf_call = call i32 (ptr, ...) @printf(ptr @formato_texto.3, ptr %concat_buffer7)
  %liberar_temp = call i32 @free(ptr %concat_buffer)
  %liberar_temp14 = call i32 @free(ptr %concat_buffer7)
  %puts_prompt15 = call i32 @puts(ptr @texto_literal.4)
  %input_buffer16 = call ptr @malloc(i64 256)
  %e_nulo19 = icmp eq ptr %input_buffer16, null
  br i1 %e_nulo19, label %alocacao_falhou17, label %alocacao_sucesso18

alocacao_falhou17:                                ; preds = %alocacao_sucesso9
  %exit_call20 = call i32 @exit(i32 1)
  unreachable

alocacao_sucesso18:                               ; preds = %alocacao_sucesso9
  %scanf_call21 = call i64 (ptr, ...) @scanf(ptr @formato_texto, ptr %input_buffer16)
  %right_len22 = call i64 @strlen(ptr %input_buffer16)
  %total_len23 = add i64 10, %right_len22
  %total_len_plus_one24 = add i64 %total_len23, 1
  %concat_buffer25 = call ptr @malloc(i64 %total_len_plus_one24)
  %e_nulo28 = icmp eq ptr %concat_buffer25, null
  br i1 %e_nulo28, label %alocacao_falhou26, label %alocacao_sucesso27

alocacao_falhou26:                                ; preds = %alocacao_sucesso18
  %exit_call29 = call i32 @exit(i32 1)
  unreachable

alocacao_sucesso27:                               ; preds = %alocacao_sucesso18
  %strcpy_first30 = call ptr @strcpy(ptr %concat_buffer25, ptr @texto_literal.5)
  %strcat_second31 = call ptr @strcat(ptr %concat_buffer25, ptr %input_buffer16)
  %total_len32 = add i64 %total_len23, 6
  %total_len_plus_one33 = add i64 %total_len32, 1
  %concat_buffer34 = call ptr @malloc(i64 %total_len_plus_one33)
  %e_nulo37 = icmp eq ptr %concat_buffer34, null
  br i1 %e_nulo37, label %alocacao_falhou35, label %alocacao_sucesso36

alocacao_falhou35:                                ; preds = %alocacao_sucesso27
  %exit_call38 = call i32 @exit(i32 1)
  unreachable

alocacao_sucesso36:                               ; preds = %alocacao_sucesso27
  %strcpy_first39 = call ptr @strcpy(ptr %concat_buffer34, ptr %concat_buffer25)
  %strcat_second40 = call ptr @strcat(ptr %concat_buffer34, ptr @texto_literal.6)
  %printf_call41 = call i32 (ptr, ...) @printf(ptr @formato_texto.3, ptr %concat_buffer34)
  %liberar_temp42 = call i32 @free(ptr %concat_buffer25)
  %liberar_temp43 = call i32 @free(ptr %concat_buffer34)
  %puts_prompt44 = call i32 @puts(ptr @texto_literal.7)
  %input_buffer45 = call ptr @malloc(i64 256)
  %e_nulo48 = icmp eq ptr %input_buffer45, null
  br i1 %e_nulo48, label %alocacao_falhou46, label %alocacao_sucesso47

alocacao_falhou46:                                ; preds = %alocacao_sucesso36
  %exit_call49 = call i32 @exit(i32 1)
  unreachable

alocacao_sucesso47:                               ; preds = %alocacao_sucesso36
  %scanf_call50 = call i64 (ptr, ...) @scanf(ptr @formato_texto, ptr %input_buffer45)
  %right_len51 = call i64 @strlen(ptr %input_buffer45)
  %total_len52 = add i64 14, %right_len51
  %total_len_plus_one53 = add i64 %total_len52, 1
  %concat_buffer54 = call ptr @malloc(i64 %total_len_plus_one53)
  %e_nulo57 = icmp eq ptr %concat_buffer54, null
  br i1 %e_nulo57, label %alocacao_falhou55, label %alocacao_sucesso56

alocacao_falhou55:                                ; preds = %alocacao_sucesso47
  %exit_call58 = call i32 @exit(i32 1)
  unreachable

alocacao_sucesso56:                               ; preds = %alocacao_sucesso47
  %strcpy_first59 = call ptr @strcpy(ptr %concat_buffer54, ptr @texto_literal.8)
  %strcat_second60 = call ptr @strcat(ptr %concat_buffer54, ptr %input_buffer45)
  %total_len61 = add i64 %total_len52, 8
  %total_len_plus_one62 = add i64 %total_len61, 1
  %concat_buffer63 = call ptr @malloc(i64 %total_len_plus_one62)
  %e_nulo66 = icmp eq ptr %concat_buffer63, null
  br i1 %e_nulo66, label %alocacao_falhou64, label %alocacao_sucesso65

alocacao_falhou64:                                ; preds = %alocacao_sucesso56
  %exit_call67 = call i32 @exit(i32 1)
  unreachable

alocacao_sucesso65:                               ; preds = %alocacao_sucesso56
  %strcpy_first68 = call ptr @strcpy(ptr %concat_buffer63, ptr %concat_buffer54)
  %strcat_second69 = call ptr @strcat(ptr %concat_buffer63, ptr @texto_literal.9)
  %printf_call70 = call i32 (ptr, ...) @printf(ptr @formato_texto.3, ptr %concat_buffer63)
  %liberar_temp71 = call i32 @free(ptr %concat_buffer54)
  %liberar_temp72 = call i32 @free(ptr %concat_buffer63)
  %puts_prompt73 = call i32 @puts(ptr @texto_literal.10)
  %input_buffer74 = call ptr @malloc(i64 256)
  %e_nulo77 = icmp eq ptr %input_buffer74, null
  br i1 %e_nulo77, label %alocacao_falhou75, label %alocacao_sucesso76

alocacao_falhou75:                                ; preds = %alocacao_sucesso65
  %exit_call78 = call i32 @exit(i32 1)
  unreachable

alocacao_sucesso76:                               ; preds = %alocacao_sucesso65
  %scanf_call79 = call i64 (ptr, ...) @scanf(ptr @formato_texto, ptr %input_buffer74)
  %right_len80 = call i64 @strlen(ptr %input_buffer74)
  %total_len81 = add i64 12, %right_len80
  %total_len_plus_one82 = add i64 %total_len81, 1
  %concat_buffer83 = call ptr @malloc(i64 %total_len_plus_one82)
  %e_nulo86 = icmp eq ptr %concat_buffer83, null
  br i1 %e_nulo86, label %alocacao_falhou84, label %alocacao_sucesso85

alocacao_falhou84:                                ; preds = %alocacao_sucesso76
  %exit_call87 = call i32 @exit(i32 1)
  unreachable

alocacao_sucesso85:                               ; preds = %alocacao_sucesso76
  %strcpy_first88 = call ptr @strcpy(ptr %concat_buffer83, ptr @texto_literal.11)
  %strcat_second89 = call ptr @strcat(ptr %concat_buffer83, ptr %input_buffer74)
  %total_len90 = add i64 %total_len81, 4
  %total_len_plus_one91 = add i64 %total_len90, 1
  %concat_buffer92 = call ptr @malloc(i64 %total_len_plus_one91)
  %e_nulo95 = icmp eq ptr %concat_buffer92, null
  br i1 %e_nulo95, label %alocacao_falhou93, label %alocacao_sucesso94

alocacao_falhou93:                                ; preds = %alocacao_sucesso85
  %exit_call96 = call i32 @exit(i32 1)
  unreachable

alocacao_sucesso94:                               ; preds = %alocacao_sucesso85
  %strcpy_first97 = call ptr @strcpy(ptr %concat_buffer92, ptr %concat_buffer83)
  %strcat_second98 = call ptr @strcat(ptr %concat_buffer92, ptr @texto_literal.12)
  %printf_call99 = call i32 (ptr, ...) @printf(ptr @formato_texto.3, ptr %concat_buffer92)
  %liberar_temp100 = call i32 @free(ptr %concat_buffer83)
  %liberar_temp101 = call i32 @free(ptr %concat_buffer92)
  %liberar_buffer = call i32 @free(ptr %input_buffer)
  %liberar_buffer102 = call i32 @free(ptr %input_buffer16)
  %liberar_buffer103 = call i32 @free(ptr %input_buffer45)
  %liberar_buffer104 = call i32 @free(ptr %input_buffer74)
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
