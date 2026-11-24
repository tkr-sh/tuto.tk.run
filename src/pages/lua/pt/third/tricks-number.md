## Mais operadores

Você já conhece `+`, `-`, `*` e `/`. O Lua tem outros três operadores para `number`s:

|Operador|O que ele faz|
|-|-|
|`//`|Divide e remove os decimais|
|`%`|Dá o resto de uma divisão|
|`^`|Eleva a uma potência|

<div class="runner">

```lua
print(7 / 2)
print(7 // 2)
print(7 % 2)
print(2 ^ 10)
```

</div>

O operador `%` é muito útil para saber se um `number` é par ou ímpar: um `number` é par quando o resto da sua divisão por `2` é `0`.

<div class="runner">

```lua
print(10 % 2 == 0)
print(7 % 2 == 0)
```

</div>

## Números inteiros e números decimais

O Lua tem dois tipos de `number`s: os números inteiros como `5`, e os números decimais como `5.0` ou `3.14`. Na maioria das vezes você não precisa pensar nisso, mas isso explica por que alguns resultados são impressos com um `.0`:

<div class="runner">

```lua
print(10 / 2)
print(10 // 2)
print(5 == 5.0)
```

</div>

O operador `/` sempre dá um `number` decimal, mesmo quando o resultado é redondo. `5` e `5.0` continuam iguais.

<div class="note">

Os `number`s decimais nem sempre são exatos! Os computadores não conseguem armazenar perfeitamente um `number` como `0.1`, então algumas comparações dão resultados surpreendentes:

<div class="runner">

```lua
print(0.1 + 0.2 == 0.3)
```
</div>

Evite usar `==` no resultado de um cálculo com `number`s decimais.

</div>

## Outras formas de escrever `number`s

Os `number`s muito grandes ou muito pequenos podem ser escritos com `e`, que significa "vezes 10 elevado a". Você também pode escrever `number`s em [hexadecimal](https://pt.wikipedia.org/wiki/Sistema_hexadecimal) começando por `0x`:

<div class="runner">

```lua
print(1e3)
print(2.5e-2)
print(0xFF)
```

</div>

## Converter `number`s e strings

A função `tostring` converte um `number` em `string`, e `tonumber`, que já usamos, faz o contrário:

<div class="runner">

```lua
print(tostring(42) .. "!")
print(tonumber("42") + 1)
```

</div>

O Lua também faz a conversão sozinho quando pode: uma `string` que parece um `number` se torna um `number` num cálculo, e um `number` se torna uma `string` com `..`.

<div class="runner">

```lua
print("10" + 5)
print(10 .. 20)
```

</div>

## `math.floor` e `math.ceil`
A função `math.floor` arredonda um `number` para baixo, e `math.ceil` para cima:

<div class="runner">

```lua
print(math.floor(3.7))
print(math.ceil(3.2))
```

</div>

Para arredondar para o `number` inteiro mais próximo, some `0.5` antes de usar `math.floor`:

<div class="runner">

```lua
print(math.floor(3.7 + 0.5))
print(math.floor(3.2 + 0.5))
```

</div>

## `math.max` e `math.min`
A função `math.max` dá o maior dos `number`s que você fornece, e `math.min` o menor:

<div class="runner">

```lua
print(math.max(4, 9, 2))
print(math.min(4, 9, 2))
```

</div>

## `math.abs`
A função `math.abs` remove o sinal de um `number`. Isso é útil para obter a distância entre dois `number`s:

<div class="runner">

```lua
print(math.abs(-5))
print(math.abs(3 - 10))
```

</div>

## `math.sqrt`
A função `math.sqrt` dá a raiz quadrada de um `number`:

<div class="runner">

```lua
print(math.sqrt(16))
```

</div>

---

Você pode consultar todas as funções `math` aqui: <https://tkr-sh.github.io/lua-std-doc/lua_std/math/index.html>
