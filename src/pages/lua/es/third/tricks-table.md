## Claves de tipo string

Cuando una clave es una `string` que parece el nombre de una variable, puedes quitar los corchetes y las comillas:

<div class="runner">

```lua
local jugador = { ["nombre"] = "Bob", ["score"] = 42 }
local mismo_jugador = { nombre = "Bob", score = 42 }

print(jugador["nombre"])
print(mismo_jugador["nombre"])
```

</div>

## Acceder a una clave con `.`

El mismo atajo existe para leer o cambiar un valor: `jugador.nombre` es igual que `jugador["nombre"]`.

<div class="runner">

```lua
local jugador = { nombre = "Bob", score = 42 }

print(jugador["nombre"])
print(jugador.nombre)

jugador.score = 50
print(jugador.score)
```

</div>

Llevas usando esto desde el primer capítulo: en `io.read()`, `io` es una `table` integrada de Lua ¡y `read` es una de sus claves!

<div class="note">

`jugador.clave` y `jugador[clave]` no son lo mismo. La primera busca la clave `"clave"`, la segunda usa el valor de la variable `clave`:

<div class="runner">

```lua
local jugador = { nombre = "Bob", score = 42 }
local clave = "score"

print(jugador.clave) -- nil
print(jugador[clave]) -- 42
```
</div>

</div>

## `table.concat`
La función `table.concat` une todos los valores de una lista en una sola `string`. Puedes darle un separador que se coloque entre los valores. Esto es muy útil para imprimir una lista, porque `print` por sí solo no muestra lo que hay dentro de una tabla:

<div class="runner">

```lua
local letras = { "a", "b", "c" }

print(letras)
print(table.concat(letras))
print(table.concat(letras, ", "))
```

</div>

## `table.insert`
La función `table.insert` añade un valor al final de una lista. Si le das una posición, el valor se inserta ahí y los valores siguientes se desplazan una posición:

<div class="runner">

```lua
local nombres = { "bob", "carol" }

table.insert(nombres, "dave")
print(table.concat(nombres, ", "))

table.insert(nombres, 2, "alice")
print(table.concat(nombres, ", "))
```

</div>

## `table.remove`
La función `table.remove` quita el valor de una posición y desplaza los valores siguientes, para que no haya un hueco en la lista. Sin una posición, quita el último valor. También devuelve el valor que se quitó:

<div class="runner">

```lua
local nombres = { "bob", "alice", "carol" }

print(table.remove(nombres, 1))
print(table.concat(nombres, ", "))

print(table.remove(nombres))
print(table.concat(nombres, ", "))
```

</div>

## `table.sort`
La función `table.sort` ordena una lista, del valor más pequeño al más grande, o en orden alfabético para las `string`s. Modifica la propia lista y no crea una nueva:

<div class="runner">

```lua
local numeros = { 42, 7, 19 }

table.sort(numeros)
print(table.concat(numeros, ", "))
```

</div>

Para ordenar en otro orden, puedes darle una función. Esta función recibe dos valores y debe devolver `true` si el primero debe colocarse antes que el segundo:

<div class="runner">

```lua
local numeros = { 42, 7, 19 }

table.sort(numeros, function(a, b) return a > b end)
print(table.concat(numeros, ", "))
```

</div>

---

Puedes consultar todas las funciones `table` aquí: <https://tkr-sh.github.io/lua-std-doc/lua_std/struct.table.html>
