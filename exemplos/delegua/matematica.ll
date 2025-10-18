; ModuleID = 'program'
source_filename = "program"

@string_literal = global [33 x i8] c"Testando fun\C3\A7\C3\B5es matem\C3\A1ticas:\00"
@format_str = global [4 x i8] c"%s\0A\00"
@string_literal.1 = global [17 x i8] c"Absoluto de -5: \00"
@format_str.2 = global [5 x i8] c"%lld\00"
@string_literal.3 = global [19 x i8] c"Pot\C3\AAncia de 2^3: \00"
@string_literal.4 = global [22 x i8] c"Raiz quadrada de 16: \00"
@string_literal.5 = global [22 x i8] c"Raiz quadrada de 25: \00"

define i64 @main() {
entry:
  %printf_str_call = call i64 (ptr, ...) @printf(ptr @format_str, ptr @string_literal)
  %absoluto_call = call i64 @matematica_absoluto(i64 -5)
  %int_str_buffer = call ptr @malloc(i64 20)
  %sprintf_int = call i64 (ptr, ptr, ...) @sprintf(ptr %int_str_buffer, ptr @format_str.2, i64 %absoluto_call)
  %left_len = call i64 @strlen(ptr @string_literal.1)
  %right_len = call i64 @strlen(ptr %int_str_buffer)
  %total_len = add i64 %left_len, %right_len
  %total_len_plus_one = add i64 %total_len, 1
  %concat_buffer = call ptr @malloc(i64 %total_len_plus_one)
  %strcpy_first = call ptr @strcpy(ptr %concat_buffer, ptr @string_literal.1)
  %strcat_second = call ptr @strcat(ptr %concat_buffer, ptr %int_str_buffer)
  %printf_str_call1 = call i64 (ptr, ...) @printf(ptr @format_str, ptr %concat_buffer)
  %pow_result = alloca i64, align 8
  %pow_counter = alloca i64, align 8
  store i64 1, ptr %pow_result, align 4
  store i64 3, ptr %pow_counter, align 4
  br label %pow_loop

pow_loop:                                         ; preds = %pow_multiply, %entry
  %load_counter = load i64, ptr %pow_counter, align 4
  %counter_positive = icmp sgt i64 %load_counter, 0
  br i1 %counter_positive, label %pow_multiply, label %pow_exit

pow_exit:                                         ; preds = %pow_loop
  %final_pow_result = load i64, ptr %pow_result, align 4
  %int_str_buffer3 = call ptr @malloc(i64 20)
  %sprintf_int4 = call i64 (ptr, ptr, ...) @sprintf(ptr %int_str_buffer3, ptr @format_str.2, i64 %final_pow_result)
  %left_len5 = call i64 @strlen(ptr @string_literal.3)
  %right_len6 = call i64 @strlen(ptr %int_str_buffer3)
  %total_len7 = add i64 %left_len5, %right_len6
  %total_len_plus_one8 = add i64 %total_len7, 1
  %concat_buffer9 = call ptr @malloc(i64 %total_len_plus_one8)
  %strcpy_first10 = call ptr @strcpy(ptr %concat_buffer9, ptr @string_literal.3)
  %strcat_second11 = call ptr @strcat(ptr %concat_buffer9, ptr %int_str_buffer3)
  %printf_str_call12 = call i64 (ptr, ...) @printf(ptr @format_str, ptr %concat_buffer9)
  %raiz_quadrada_call = call i64 @matematica_raiz_quadrada(i64 16)
  %int_str_buffer13 = call ptr @malloc(i64 20)
  %sprintf_int14 = call i64 (ptr, ptr, ...) @sprintf(ptr %int_str_buffer13, ptr @format_str.2, i64 %raiz_quadrada_call)
  %left_len15 = call i64 @strlen(ptr @string_literal.4)
  %right_len16 = call i64 @strlen(ptr %int_str_buffer13)
  %total_len17 = add i64 %left_len15, %right_len16
  %total_len_plus_one18 = add i64 %total_len17, 1
  %concat_buffer19 = call ptr @malloc(i64 %total_len_plus_one18)
  %strcpy_first20 = call ptr @strcpy(ptr %concat_buffer19, ptr @string_literal.4)
  %strcat_second21 = call ptr @strcat(ptr %concat_buffer19, ptr %int_str_buffer13)
  %printf_str_call22 = call i64 (ptr, ...) @printf(ptr @format_str, ptr %concat_buffer19)
  %raiz_quadrada_call23 = call i64 @matematica_raiz_quadrada(i64 25)
  %int_str_buffer24 = call ptr @malloc(i64 20)
  %sprintf_int25 = call i64 (ptr, ptr, ...) @sprintf(ptr %int_str_buffer24, ptr @format_str.2, i64 %raiz_quadrada_call23)
  %left_len26 = call i64 @strlen(ptr @string_literal.5)
  %right_len27 = call i64 @strlen(ptr %int_str_buffer24)
  %total_len28 = add i64 %left_len26, %right_len27
  %total_len_plus_one29 = add i64 %total_len28, 1
  %concat_buffer30 = call ptr @malloc(i64 %total_len_plus_one29)
  %strcpy_first31 = call ptr @strcpy(ptr %concat_buffer30, ptr @string_literal.5)
  %strcat_second32 = call ptr @strcat(ptr %concat_buffer30, ptr %int_str_buffer24)
  %printf_str_call33 = call i64 (ptr, ...) @printf(ptr @format_str, ptr %concat_buffer30)
  ret i64 0

pow_multiply:                                     ; preds = %pow_loop
  %load_result = load i64, ptr %pow_result, align 4
  %pow_multiply2 = mul i64 %load_result, 2
  %decrement_counter = sub i64 %load_counter, 1
  store i64 %pow_multiply2, ptr %pow_result, align 4
  store i64 %decrement_counter, ptr %pow_counter, align 4
  br label %pow_loop
}

