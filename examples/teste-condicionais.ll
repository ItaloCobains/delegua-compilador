; ModuleID = 'program'
source_filename = "program"

@string_literal = global [23 x i8] c"N\C3\BAmero \C3\A9 maior que 5\00"
@format_str = global [4 x i8] c"%s\0A\00"
@string_literal.1 = global [23 x i8] c"N\C3\BAmero \C3\A9 menor que 5\00"
@string_literal.2 = global [28 x i8] c"N\C3\BAmero n\C3\A3o \C3\A9 menor que 5\00"
@string_literal.3 = global [13 x i8] c"N\C3\BAmero \C3\A9 1\00"
@string_literal.4 = global [14 x i8] c"N\C3\BAmero \C3\A9 10\00"
@string_literal.5 = global [25 x i8] c"N\C3\BAmero n\C3\A3o \C3\A9 1 nem 10\00"
@string_literal.6 = global [31 x i8] c"N\C3\BAmero \C3\A9 maior ou igual a 10\00"
@string_literal.7 = global [31 x i8] c"N\C3\BAmero \C3\A9 menor ou igual a 15\00"
@string_literal.8 = global [26 x i8] c"N\C3\BAmero \C3\A9 diferente de 5\00"

define i64 @main() {
entry:
  %numero = alloca i64, align 8
  store i64 10, ptr %numero, align 4
  %numero1 = load i64, ptr %numero, align 4
  %gt = icmp sgt i64 %numero1, 5
  br i1 %gt, label %then, label %merge

then:                                             ; preds = %entry
  %printf_str_call = call i64 (ptr, ...) @printf(ptr @format_str, ptr @string_literal)
  br label %merge

merge:                                            ; preds = %then, %entry
  %numero2 = load i64, ptr %numero, align 4
  %lt = icmp slt i64 %numero2, 5
  br i1 %lt, label %then3, label %else

then3:                                            ; preds = %merge
  %printf_str_call5 = call i64 (ptr, ...) @printf(ptr @format_str, ptr @string_literal.1)
  br label %merge4

merge4:                                           ; preds = %else, %then3
  %numero7 = load i64, ptr %numero, align 4
  %eq = icmp eq i64 %numero7, 1
  br i1 %eq, label %then8, label %else_if_start

else:                                             ; preds = %merge
  %printf_str_call6 = call i64 (ptr, ...) @printf(ptr @format_str, ptr @string_literal.2)
  br label %merge4

then8:                                            ; preds = %merge4
  %printf_str_call10 = call i64 (ptr, ...) @printf(ptr @format_str, ptr @string_literal.3)
  br label %merge9

else_if_start:                                    ; preds = %merge4
  %numero11 = load i64, ptr %numero, align 4
  %eq12 = icmp eq i64 %numero11, 10
  br i1 %eq12, label %else_if_then_0, label %final_else

merge9:                                           ; preds = %final_else, %else_if_then_0, %then8
  %numero15 = load i64, ptr %numero, align 4
  %ge = icmp sge i64 %numero15, 10
  br i1 %ge, label %then16, label %merge17

else_if_then_0:                                   ; preds = %else_if_start
  %printf_str_call13 = call i64 (ptr, ...) @printf(ptr @format_str, ptr @string_literal.4)
  br label %merge9

final_else:                                       ; preds = %else_if_start
  %printf_str_call14 = call i64 (ptr, ...) @printf(ptr @format_str, ptr @string_literal.5)
  br label %merge9

then16:                                           ; preds = %merge9
  %printf_str_call18 = call i64 (ptr, ...) @printf(ptr @format_str, ptr @string_literal.6)
  br label %merge17

merge17:                                          ; preds = %then16, %merge9
  %numero19 = load i64, ptr %numero, align 4
  %le = icmp sle i64 %numero19, 15
  br i1 %le, label %then20, label %merge21

then20:                                           ; preds = %merge17
  %printf_str_call22 = call i64 (ptr, ...) @printf(ptr @format_str, ptr @string_literal.7)
  br label %merge21

merge21:                                          ; preds = %then20, %merge17
  %numero23 = load i64, ptr %numero, align 4
  %ne = icmp ne i64 %numero23, 5
  br i1 %ne, label %then24, label %merge25

then24:                                           ; preds = %merge21
  %printf_str_call26 = call i64 (ptr, ...) @printf(ptr @format_str, ptr @string_literal.8)
  br label %merge25

merge25:                                          ; preds = %then24, %merge21
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
