Pela primeira vez, não vamos nos concentrar em uma única linha! A linha atual é apenas

```lua
repeat
```

mas em si mesma, ela não faz nada: ela vai com `until`.

```lua
repeat
    ...
until chute_do_usuario == number_to_guess
```

Mas primeiro de tudo... o que são `repeat` e `until` e por que são tão especiais? Não são apenas variáveis estranhas?

Você pode se lembrar da definição de uma variável

> `nome_da_variavel` pode ser qualquer coisa. Exceto `true`, `false`, `nil` (e outros que vamos ver mais tarde)

E o mais tarde... é agora!

## Palavras-chave

`repeat` e `until` são o que chamamos de "palavras-chave". Palavras-chave são nomes especiais que, em vez de ter um valor (como variáveis) ou fazer alguma computação (como funções), são instruções que dizem ao Lua para tomar certas decisões.

No nosso exemplo de `repeat`/`until`, tudo o que está entre `repeat` e `until` será repetido até que uma condição seja satisfeita. Quando a execução chega a `until`, ela verifica se quer reexecutar, e se quiser, o código volta para `repeat` e repete.

Esse tipo de comportamento - repetir algum código até _alguma coisa_ - é chamado de "loop" e é um conceito muito comum na programação.

Na maioria das vezes, você quer sair do seu loop em um certo ponto...

### Mas como saber quando sair?

Quando usar loops como `until`, você deve fornecer um valor booleano (`true` ou `false`) para ele, para dizer se você quer continuar a iterar.

E na maioria das vezes, você faz isso com operadores relacionais:

- `==`: Verifica se dois valores são iguais. **NÃO É O MESMO QUE 1 `=`**. 1 `` é usado para atribuir um valor, 2 `=` (`==`) é usado para verificar se dois valores são iguais. Se eles forem iguais, o valor é `true`, 
caso contrário é `false`
- `~=`: Verifica se dois valores são diferentes. Se eles forem diferentes, o valor é `true`, caso contrário é `false`
- `>`: Verifica se o valor da esquerda é maior que o valor da direita. Se ele for, o valor é `true`, caso contrário é `false`
- `<`: Verifica se o valor da esquerda é menor que o valor da direita. Se ele for, o valor é `true`, caso contrário é `false`
- `>=`: Verifica se o valor da esquerda é maior ou igual ao valor da direita. Se ele for, o valor é `true`, caso contrário é `false`
- `<=`: Verifica se o valor da esquerda é menor ou igual ao valor da direita. Se ele for, o valor é `true`, caso contrário é `false`

<div class="runner">

```lua
print(1984 == 2025)
print(1984 ~= 2025)
print(1984 < 2025)
print(2025 <= 2025)
print(2025 < 2025)
print(1984 > 2025)
```
</div>

### Tipos de loops

- `repeat`/`until`: Vai repetir algum código até que uma condição seja satisfeita, e parar assim que essa condição for satisfeita.

<div class="runner">

```lua
idade = 0
repeat
    print(idade)
    idade = idade + 1
until idade == 4
```
</div>

- `while`: Os loops `while` são muito semelhantes aos loops `repeat`/`until`, mas funcionam de forma contrária. Você faz um loop enquanto a condição for satisfeita! Depois da condição, você usa a palavra-chave `do` para começar a 
executar o código e volta ao início quando vê a palavra-chave `end`. Na maioria das situações em que pode usar `repeat`, também pode usar `while`. É principalmente uma questão de preferência pessoal.

<div class="runner">

```lua
idade = 0
while idade ~= 4 do
    print(idade)
    idade = idade + 1
end
```
</div>

- `for`: O último loop é o loop `for` e é um pouco diferente, pois você já sabe quantas vezes deseja iterar. E define a partir de qual número até qual número deseja iterar. No final de uma iteração, ele incrementará automaticamente a 
`idade` em `1`.

<div class="runner">

```lua
for idade = 0, 3 do
    print(idade)
end
```
</div>


## Voltando ao nosso caso

No nosso caso, queremos continuar perguntando ao usuário por um número até encontrar o correto. Portanto, usamos:
```lua
repeat
    ...
until chute_do_usuario == number_to_guess
```
