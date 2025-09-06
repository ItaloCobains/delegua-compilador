; ModuleID = 'program'
source_filename = "program"

@string_literal = global [33 x i8] c"Testando fun\C3\A7\C3\B5es matem\C3\A1ticas:\00"
@format_str = global [4 x i8] c"%s\0A\00"
@string_literal.1 = global [17 x i8] c"Absoluto de -5: \00"
@int_format = global [5 x i8] c"%lld\00"
@format_str.2 = global [4 x i8] c"%s\0A\00"
@string_literal.3 = global [19 x i8] c"Pot\C3\AAncia de 2^3: \00"
@int_format.4 = global [5 x i8] c"%lld\00"
@format_str.5 = global [4 x i8] c"%s\0A\00"
@string_literal.6 = global [22 x i8] c"Raiz quadrada de 16: \00"
@int_format.7 = global [5 x i8] c"%lld\00"
@format_str.8 = global [4 x i8] c"%s\0A\00"
@string_literal.9 = global [22 x i8] c"Raiz quadrada de 25: \00"
@int_format.10 = global [5 x i8] c"%lld\00"
@format_str.11 = global [4 x i8] c"%s\0A\00"

define i64 @main() {
entry:
  %printf_call = call i64 (ptr, ...) @printf(ptr @format_str, ptr @string_literal)
  %module_call = call i64 @matematica_absoluto(i64 -5)
  %int_str_buffer = call ptr @malloc(i64 20)
  %sprintf_call = call i64 (ptr, ptr, ...) @sprintf(ptr %int_str_buffer, ptr @int_format, i64 %module_call)
  %left_len = call i64 @strlen(ptr @string_literal.1)
  %right_len = call i64 @strlen(ptr %int_str_buffer)
  %total_len = add i64 %left_len, %right_len
  %total_len_plus_one = add i64 %total_len, 1
  %concat_buffer = call ptr @malloc(i64 %total_len_plus_one)
  %strcpy_first = call ptr @strcpy(ptr %concat_buffer, ptr @string_literal.1)
  %strcat_second = call ptr @strcat(ptr %concat_buffer, ptr %int_str_buffer)
  %printf_call1 = call i64 (ptr, ...) @printf(ptr @format_str.2, ptr %concat_buffer)
  %module_call2 = call i64 @matematica_potencia(i64 2, i64 3)
  %int_str_buffer3 = call ptr @malloc(i64 20)
  %sprintf_call4 = call i64 (ptr, ptr, ...) @sprintf(ptr %int_str_buffer3, ptr @int_format.4, i64 %module_call2)
  %left_len5 = call i64 @strlen(ptr @string_literal.3)
  %right_len6 = call i64 @strlen(ptr %int_str_buffer3)
  %total_len7 = add i64 %left_len5, %right_len6
  %total_len_plus_one8 = add i64 %total_len7, 1
  %concat_buffer9 = call ptr @malloc(i64 %total_len_plus_one8)
  %strcpy_first10 = call ptr @strcpy(ptr %concat_buffer9, ptr @string_literal.3)
  %strcat_second11 = call ptr @strcat(ptr %concat_buffer9, ptr %int_str_buffer3)
  %printf_call12 = call i64 (ptr, ...) @printf(ptr @format_str.5, ptr %concat_buffer9)
  %module_call13 = call i64 @matematica_raiz_quadrada(i64 16)
  %int_str_buffer14 = call ptr @malloc(i64 20)
  %sprintf_call15 = call i64 (ptr, ptr, ...) @sprintf(ptr %int_str_buffer14, ptr @int_format.7, i64 %module_call13)
  %left_len16 = call i64 @strlen(ptr @string_literal.6)
  %right_len17 = call i64 @strlen(ptr %int_str_buffer14)
  %total_len18 = add i64 %left_len16, %right_len17
  %total_len_plus_one19 = add i64 %total_len18, 1
  %concat_buffer20 = call ptr @malloc(i64 %total_len_plus_one19)
  %strcpy_first21 = call ptr @strcpy(ptr %concat_buffer20, ptr @string_literal.6)
  %strcat_second22 = call ptr @strcat(ptr %concat_buffer20, ptr %int_str_buffer14)
  %printf_call23 = call i64 (ptr, ...) @printf(ptr @format_str.8, ptr %concat_buffer20)
  %module_call24 = call i64 @matematica_raiz_quadrada(i64 25)
  %int_str_buffer25 = call ptr @malloc(i64 20)
  %sprintf_call26 = call i64 (ptr, ptr, ...) @sprintf(ptr %int_str_buffer25, ptr @int_format.10, i64 %module_call24)
  %left_len27 = call i64 @strlen(ptr @string_literal.9)
  %right_len28 = call i64 @strlen(ptr %int_str_buffer25)
  %total_len29 = add i64 %left_len27, %right_len28
  %total_len_plus_one30 = add i64 %total_len29, 1
  %concat_buffer31 = call ptr @malloc(i64 %total_len_plus_one30)
  %strcpy_first32 = call ptr @strcpy(ptr %concat_buffer31, ptr @string_literal.9)
  %strcat_second33 = call ptr @strcat(ptr %concat_buffer31, ptr %int_str_buffer25)
  %printf_call34 = call i64 (ptr, ...) @printf(ptr @format_str.11, ptr %concat_buffer31)
  ret i64 0
}

define i64 @matematica_absoluto(i64 %0) {
entry:
  %eh_negativo = icmp slt i64 %0, 0
  %menos_x = sub i64 0, %0
  %resultado_absoluto = select i1 %eh_negativo, i64 %menos_x, i64 %0
  ret i64 %resultado_absoluto
}

define i64 @matematica_potencia(i64 %0, i64 %1) {
entry:
  %resultado = alloca i64, align 8
  %contador = alloca i64, align 8
  store i64 1, ptr %resultado, align 4
  store i64 %1, ptr %contador, align 4
  br label %laco

laco:                                             ; preds = %corpo_laco, %entry
  %valor_contador = load i64, ptr %contador, align 4
  %condicao = icmp sgt i64 %valor_contador, 0
  br i1 %condicao, label %corpo_laco, label %apos

corpo_laco:                                       ; preds = %laco
  %valor_resultado = load i64, ptr %resultado, align 4
  %novo_resultado = mul i64 %valor_resultado, %0
  store i64 %novo_resultado, ptr %resultado, align 4
  %novo_contador = sub i64 %valor_contador, 1
  store i64 %novo_contador, ptr %contador, align 4
  br label %laco

apos:                                             ; preds = %laco
  %resultado_final = load i64, ptr %resultado, align 4
  ret i64 %resultado_final
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
