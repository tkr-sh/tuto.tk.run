Pela primeira vez, não vamos nos concentrar em uma única linha! A linha atual é apenas

```lua
repeat
```

mas, por si só, ela não faz nada: ela anda junto com `until`.

```lua
repeat
	chute_do_usuario = tonumber(read())
	numero_de_tentativas = numero_de_tentativas + 1

	if numero_para_adivinhar < chute_do_usuario then
		print("O número é mais baixo!")
	end

	if numero_para_adivinhar > chute_do_usuario then
		print("O número é mais alto!")
	end
until chute_do_usuario == numero_para_adivinhar
```

Mas antes de tudo... o que são `repeat` e `until` e por que são tão especiais? Não são apenas variáveis estranhas?

Você talvez se lembre da definição de uma variável

> `o_nome_da_variavel` pode ser qualquer coisa. Exceto `true`, `false`, `nil` (e outros que vamos ver mais tarde)

E esse «mais tarde»... é agora!

## Palavras-chave

`repeat` e `until` são o que chamamos de «palavras-chave». Palavras-chave são nomes especiais que, em vez de ter um valor (como as variáveis) ou fazer alguma computação (como as funções), são instruções que dizem ao Lua para tomar certas decisões.

No nosso exemplo de `repeat`/`until`, tudo o que está entre `repeat` e `until` será repetido até que uma condição seja satisfeita. Quando a execução chega a `until`, ela verifica se deve executar de novo e, se sim, o código volta para `repeat` e se repete.

Esse tipo de comportamento - repetir algum código até que _algo_ aconteça - é chamado de «loop» e é um conceito muito comum na programação.

Na maioria das vezes, você quer sair do seu loop em um certo ponto...

### Mas como saber quando sair?

Quando você usa loops como `until`, você deve fornecer um `boolean` (`true` ou `false`) para ele, para dizer quando parar de repetir.

E na maioria das vezes, você faz isso com operadores relacionais:

- `==`: Verifica se dois valores são iguais. **NÃO CONFUNDIR COM 1 `=`**. 1 `=` serve para atribuir um valor, 2 `=` (`==`) servem para verificar se dois valores são iguais. Se eles forem iguais, o valor é `true`; caso contrário, é `false`
- `~=`: Verifica se dois valores são diferentes. Se eles forem diferentes, o valor é `true`; caso contrário, é `false`
- `>`: Verifica se o valor da esquerda é maior que o valor da direita. Se for, o valor é `true`; caso contrário, é `false`
- `<`: Verifica se o valor da esquerda é menor que o valor da direita. Se for, o valor é `true`; caso contrário, é `false`
- `>=`: Verifica se o valor da esquerda é maior **OU IGUAL** ao valor da direita. Se for, o valor é `true`; caso contrário, é `false`
- `<=`: Verifica se o valor da esquerda é menor **OU IGUAL** ao valor da direita. Se for, o valor é `true`; caso contrário, é `false`

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

- `repeat`/`until`: vai repetir algum código até que uma condição seja satisfeita, e parar assim que essa condição for satisfeita.

<div class="runner">

```lua
idade = 0
repeat
    print(idade)
    idade = idade + 1
until idade == 4
```
</div>

- `while`: os loops `while` são muito semelhantes aos loops `repeat`/`until`, mas ao contrário! Você repete enquanto a condição for satisfeita! Depois da condição, você usa a palavra-chave `do` para começar a executar o código, e volta ao início quando encontra a palavra-chave `end`. Na maioria das situações em que você pode usar `repeat`, também pode usar `while`. É principalmente uma questão de gosto pessoal.

<div class="runner">

```lua
idade = 0
while idade ~= 4 do
    print(idade)
    idade = idade + 1
end
```
</div>

- `for`: o último loop é o loop `for`, e ele é um pouco diferente, porque você já sabe quantas vezes quer iterar. E você define de qual número até qual número quer iterar. No final de cada iteração, ele incrementará automaticamente `idade` em `1`!

<div class="runner">

```lua
for idade = 0, 3 do
    print(idade)
end
```
</div>


## Voltando ao nosso caso

No nosso caso, queremos continuar perguntando ao usuário um número até ele encontrar o correto. Portanto, usamos:
```lua
repeat
    ...
until chute_do_usuario == numero_para_adivinhar
```
