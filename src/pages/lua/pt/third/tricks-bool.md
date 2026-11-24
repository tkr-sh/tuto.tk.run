## O que conta como `true`?

Numa condição, o Lua não aceita apenas `true` e `false`: qualquer valor pode ser usado. A regra é simples: `false` e `nil` contam como falso, e **todo o resto** conta como verdadeiro.

Tenha cuidado se você conhece outra linguagem de programação: no Lua, `0` e a string vazia `""` contam como verdadeiro!

<div class="runner">

```lua
if 0 then
    print("0 conta como verdadeiro")
end

if "" then
    print("Uma string vazia conta como verdadeiro")
end

if nil then
    print("Isto nunca é exibido")
end
```

</div>

## Verificar `nil` sem `== nil`

Graças a esta regra, podemos verificar que uma variável tem um valor usando-a diretamente como condição:

<div class="runner">

```lua
local nome = nil

if nome then
    print("Olá " .. nome)
else
    print("Não há nenhum nome")
end
```

</div>

`if nome then` é uma forma mais curta de escrever `if nome ~= nil then`, e `if not nome then` é uma forma mais curta de escrever `if nome == nil then`.

<div class="note">

Não são exatamente a mesma coisa quando a variável pode conter `false`: `false` não é `nil`, mas também conta como falso numa condição.

</div>

## `not`

A palavra-chave `not` sempre dá um booleano de verdade. Ela dá `true` para `false` e `nil`, e `false` para todo o resto:

<div class="runner">

```lua
print(not true)
print(not nil)
print(not 0)
print(not "olá")
```

</div>

## `or` e `and` retornam um dos seus valores

Vimos `or` e `and` com booleanos, mas eles funcionam com qualquer valor, e não dão sempre um booleano: eles retornam um dos seus dois valores.

- `a or b` dá `a` se ele conta como verdadeiro, e `b` caso contrário.
- `a and b` dá `a` se ele conta como falso, e `b` caso contrário.

<div class="runner">

```lua
print(nil or "padrão")
print("valor" or "padrão")

print(nil and "próximo")
print("valor" and "próximo")
```

</div>

Com `true` e `false`, isso dá exatamente as tabelas que vimos antes.

## Valores padrão com `or`

É por isso que `or` pode dar um valor padrão a uma variável, como fizemos com os argumentos opcionais:

<div class="runner">

```lua
local nome = nil
nome = nome or "Convidado"

print(nome)
```

</div>

Se `nome` tem um valor, ele é mantido. Se `nome` é `nil`, ele passa a ser `"Convidado"`.

## Escolher entre dois valores

Usando `and` e `or` juntos, podemos escolher entre dois valores com uma condição, numa única linha:

<div class="runner">

```lua
local score = 70
local resultado = score > 50 and "Você ganha!" or "Você perde!"

print(resultado)
```

</div>

Isto é o mesmo que:

```lua
local resultado
if score > 50 then
    resultado = "Você ganha!"
else
    resultado = "Você perde!"
end
```

<div class="note">

Este truque não funciona se o primeiro valor for `false` ou `nil`: `condition and false or "outro"` dá sempre `"outro"`. Nesse caso, use um `if` normal.

</div>

## `and` para assim que pode

Quando o primeiro valor de `and` conta como falso, o Lua nem olha para o segundo. Lembra do erro que tivemos no início do curso ao comparar `nil` com um número? `and` nos permite evitá-lo:

<div class="runner">

```lua
local chute_do_usuario = nil

if chute_do_usuario ~= nil and chute_do_usuario > 10 then
    print("O chute é maior que 10")
end

print("Nenhum erro!")
```

</div>

Como `chute_do_usuario ~= nil` é `false`, `chute_do_usuario > 10` nunca é executado. Da mesma forma, `or` não olha para o seu segundo valor quando o primeiro conta como verdadeiro.

## `nil` nas tabelas

Uma chave que não existe numa tabela dá `nil`, sem nenhum erro. E para remover uma chave de uma tabela, você atribui `nil` a ela:

<div class="runner">

```lua
local jogador = { nome = "Bob", score = 42 }

print(jogador.idade)

jogador.score = nil
print(jogador.score)
```

</div>

<div class="note">

Esta é a forma correta de remover uma chave como `score`, mas não um valor de uma lista: como vimos, isso criaria um buraco. Para listas, use `table.remove`.

</div>