define i64 @matematica_absoluto(i64 %0) {
entry:
  %eh_negativo = icmp slt i64 %0, 0
  %menos_x = sub i64 0, %0
  %resultado_absoluto = select i1 %eh_negativo, i64 %menos_x, i64 %0
  ret i64 %resultado_absoluto
}

define i64 @matematica_raiz_quadrada(i64 %0) {
entry:
  %eh_zero = icmp eq i64 %0, 0
  %eh_um = icmp eq i64 %0, 1
  %eh_zero_ou_um = or i1 %eh_zero, %eh_um
  br i1 %eh_zero_ou_um, label %caso_especial, label %caso_normal

caso_especial:                                    ; preds = %entry
  ret i64 %0

caso_normal:                                      ; preds = %entry
  %baixo = alloca i64, align 8
  %alto = alloca i64, align 8
  %resultado = alloca i64, align 8
  store i64 1, ptr %baixo, align 4
  store i64 %0, ptr %alto, align 4
  br label %laco

fim:                                              ; No predecessors!
  unreachable

laco:                                             ; preds = %continuar, %caso_normal
  %valor_baixo = load i64, ptr %baixo, align 4
  %valor_alto = load i64, ptr %alto, align 4
  %condicao = icmp sle i64 %valor_baixo, %valor_alto
  br i1 %condicao, label %corpo_laco, label %apos_laco

corpo_laco:                                       ; preds = %laco
  %soma_meio = add i64 %valor_baixo, %valor_alto
  %meio = sdiv i64 %soma_meio, 2
  %meio_quadrado = mul i64 %meio, %meio
  %eh_exata = icmp eq i64 %meio_quadrado, %0
  br i1 %eh_exata, label %retornar_meio, label %continuar

retornar_meio:                                    ; preds = %corpo_laco
  ret i64 %meio

apos_laco:                                        ; preds = %laco
  %resultado_final = load i64, ptr %resultado, align 4
  ret i64 %resultado_final

continuar:                                        ; preds = %corpo_laco
  %comparacao = icmp sgt i64 %meio_quadrado, %0
  %meio_menos_um = sub i64 %meio, 1
  %novo_alto = select i1 %comparacao, i64 %meio_menos_um, i64 %valor_alto
  %meio_mais_um = add i64 %meio, 1
  %novo_baixo = select i1 %comparacao, i64 %valor_baixo, i64 %meio_mais_um
  store i64 %novo_alto, ptr %alto, align 4
  store i64 %novo_baixo, ptr %baixo, align 4
  store i64 %meio, ptr %resultado, align 4
  br label %laco
}

declare i64 @printf(ptr, ...)

declare ptr @malloc(i64)

declare i64 @sprintf(ptr, ptr, ...)

declare i64 @strlen(ptr)

declare ptr @strcpy(ptr, ptr)

declare ptr @strcat(ptr, ptr)
