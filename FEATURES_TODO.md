# Recursos Faltantes para Compatibilidade com Delegua

Este documento lista todos os recursos que precisam ser implementados no compilador DC para alcançar compatibilidade completa com a linguagem Delegua.

## Status Atual

### ✅ Recursos Já Implementados
- [x] Variáveis (`var x = 10;`)
- [x] Operadores aritméticos básicos (`+`, `-`, `*`, `/`, `%`, `**`)
- [x] Operadores de incremento/decremento (`++`, `--`)
- [x] Strings e concatenação (`"texto" + "outro"`)
- [x] Função `escreva()` para output
- [x] Função `texto()` para conversão de tipos
- [x] Condicionais (`se`, `senao`, `senao se`)
- [x] Estrutura de escolha (`escolha`, `caso`, `padrao`)
- [x] Loops (`enquanto`, `para`, `para cada`)
- [x] Funções (`funcao`, `retorna`)
- [x] Comentários (`//`)
- [x] Booleanos (`verdadeiro`, `falso`)
- [x] Comparações (`==`, `!=`, `<`, `>`, `<=`, `>=`)
- [x] Controle de fluxo (`sustar`, `continua`)

---

## 🚨 Recursos Críticos Faltando

### 1. Operadores Lógicos (Prioridade: ALTA)
**Status:** ❌ Não implementado
**Complexidade:** Baixa

#### Recursos necessários:
- [ ] `e` - Operador AND lógico
- [ ] `ou` - Operador OR lógico
- [ ] `não` - Operador NOT lógico

#### Exemplo de uso:
```delegua
se (x > 0 e y < 10) {
    escreva("Condição atendida");
}

se (não ativo ou idade < 18) {
    escreva("Acesso negado");
}
```

#### Implementação necessária:
- Adicionar tokens no `token.rs`
- Implementar parsing no `parser.rs`
- Gerar código LLVM no `codegen.rs`

---

### 2. Função de Input (Prioridade: ALTA)
**Status:** ❌ Não implementado
**Complexidade:** Média

#### Recursos necessários:
- [ ] `leia()` - Entrada de dados do usuário
- [ ] `leia_numero()` - Entrada de números
- [ ] `leia_texto()` - Entrada de texto

#### Exemplo de uso:
```delegua
var nome = leia("Digite seu nome: ");
var idade = leia_numero("Digite sua idade: ");
escreva("Olá " + nome + ", você tem " + texto(idade) + " anos");
```

#### Implementação necessária:
- Integrar com biblioteca de input (readline/stdin)
- Tratamento de tipos de entrada
- Validação e conversão automática

---

### 3. Estruturas de Dados (Prioridade: ALTA)
**Status:** ❌ Não implementado
**Complexidade:** Alta

#### 3.1 Arrays/Listas
- [ ] Declaração de arrays: `var lista = [1, 2, 3];`
- [ ] Acesso por índice: `lista[0]`
- [ ] Modificação: `lista[1] = 5;`
- [ ] Métodos básicos: `tamanho()`, `adicionar()`, `remover()`

#### 3.2 Dicionários/Objetos
- [ ] Declaração: `var obj = {nome: "João", idade: 25};`
- [ ] Acesso por chave: `obj["nome"]` ou `obj.nome`
- [ ] Modificação: `obj.idade = 26;`

#### Exemplo de uso:
```delegua
var numeros = [1, 2, 3, 4, 5];
var pessoa = {
    nome: "Maria",
    idade: 30,
    ativo: verdadeiro
};

escreva("Primeiro número: " + texto(numeros[0]));
escreva("Nome: " + pessoa.nome);
```

---

## 📚 Recursos de Biblioteca (Prioridade: MÉDIA)

### 4. Biblioteca Matemática Expandida
**Status:** 🟡 Parcialmente implementado
**Complexidade:** Baixa-Média

#### Recursos faltando:
- [ ] `abs()` - Valor absoluto
- [ ] `sqrt()` - Raiz quadrada
- [ ] `pow()` - Potência
- [ ] `min()`, `max()` - Menor/maior valor
- [ ] `aleatorio()` - Números aleatórios
- [ ] `pi`, `e` - Constantes matemáticas
- [ ] Funções trigonométricas (`sin`, `cos`, `tan`)

### 5. Funções de String
**Status:** ❌ Não implementado
**Complexidade:** Média

#### Recursos necessários:
- [ ] `comprimento()` - Tamanho da string
- [ ] `maiuscula()` - Converter para maiúsculas
- [ ] `minuscula()` - Converter para minúsculas
- [ ] `substring()` - Extrair parte da string
- [ ] `substituir()` - Substituir texto
- [ ] `dividir()` - Dividir string em array
- [ ] `contem()` - Verificar se contém substring

