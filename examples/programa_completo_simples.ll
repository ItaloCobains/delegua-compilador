; ModuleID = 'program'
source_filename = "program"

@string_literal = global [34 x i8] c"=== PROGRAMA DELEGUA COMPLETO ===\00"
@format_str = global [4 x i8] c"%s\0A\00"
@string_literal.1 = global [8 x i8] c"Delegua\00"
@string_literal.2 = global [28 x i8] c"--- VARI\C3\81VEIS B\C3\81SICAS ---\00"
@string_literal.3 = global [10 x i8] c"N\C3\BAmero: \00"
@format_str.4 = global [6 x i8] c"%lld\0A\00"
@string_literal.5 = global [7 x i8] c"Nome: \00"
@string_literal.6 = global [33 x i8] c"--- OPERA\C3\87\C3\95ES ARITM\C3\89TICAS ---\00"
@string_literal.7 = global [11 x i8] c"42 + 58 = \00"
@string_literal.8 = global [10 x i8] c"42 * 2 = \00"
@string_literal.9 = global [11 x i8] c"42 ** 2 = \00"
@string_literal.10 = global [10 x i8] c"42 / 6 = \00"
@string_literal.11 = global [10 x i8] c"42 % 5 = \00"
@string_literal.12 = global [30 x i8] c"--- INCREMENTO/DECREMENTO ---\00"
@string_literal.13 = global [19 x i8] c"Contador inicial: \00"
@string_literal.14 = global [11 x i8] c"Ap\C3\B3s ++: \00"
@string_literal.15 = global [11 x i8] c"Ap\C3\B3s --: \00"
@string_literal.16 = global [28 x i8] c"--- FUN\C3\87\C3\95ES DE STRING ---\00"
@string_literal.17 = global [12 x i8] c"Hello World\00"
@string_literal.18 = global [17 x i8] c"Texto original: \00"
@string_literal.19 = global [14 x i8] c"Comprimento: \00"
@string_literal.20 = global [13 x i8] c"Mai\C3\BAscula: \00"
@string_literal.21 = global [13 x i8] c"Min\C3\BAscula: \00"
@string_literal.22 = global [31 x i8] c"--- FUN\C3\87\C3\95ES MATEM\C3\81TICAS ---\00"
@string_literal.23 = global [17 x i8] c"absoluto(-15) = \00"
@string_literal.24 = global [22 x i8] c"raiz_quadrada(144) = \00"
@string_literal.25 = global [18 x i8] c"potencia(3, 4) = \00"
@string_literal.26 = global [22 x i8] c"--- COMPARA\C3\87\C3\95ES ---\00"
@string_literal.27 = global [19 x i8] c"10 \C3\A9 menor que 20\00"
@string_literal.28 = global [19 x i8] c"20 \C3\A9 maior que 10\00"
@string_literal.29 = global [21 x i8] c"--- CONDICIONAIS ---\00"
@string_literal.30 = global [24 x i8] c"N\C3\BAmero \C3\A9 maior que 40\00"
@string_literal.31 = global [24 x i8] c"N\C3\BAmero \C3\A9 maior que 20\00"
@string_literal.32 = global [23 x i8] c"N\C3\BAmero \C3\A9 20 ou menor\00"
@string_literal.33 = global [15 x i8] c"--- ARRAYS ---\00"
@string_literal.34 = global [20 x i8] c"Primeiro elemento: \00"
@string_literal.35 = global [19 x i8] c"\C3\9Altimo elemento: \00"
@string_literal.36 = global [31 x i8] c"--- OPERA\C3\87\C3\95ES COMBINADAS ---\00"
@string_literal.37 = global [49 x i8] c"absoluto(potencia(-2, 3)) + raiz_quadrada(64) = \00"
@string_literal.38 = global [12 x i8] c"TeSte MiXtO\00"
@string_literal.39 = global [39 x i8] c"maiuscula(minuscula('TeSte MiXtO')) = \00"
@string_literal.40 = global [24 x i8] c"--- CASOS ESPECIAIS ---\00"
@string_literal.41 = global [20 x i8] c"raiz_quadrada(0) = \00"
@string_literal.42 = global [20 x i8] c"raiz_quadrada(1) = \00"
@string_literal.43 = global [18 x i8] c"potencia(2, 0) = \00"
@string_literal.44 = global [15 x i8] c"absoluto(0) = \00"
@string_literal.45 = global [6 x i8] c"teste\00"
@string_literal.46 = global [22 x i8] c"Express\C3\A3o complexa: \00"
@string_literal.47 = global [24 x i8] c"--- FIM DO PROGRAMA ---\00"

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

