def eh_primo?(n)
  return false if n < 2
  return true if n == 2
  return false if n.even?

  limite = n / 2
  (3..limite).each do |i|
    return false if n % i == 0
  end

  true
end

def contar_primos(limite)
  contador = 0
  (2..limite).each do |numero|
    contador += 1 if eh_primo?(numero)
  end
  contador
end

limite = 1_000_000
resultado = contar_primos(limite)
puts "RESULT:#{resultado}"
