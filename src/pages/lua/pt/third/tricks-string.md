## Aspas

Uma string pode ser escrita com aspas duplas `"..."` ou aspas simples `'...'`. Ambas dão exatamente a mesma string. Isso é útil quando o seu texto contém uma das duas:

<div class="runner">

```lua
print("Está um belo dia")
print('Ele disse "olá"')
```

</div>

## Strings de várias linhas

Você não pode simplesmente adicionar uma quebra de linha no meio de uma string `"..."` assim:

<div class="runner">

```lua
print("linha 1
linha 2")
```
</div>

O Lua simplesmente não permite ter uma quebra de linha assim numa string quando você a escreve com `"` ou `'`.

MAS! Você pode fazer isso substituindo `"` ou `'` por `[[`/`]]`:

<div class="runner">

```lua
print([[linha 1
linha 2]])
```

</div>

## Comprimento

O operador `#` que usamos em tabelas também funciona em strings:

<div class="runner">

```lua
print(#"olá")
print(#"olá mundo")
```

</div>

## `string.sub`
A função `string.sub` permite criar uma nova `string`, a partir de uma parte desta `string`, especificando o índice de início e de fim:

<div class="runner">

```lua
print(("olá"):sub(2, 4))
-- O mesmo que
-- print(string.sub("olá", 2, 4))
```

</div>

## `string.rep`
A função `string.rep` permite repetir uma `string` várias vezes:

<div class="runner">

```lua
print(("olá"):rep(3))
```

</div>

## `string.reverse`
A função `string.reverse`, como o nome indica, inverte uma `string`:

<div class="runner">

```lua
print(("olá"):reverse())
```

</div>

## `string.gsub`
A função `string.gsub` substitui uma `string` por outra, e também mostra quantas vezes ela foi substituída:

<div class="runner">

```lua
print(("olá"):gsub("olá", "tchau"))
print(("olá"):gsub("á", "a"))
```

</div>

---

Você pode consultar todas as funções `string` aqui: <https://tkr-sh.github.io/lua-std-doc/lua_std/struct.string.html>
