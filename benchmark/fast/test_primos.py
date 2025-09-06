def eh_primo(n):
    if n < 2:
        return False
    if n == 2:
        return True
    if n % 2 == 0:
        return False
    limite = n // 2
    for i in range(3, limite + 1):
        if n % i == 0:
            return False
    return True

def contar_primos(limite):
    contador = 0
    for numero in range(2, limite + 1):
        if eh_primo(numero):
            contador += 1
    return contador

limite = 1_000_000
resultado = contar_primos(limite)
print("RESULT:" + str(resultado))
