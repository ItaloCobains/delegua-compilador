; ModuleID = 'program'
source_filename = "program"

@format_str = global [6 x i8] c"%lld\0A\00"

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
  %absoluto_call = call i64 @matematica_absoluto(i64 -42)
  %x = alloca i64, align 8
  store i64 %absoluto_call, ptr %x, align 4
  %x1 = load i64, ptr %x, align 4
  %printf_int_call = call i64 (ptr, ...) @printf(ptr @format_str, i64 %x1)
  ret i64 0
}

declare i64 @printf(ptr, ...)
