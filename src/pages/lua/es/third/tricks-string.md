## Comillas

Una cadena puede escribirse con comillas dobles `"..."` o comillas simples `'...'`. Ambas dan exactamente la misma cadena. Esto es útil cuando tu texto contiene una de las dos:

<div class="runner">

```lua
print("Es un buen día")
print('Él dijo "hola"')
```

</div>

## Cadenas de varias líneas

No puedes simplemente añadir un salto de línea en medio de una cadena `"..."` así:

<div class="runner">

```lua
print("línea 1
línea 2")
```
</div>

Lua simplemente no permite tener un salto de línea así en una cadena cuando la escribes con `"` o `'`.

¡PERO! Puedes hacerlo sustituyendo `"` o `'` por `[[`/`]]`:

<div class="runner">

```lua
print([[línea 1
línea 2]])
```

</div>

## Longitud

El operador `#` que usamos con las tablas también funciona con las cadenas:

<div class="runner">

```lua
print(#"hola")
print(#"hola mundo")
```

</div>

## `string.sub`
La función `string.sub` permite crear una nueva `string`, a partir de una parte de esta `string` indicando el índice de inicio y de fin:

<div class="runner">

```lua
print(("hola"):sub(2, 4))
-- Igual que
-- print(string.sub("hola", 2, 4))
```

</div>

## `string.rep`
La función `string.rep` permite repetir una `string` varias veces:

<div class="runner">

```lua
print(("hola"):rep(3))
```

</div>

## `string.reverse`
La función `string.reverse`, como su nombre indica, invierte una `string`:

<div class="runner">

```lua
print(("hola"):reverse())
```

</div>

## `string.gsub`
La función `string.gsub` sustituye una `string` por otra, y además muestra cuántas veces se ha sustituido:

<div class="runner">

```lua
print(("hola"):gsub("hola", "adiós"))
print(("hola"):gsub("o", "a"))
```

</div>

---

Puedes consultar todas las funciones `string` aquí: <https://tkr-sh.github.io/lua-std-doc/lua_std/struct.string.html>