#### Exemplo de uso:
```delegua
var texto = "Olá Mundo";
escreva("Tamanho: " + texto(comprimento(texto)));
escreva("Maiúscula: " + maiuscula(texto));
```

### 6. Funções de Conversão
**Status:** 🟡 Parcialmente implementado
**Complexidade:** Baixa

#### Recursos faltando:
- [ ] `numero()` - Converter para número
- [ ] `booleano()` - Converter para booleano
- [ ] `tipo_de()` - Retornar tipo da variável

---

## 🛡️ Tratamento de Erros (Prioridade: MÉDIA)

### 7. Sistema de Exceções
**Status:** ❌ Não implementado
**Complexidade:** Alta

#### Recursos necessários:
- [ ] `tente` - Bloco try
- [ ] `pegue` - Bloco catch
- [ ] `finalmente` - Bloco finally
- [ ] `lance` - Lançar exceção personalizada

#### Exemplo de uso:
```delegua
tente {
    var resultado = 10 / 0;
} pegue (erro) {
    escreva("Erro: " + erro.mensagem);
} finalmente {
    escreva("Operação concluída");
}
```

---

## 🎯 Programação Orientada a Objetos (Prioridade: BAIXA)

### 8. Classes e Objetos
**Status:** ❌ Não implementado
**Complexidade:** Muito Alta

#### Recursos necessários:
- [ ] `classe` - Definição de classes
- [ ] `construtor` - Método construtor
- [ ] `herda` - Herança de classes
- [ ] `publico`, `privado` - Modificadores de acesso
- [ ] `estatico` - Membros estáticos
- [ ] `metodo` - Definição de métodos

#### Exemplo de uso:
```delegua
classe Pessoa {
    construtor(nome, idade) {
        este.nome = nome;
        este.idade = idade;
    }

    metodo cumprimentar() {
        escreva("Olá, eu sou " + este.nome);
    }
}

classe Estudante herda Pessoa {
    construtor(nome, idade, curso) {
        super(nome, idade);
        este.curso = curso;
    }
}
```

---

## 🔧 Funcionalidades Avançadas (Prioridade: BAIXA)

### 9. Closures e Lambdas
**Status:** ❌ Não implementado
**Complexidade:** Muito Alta

#### Recursos necessários:
- [ ] Funções anônimas
- [ ] Closures com captura de variáveis
- [ ] Funções de alta ordem

### 10. Decoradores
**Status:** ❌ Não implementado
**Complexidade:** Muito Alta

### 11. Inferência de Tipos
**Status:** ❌ Não implementado
**Complexidade:** Muito Alta

---

## 📊 Sistema de Importação Melhorado

### 12. Módulos e Namespaces
**Status:** 🟡 Parcialmente implementado
**Complexidade:** Alta

#### Recursos faltando:
- [ ] Importação seletiva: `de modulo importe funcao1, funcao2`
- [ ] Aliases: `importe modulo como alias`
- [ ] Exportação explícita
- [ ] Namespaces aninhados

---

## 🗂️ Priorização de Implementação

### Fase 1 - Fundamentos (1-2 semanas)
1. **Operadores lógicos** (`e`, `ou`, `não`)
2. **Função `leia()`** para input
3. **Arrays básicos** com acesso por índice

### Fase 2 - Estruturas (2-3 semanas)
1. **Dicionários/Objetos** básicos
2. **Biblioteca de strings** expandida
3. **Funções matemáticas** adicionais

### Fase 3 - Robustez (3-4 semanas)
1. **Tratamento de erros** (`tente`/`pegue`)
2. **Sistema de importação** melhorado
3. **Validação de tipos** aprimorada

### Fase 4 - Avançado (Opcional)
1. **Classes e herança**
2. **Closures e lambdas**
3. **Decoradores**

---

## 📝 Notas de Implementação

### Considerações Técnicas:
- **LLVM IR**: Muitos recursos requerem expansão da geração de código
- **Biblioteca padrão**: Necessário expandir módulos em `src/modules/`
- **Parser**: Precedência de operadores precisa ser ajustada
- **Testes**: Cada novo recurso deve ter testes unitários e de integração

### Compatibilidade:
- Manter compatibilidade com código DC existente
- Seguir convenções de nomenclatura portuguesa do Delegua
- Implementar dialectos quando aplicável

### Performance:
- Usar zero-copy strings quando possível
- Memory pooling para estruturas frequentes
- Otimizações LLVM para geração de código eficiente

---

## 🎯 Meta Final

Alcançar **100% de compatibilidade** com a especificação Delegua, permitindo:
- Executar programas Delegua existentes
- Tradução bidirecional entre DC e Delegua
- Suporte completo ao ecosystem Delegua
- Integração com bibliotecas NPM (quando aplicável)

**Estimativa total:** 10-16 semanas de desenvolvimento (dependendo da complexidade das fases 3-4)