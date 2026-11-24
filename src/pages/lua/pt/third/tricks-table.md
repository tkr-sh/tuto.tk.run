## Chaves de tipo string

Quando uma chave é uma `string` que parece o nome de uma variável, você pode remover os colchetes e as aspas:

<div class="runner">

```lua
local jogador = { ["nome"] = "Bob", ["score"] = 42 }
local mesmo_jogador = { nome = "Bob", score = 42 }

print(jogador["nome"])
print(mesmo_jogador["nome"])
```

</div>

## Acessar uma chave com `.`

O mesmo atalho existe para ler ou alterar um valor: `jogador.nome` é o mesmo que `jogador["nome"]`.

<div class="runner">

```lua
local jogador = { nome = "Bob", score = 42 }

print(jogador["nome"])
print(jogador.nome)

jogador.score = 50
print(jogador.score)
```

</div>

Você vem usando isso desde o primeiro capítulo: em `io.read()`, `io` é uma `table` embutida do Lua e `read` é uma de suas chaves!

<div class="note">

`jogador.chave` e `jogador[chave]` não são a mesma coisa! A primeira procura a chave `"chave"`, a segunda usa o valor da variável `chave`:

<div class="runner">

```lua
local jogador = { nome = "Bob", score = 42 }
local chave = "score"

print(jogador.chave) -- nil
print(jogador[chave]) -- 42
```
</div>

</div>

## `table.concat`
A função `table.concat` junta todos os valores de uma lista numa única `string`. Você pode dar a ela um separador para colocar entre os valores. Isso é muito útil para imprimir uma lista, porque `print` sozinho não mostra o que há dentro de uma tabela:

<div class="runner">

```lua
local letras = { "a", "b", "c" }

print(letras)
print(table.concat(letras))
print(table.concat(letras, ", "))
```

</div>

## `table.insert`
A função `table.insert` adiciona um valor ao final de uma lista. Se você der uma posição, o valor é inserido ali e os valores seguintes são movidos uma posição:

<div class="runner">

```lua
local nomes = { "bob", "carol" }

table.insert(nomes, "dave")
print(table.concat(nomes, ", "))

table.insert(nomes, 2, "alice")
print(table.concat(nomes, ", "))
```

</div>

## `table.remove`
A função `table.remove` remove o valor de uma posição e move os valores seguintes, para que não haja um buraco na lista. Sem uma posição, ela remove o último valor. Ela também devolve o valor que foi removido:

<div class="runner">

```lua
local nomes = { "bob", "alice", "carol" }

print(table.remove(nomes, 1))
print(table.concat(nomes, ", "))

print(table.remove(nomes))
print(table.concat(nomes, ", "))
```

</div>

## `table.sort`
A função `table.sort` ordena uma lista, do menor para o maior valor, ou em ordem alfabética para `string`s. Ela altera a própria lista e não cria uma nova:

<div class="runner">

```lua
local numeros = { 42, 7, 19 }

table.sort(numeros)
print(table.concat(numeros, ", "))
```

</div>

Para ordenar em outra ordem, você pode dar a ela uma função. Essa função recebe dois valores e deve retornar `true` se o primeiro deve ser colocado antes do segundo:

<div class="runner">

```lua
local numeros = { 42, 7, 19 }

table.sort(numeros, function(a, b) return a > b end)
print(table.concat(numeros, ", "))
```

</div>

---

Você pode consultar todas as funções `table` aqui: <https://tkr-sh.github.io/lua-std-doc/lua_std/struct.table.html>