define i64 @main() {
entry:
  %printf_str_call = call i64 (ptr, ...) @printf(ptr @format_str, ptr @string_literal)
  %numero = alloca i64, align 8
  store i64 42, ptr %numero, align 4
  %nome = alloca ptr, align 8
  store ptr @string_literal.1, ptr %nome, align 8
  %negativo = alloca i64, align 8
  store i64 -15, ptr %negativo, align 4
  %printf_str_call1 = call i64 (ptr, ...) @printf(ptr @format_str, ptr @string_literal.2)
  %printf_str_call2 = call i64 (ptr, ...) @printf(ptr @format_str, ptr @string_literal.3)
  %numero3 = load i64, ptr %numero, align 4
  %printf_int_call = call i64 (ptr, ...) @printf(ptr @format_str.4, i64 %numero3)
  %printf_str_call4 = call i64 (ptr, ...) @printf(ptr @format_str, ptr @string_literal.5)
  %nome5 = load ptr, ptr %nome, align 8
  %printf_str_call6 = call i64 (ptr, ...) @printf(ptr @format_str, ptr %nome5)
  %printf_str_call7 = call i64 (ptr, ...) @printf(ptr @format_str, ptr @string_literal.6)
  %numero8 = load i64, ptr %numero, align 4
  %add = add i64 %numero8, 58
  %soma = alloca i64, align 8
  store i64 %add, ptr %soma, align 4
  %numero9 = load i64, ptr %numero, align 4
  %mul = mul i64 %numero9, 2
  %produto = alloca i64, align 8
  store i64 %mul, ptr %produto, align 4
  %numero10 = load i64, ptr %numero, align 4
  %pow_result = alloca i64, align 8
  %pow_counter = alloca i64, align 8
  store i64 1, ptr %pow_result, align 4
  store i64 2, ptr %pow_counter, align 4
  br label %pow_loop

pow_loop:                                         ; preds = %pow_multiply, %entry
  %load_counter = load i64, ptr %pow_counter, align 4
  %counter_positive = icmp sgt i64 %load_counter, 0
  br i1 %counter_positive, label %pow_multiply, label %pow_exit

pow_exit:                                         ; preds = %pow_loop
  %final_pow_result = load i64, ptr %pow_result, align 4
  %potencia_result = alloca i64, align 8
  store i64 %final_pow_result, ptr %potencia_result, align 4
  %numero12 = load i64, ptr %numero, align 4
  %div = sdiv i64 %numero12, 6
  %divisao = alloca i64, align 8
  store i64 %div, ptr %divisao, align 4
  %numero13 = load i64, ptr %numero, align 4
  %mod = srem i64 %numero13, 5
  %resto = alloca i64, align 8
  store i64 %mod, ptr %resto, align 4
  %printf_str_call14 = call i64 (ptr, ...) @printf(ptr @format_str, ptr @string_literal.7)
  %soma15 = load i64, ptr %soma, align 4
  %printf_int_call16 = call i64 (ptr, ...) @printf(ptr @format_str.4, i64 %soma15)
  %printf_str_call17 = call i64 (ptr, ...) @printf(ptr @format_str, ptr @string_literal.8)
  %produto18 = load i64, ptr %produto, align 4
  %printf_int_call19 = call i64 (ptr, ...) @printf(ptr @format_str.4, i64 %produto18)
  %printf_str_call20 = call i64 (ptr, ...) @printf(ptr @format_str, ptr @string_literal.9)
  %potencia_result21 = load i64, ptr %potencia_result, align 4
  %printf_int_call22 = call i64 (ptr, ...) @printf(ptr @format_str.4, i64 %potencia_result21)
  %printf_str_call23 = call i64 (ptr, ...) @printf(ptr @format_str, ptr @string_literal.10)
  %divisao24 = load i64, ptr %divisao, align 4
  %printf_int_call25 = call i64 (ptr, ...) @printf(ptr @format_str.4, i64 %divisao24)
  %printf_str_call26 = call i64 (ptr, ...) @printf(ptr @format_str, ptr @string_literal.11)
  %resto27 = load i64, ptr %resto, align 4
  %printf_int_call28 = call i64 (ptr, ...) @printf(ptr @format_str.4, i64 %resto27)
  %contador = alloca i64, align 8
  store i64 10, ptr %contador, align 4
  %printf_str_call29 = call i64 (ptr, ...) @printf(ptr @format_str, ptr @string_literal.12)
  %printf_str_call30 = call i64 (ptr, ...) @printf(ptr @format_str, ptr @string_literal.13)
  %contador31 = load i64, ptr %contador, align 4
  %printf_int_call32 = call i64 (ptr, ...) @printf(ptr @format_str.4, i64 %contador31)
  %load_contador = load i64, ptr %contador, align 4
  %inc_contador = add i64 %load_contador, 1
  store i64 %inc_contador, ptr %contador, align 4
  %printf_str_call33 = call i64 (ptr, ...) @printf(ptr @format_str, ptr @string_literal.14)
  %contador34 = load i64, ptr %contador, align 4
  %printf_int_call35 = call i64 (ptr, ...) @printf(ptr @format_str.4, i64 %contador34)
  %load_contador36 = load i64, ptr %contador, align 4
  %dec_contador = sub i64 %load_contador36, 1
  store i64 %dec_contador, ptr %contador, align 4
  %printf_str_call37 = call i64 (ptr, ...) @printf(ptr @format_str, ptr @string_literal.15)
  %contador38 = load i64, ptr %contador, align 4
  %printf_int_call39 = call i64 (ptr, ...) @printf(ptr @format_str.4, i64 %contador38)
  %printf_str_call40 = call i64 (ptr, ...) @printf(ptr @format_str, ptr @string_literal.16)
  %texto = alloca ptr, align 8
  store ptr @string_literal.17, ptr %texto, align 8
  %texto41 = load ptr, ptr %texto, align 8
  %strlen_call = call i64 @strlen(ptr %texto41)
  %tamanho = alloca i64, align 8
  store i64 %strlen_call, ptr %tamanho, align 4
  %texto42 = load ptr, ptr %texto, align 8
  %strlen_call43 = call i64 @strlen(ptr %texto42)
  %buffer_size = add i64 %strlen_call43, 1
  %upper_buffer = call ptr @malloc(i64 %buffer_size)
  %strcpy_call = call ptr @strcpy(ptr %upper_buffer, ptr %texto42)
  %upper_counter = alloca i64, align 8
  store i64 0, ptr %upper_counter, align 4
  br label %upper_loop

pow_multiply:                                     ; preds = %pow_loop
  %load_result = load i64, ptr %pow_result, align 4
  %pow_multiply11 = mul i64 %load_result, %numero10
  %decrement_counter = sub i64 %load_counter, 1
  store i64 %pow_multiply11, ptr %pow_result, align 4
  store i64 %decrement_counter, ptr %pow_counter, align 4
  br label %pow_loop

upper_loop:                                       ; preds = %upper_body, %pow_exit
  %load_counter44 = load i64, ptr %upper_counter, align 4
  %is_end = icmp eq i64 %load_counter44, %strlen_call43
  br i1 %is_end, label %upper_done, label %upper_body

upper_body:                                       ; preds = %upper_loop
  %char_ptr = getelementptr i8, ptr %upper_buffer, i64 %load_counter44
  %char_val = load i8, ptr %char_ptr, align 1
  %char_ext = zext i8 %char_val to i64
  %is_ge_a = icmp sge i64 %char_ext, 97
  %is_le_z = icmp sle i64 %char_ext, 122
  %is_lowercase = and i1 %is_ge_a, %is_le_z
  %upper_char = sub i64 %char_ext, 32
  %final_char = select i1 %is_lowercase, i64 %upper_char, i64 %char_ext
  %final_char_i8 = trunc i64 %final_char to i8
  store i8 %final_char_i8, ptr %char_ptr, align 1
  %increment = add i64 %load_counter44, 1
  store i64 %increment, ptr %upper_counter, align 4
  br label %upper_loop

upper_done:                                       ; preds = %upper_loop
  %maiusculo = alloca ptr, align 8
  store ptr %upper_buffer, ptr %maiusculo, align 8
  %texto45 = load ptr, ptr %texto, align 8
  %strlen_call46 = call i64 @strlen(ptr %texto45)
  %buffer_size47 = add i64 %strlen_call46, 1
  %lower_buffer = call ptr @malloc(i64 %buffer_size47)
  %strcpy_call48 = call ptr @strcpy(ptr %lower_buffer, ptr %texto45)
  %lower_counter = alloca i64, align 8
  store i64 0, ptr %lower_counter, align 4
  br label %lower_loop

lower_loop:                                       ; preds = %lower_body, %upper_done
  %load_counter49 = load i64, ptr %lower_counter, align 4
  %is_end50 = icmp eq i64 %load_counter49, %strlen_call46
  br i1 %is_end50, label %lower_done, label %lower_body

lower_body:                                       ; preds = %lower_loop
  %char_ptr51 = getelementptr i8, ptr %lower_buffer, i64 %load_counter49
  %char_val52 = load i8, ptr %char_ptr51, align 1
  %char_ext53 = zext i8 %char_val52 to i64
  %is_ge_A = icmp sge i64 %char_ext53, 65
  %is_le_Z = icmp sle i64 %char_ext53, 90
  %is_uppercase = and i1 %is_ge_A, %is_le_Z
  %lower_char = add i64 %char_ext53, 32
  %final_char54 = select i1 %is_uppercase, i64 %lower_char, i64 %char_ext53
  %final_char_i855 = trunc i64 %final_char54 to i8
  store i8 %final_char_i855, ptr %char_ptr51, align 1
  %increment56 = add i64 %load_counter49, 1
  store i64 %increment56, ptr %lower_counter, align 4
  br label %lower_loop

lower_done:                                       ; preds = %lower_loop
  %minusculo = alloca ptr, align 8
  store ptr %lower_buffer, ptr %minusculo, align 8
  %printf_str_call57 = call i64 (ptr, ...) @printf(ptr @format_str, ptr @string_literal.18)
  %texto58 = load ptr, ptr %texto, align 8
  %printf_str_call59 = call i64 (ptr, ...) @printf(ptr @format_str, ptr %texto58)
  %printf_str_call60 = call i64 (ptr, ...) @printf(ptr @format_str, ptr @string_literal.19)
  %tamanho61 = load i64, ptr %tamanho, align 4
  %printf_int_call62 = call i64 (ptr, ...) @printf(ptr @format_str.4, i64 %tamanho61)
  %printf_str_call63 = call i64 (ptr, ...) @printf(ptr @format_str, ptr @string_literal.20)
  %maiusculo64 = load ptr, ptr %maiusculo, align 8
  %printf_str_call65 = call i64 (ptr, ...) @printf(ptr @format_str, ptr %maiusculo64)
  %printf_str_call66 = call i64 (ptr, ...) @printf(ptr @format_str, ptr @string_literal.21)
  %minusculo67 = load ptr, ptr %minusculo, align 8
  %printf_str_call68 = call i64 (ptr, ...) @printf(ptr @format_str, ptr %minusculo67)
  %printf_str_call69 = call i64 (ptr, ...) @printf(ptr @format_str, ptr @string_literal.22)
  %negativo70 = load i64, ptr %negativo, align 4
  %absoluto_call = call i64 @matematica_absoluto(i64 %negativo70)
  %absoluto_val = alloca i64, align 8
  store i64 %absoluto_call, ptr %absoluto_val, align 4
  %raiz_quadrada_call = call i64 @matematica_raiz_quadrada(i64 144)
  %raiz = alloca i64, align 8
  store i64 %raiz_quadrada_call, ptr %raiz, align 4
  %potencia_call = call i64 @matematica_potencia(i64 3, i64 4)
  %pot = alloca i64, align 8
  store i64 %potencia_call, ptr %pot, align 4
  %printf_str_call71 = call i64 (ptr, ...) @printf(ptr @format_str, ptr @string_literal.23)
  %absoluto_val72 = load i64, ptr %absoluto_val, align 4
  %printf_int_call73 = call i64 (ptr, ...) @printf(ptr @format_str.4, i64 %absoluto_val72)
  %printf_str_call74 = call i64 (ptr, ...) @printf(ptr @format_str, ptr @string_literal.24)
  %raiz75 = load i64, ptr %raiz, align 4
  %printf_int_call76 = call i64 (ptr, ...) @printf(ptr @format_str.4, i64 %raiz75)
  %printf_str_call77 = call i64 (ptr, ...) @printf(ptr @format_str, ptr @string_literal.25)
  %pot78 = load i64, ptr %pot, align 4
  %printf_int_call79 = call i64 (ptr, ...) @printf(ptr @format_str.4, i64 %pot78)
  %printf_str_call80 = call i64 (ptr, ...) @printf(ptr @format_str, ptr @string_literal.26)
  %a = alloca i64, align 8
  store i64 10, ptr %a, align 4
  %b = alloca i64, align 8
  store i64 20, ptr %b, align 4
  %a81 = load i64, ptr %a, align 4
  %b82 = load i64, ptr %b, align 4
  %lt = icmp slt i64 %a81, %b82
  br i1 %lt, label %then, label %merge

then:                                             ; preds = %lower_done
  %printf_str_call83 = call i64 (ptr, ...) @printf(ptr @format_str, ptr @string_literal.27)
  br label %merge

merge:                                            ; preds = %then, %lower_done
  %b84 = load i64, ptr %b, align 4
  %a85 = load i64, ptr %a, align 4
  %gt = icmp sgt i64 %b84, %a85
  br i1 %gt, label %then86, label %merge87

then86:                                           ; preds = %merge
  %printf_str_call88 = call i64 (ptr, ...) @printf(ptr @format_str, ptr @string_literal.28)
  br label %merge87

merge87:                                          ; preds = %then86, %merge
  %printf_str_call89 = call i64 (ptr, ...) @printf(ptr @format_str, ptr @string_literal.29)
  %numero90 = load i64, ptr %numero, align 4
  %gt91 = icmp sgt i64 %numero90, 40
  br i1 %gt91, label %then92, label %else_if_start

then92:                                           ; preds = %merge87
  %printf_str_call94 = call i64 (ptr, ...) @printf(ptr @format_str, ptr @string_literal.30)
  br label %merge93

else_if_start:                                    ; preds = %merge87
  %numero95 = load i64, ptr %numero, align 4
  %gt96 = icmp sgt i64 %numero95, 20
  br i1 %gt96, label %else_if_then_0, label %final_else

merge93:                                          ; preds = %final_else, %else_if_then_0, %then92
  %printf_str_call99 = call i64 (ptr, ...) @printf(ptr @format_str, ptr @string_literal.33)
  %array_alloc = call ptr @malloc(i64 48)
  store i64 5, ptr %array_alloc, align 4
  %array_elem_ptr_0 = getelementptr i64, ptr %array_alloc, i64 1
  store i64 1, ptr %array_elem_ptr_0, align 4
  %array_elem_ptr_1 = getelementptr i64, ptr %array_alloc, i64 2
  store i64 2, ptr %array_elem_ptr_1, align 4
  %array_elem_ptr_2 = getelementptr i64, ptr %array_alloc, i64 3
  store i64 3, ptr %array_elem_ptr_2, align 4
  %array_elem_ptr_3 = getelementptr i64, ptr %array_alloc, i64 4
  store i64 4, ptr %array_elem_ptr_3, align 4
  %array_elem_ptr_4 = getelementptr i64, ptr %array_alloc, i64 5
  store i64 5, ptr %array_elem_ptr_4, align 4
  %numeros = alloca ptr, align 8
  store ptr %array_alloc, ptr %numeros, align 8
  %numeros100 = load ptr, ptr %numeros, align 8
  %array_element_ptr = getelementptr i64, ptr %numeros100, i64 1
  %array_element = load i64, ptr %array_element_ptr, align 4
  %primeiro = alloca i64, align 8
  store i64 %array_element, ptr %primeiro, align 4
  %numeros101 = load ptr, ptr %numeros, align 8
  %array_element_ptr102 = getelementptr i64, ptr %numeros101, i64 5
  %array_element103 = load i64, ptr %array_element_ptr102, align 4
  %ultimo = alloca i64, align 8
  store i64 %array_element103, ptr %ultimo, align 4
  %printf_str_call104 = call i64 (ptr, ...) @printf(ptr @format_str, ptr @string_literal.34)
  %primeiro105 = load i64, ptr %primeiro, align 4
  %printf_int_call106 = call i64 (ptr, ...) @printf(ptr @format_str.4, i64 %primeiro105)
  %printf_str_call107 = call i64 (ptr, ...) @printf(ptr @format_str, ptr @string_literal.35)
  %ultimo108 = load i64, ptr %ultimo, align 4
  %printf_int_call109 = call i64 (ptr, ...) @printf(ptr @format_str.4, i64 %ultimo108)
  %printf_str_call110 = call i64 (ptr, ...) @printf(ptr @format_str, ptr @string_literal.36)
  %potencia_call111 = call i64 @matematica_potencia(i64 -2, i64 3)
  %absoluto_call112 = call i64 @matematica_absoluto(i64 %potencia_call111)
  %raiz_quadrada_call113 = call i64 @matematica_raiz_quadrada(i64 64)
  %add114 = add i64 %absoluto_call112, %raiz_quadrada_call113
  %resultado_complexo = alloca i64, align 8
  store i64 %add114, ptr %resultado_complexo, align 4
  %printf_str_call115 = call i64 (ptr, ...) @printf(ptr @format_str, ptr @string_literal.37)
  %resultado_complexo116 = load i64, ptr %resultado_complexo, align 4
  %printf_int_call117 = call i64 (ptr, ...) @printf(ptr @format_str.4, i64 %resultado_complexo116)
  %strlen_call118 = call i64 @strlen(ptr @string_literal.38)
  %buffer_size119 = add i64 %strlen_call118, 1
  %lower_buffer120 = call ptr @malloc(i64 %buffer_size119)
  %strcpy_call121 = call ptr @strcpy(ptr %lower_buffer120, ptr @string_literal.38)
  %lower_counter125 = alloca i64, align 8
  store i64 0, ptr %lower_counter125, align 4
  br label %lower_loop122

else_if_then_0:                                   ; preds = %else_if_start
  %printf_str_call97 = call i64 (ptr, ...) @printf(ptr @format_str, ptr @string_literal.31)
  br label %merge93

final_else:                                       ; preds = %else_if_start
  %printf_str_call98 = call i64 (ptr, ...) @printf(ptr @format_str, ptr @string_literal.32)
  br label %merge93

lower_loop122:                                    ; preds = %lower_body123, %merge93
  %load_counter126 = load i64, ptr %lower_counter125, align 4
  %is_end127 = icmp eq i64 %load_counter126, %strlen_call118
  br i1 %is_end127, label %lower_done124, label %lower_body123

lower_body123:                                    ; preds = %lower_loop122
  %char_ptr128 = getelementptr i8, ptr %lower_buffer120, i64 %load_counter126
  %char_val129 = load i8, ptr %char_ptr128, align 1
  %char_ext130 = zext i8 %char_val129 to i64
  %is_ge_A131 = icmp sge i64 %char_ext130, 65
  %is_le_Z132 = icmp sle i64 %char_ext130, 90
  %is_uppercase133 = and i1 %is_ge_A131, %is_le_Z132
  %lower_char134 = add i64 %char_ext130, 32
  %final_char135 = select i1 %is_uppercase133, i64 %lower_char134, i64 %char_ext130
  %final_char_i8136 = trunc i64 %final_char135 to i8
  store i8 %final_char_i8136, ptr %char_ptr128, align 1
  %increment137 = add i64 %load_counter126, 1
  store i64 %increment137, ptr %lower_counter125, align 4
  br label %lower_loop122

lower_done124:                                    ; preds = %lower_loop122
  %strlen_call138 = call i64 @strlen(ptr %lower_buffer120)
  %buffer_size139 = add i64 %strlen_call138, 1
  %upper_buffer140 = call ptr @malloc(i64 %buffer_size139)
  %strcpy_call141 = call ptr @strcpy(ptr %upper_buffer140, ptr %lower_buffer120)
  %upper_counter145 = alloca i64, align 8
  store i64 0, ptr %upper_counter145, align 4
  br label %upper_loop142

upper_loop142:                                    ; preds = %upper_body143, %lower_done124
  %load_counter146 = load i64, ptr %upper_counter145, align 4
  %is_end147 = icmp eq i64 %load_counter146, %strlen_call138
  br i1 %is_end147, label %upper_done144, label %upper_body143

upper_body143:                                    ; preds = %upper_loop142
  %char_ptr148 = getelementptr i8, ptr %upper_buffer140, i64 %load_counter146
  %char_val149 = load i8, ptr %char_ptr148, align 1
  %char_ext150 = zext i8 %char_val149 to i64
  %is_ge_a151 = icmp sge i64 %char_ext150, 97
  %is_le_z152 = icmp sle i64 %char_ext150, 122
  %is_lowercase153 = and i1 %is_ge_a151, %is_le_z152
  %upper_char154 = sub i64 %char_ext150, 32
  %final_char155 = select i1 %is_lowercase153, i64 %upper_char154, i64 %char_ext150
  %final_char_i8156 = trunc i64 %final_char155 to i8
  store i8 %final_char_i8156, ptr %char_ptr148, align 1
  %increment157 = add i64 %load_counter146, 1
  store i64 %increment157, ptr %upper_counter145, align 4
  br label %upper_loop142

upper_done144:                                    ; preds = %upper_loop142
  %texto_processado = alloca ptr, align 8
  store ptr %upper_buffer140, ptr %texto_processado, align 8
  %printf_str_call158 = call i64 (ptr, ...) @printf(ptr @format_str, ptr @string_literal.39)
  %texto_processado159 = load ptr, ptr %texto_processado, align 8
  %printf_str_call160 = call i64 (ptr, ...) @printf(ptr @format_str, ptr %texto_processado159)
  %printf_str_call161 = call i64 (ptr, ...) @printf(ptr @format_str, ptr @string_literal.40)
  %zero = alloca i64, align 8
  store i64 0, ptr %zero, align 4
  %um = alloca i64, align 8
  store i64 1, ptr %um, align 4
  %printf_str_call162 = call i64 (ptr, ...) @printf(ptr @format_str, ptr @string_literal.41)
  %zero163 = load i64, ptr %zero, align 4
  %raiz_quadrada_call164 = call i64 @matematica_raiz_quadrada(i64 %zero163)
  %printf_int_call165 = call i64 (ptr, ...) @printf(ptr @format_str.4, i64 %raiz_quadrada_call164)
  %printf_str_call166 = call i64 (ptr, ...) @printf(ptr @format_str, ptr @string_literal.42)
  %um167 = load i64, ptr %um, align 4
  %raiz_quadrada_call168 = call i64 @matematica_raiz_quadrada(i64 %um167)
  %printf_int_call169 = call i64 (ptr, ...) @printf(ptr @format_str.4, i64 %raiz_quadrada_call168)
  %printf_str_call170 = call i64 (ptr, ...) @printf(ptr @format_str, ptr @string_literal.43)
  %potencia_call171 = call i64 @matematica_potencia(i64 2, i64 0)
  %printf_int_call172 = call i64 (ptr, ...) @printf(ptr @format_str.4, i64 %potencia_call171)
  %printf_str_call173 = call i64 (ptr, ...) @printf(ptr @format_str, ptr @string_literal.44)
  %zero174 = load i64, ptr %zero, align 4
  %absoluto_call175 = call i64 @matematica_absoluto(i64 %zero174)
  %printf_int_call176 = call i64 (ptr, ...) @printf(ptr @format_str.4, i64 %absoluto_call175)
  %absoluto_call177 = call i64 @matematica_absoluto(i64 -5)
  %potencia_call178 = call i64 @matematica_potencia(i64 2, i64 3)
  %mul179 = mul i64 %absoluto_call177, %potencia_call178
  %strlen_call180 = call i64 @strlen(ptr @string_literal.45)
  %add181 = add i64 %mul179, %strlen_call180
  %expressao_complexa = alloca i64, align 8
  store i64 %add181, ptr %expressao_complexa, align 4
  %printf_str_call182 = call i64 (ptr, ...) @printf(ptr @format_str, ptr @string_literal.46)
  %expressao_complexa183 = load i64, ptr %expressao_complexa, align 4
  %printf_int_call184 = call i64 (ptr, ...) @printf(ptr @format_str.4, i64 %expressao_complexa183)
  %printf_str_call185 = call i64 (ptr, ...) @printf(ptr @format_str, ptr @string_literal.47)
  ret i64 0
}

declare i64 @printf(ptr, ...)

declare i64 @strlen(ptr)

declare ptr @malloc(i64)

declare ptr @strcpy(ptr, ptr)
