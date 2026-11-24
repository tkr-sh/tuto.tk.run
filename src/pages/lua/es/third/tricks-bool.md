## ¿Qué cuenta como `true`?

En una condición, Lua no solo acepta `true` y `false`: cualquier valor puede usarse. La regla es simple: `false` y `nil` cuentan como falso, y **todo lo demás** cuenta como verdadero.

Ten cuidado si conoces otro lenguaje de programación: ¡en Lua, `0` y la cadena vacía `""` cuentan como verdadero!

<div class="runner">

```lua
if 0 then
    print("0 cuenta como verdadero")
end

if "" then
    print("Una cadena vacía cuenta como verdadero")
end

if nil then
    print("Esto nunca se imprime")
end
```

</div>

## Comprobar `nil` sin `== nil`

Gracias a esta regla, podemos comprobar que una variable tiene un valor usándola directamente como condición:

<div class="runner">

```lua
local nombre = nil

if nombre then
    print("Hola " .. nombre)
else
    print("No hay ningún nombre")
end
```

</div>

`if nombre then` es una forma más corta de escribir `if nombre ~= nil then`, y `if not nombre then` es una forma más corta de escribir `if nombre == nil then`.

<div class="note">

No son exactamente lo mismo cuando la variable puede contener `false`: `false` no es `nil`, pero también cuenta como falso en una condición.

</div>

## `not`

La palabra clave `not` siempre da un booleano de verdad. Da `true` para `false` y `nil`, y `false` para todo lo demás:

<div class="runner">

```lua
print(not true)
print(not nil)
print(not 0)
print(not "hola")
```

</div>

## `or` y `and` devuelven uno de sus valores

Vimos `or` y `and` con booleanos, pero funcionan con cualquier valor, y no siempre dan un booleano: devuelven uno de sus dos valores.

- `a or b` da `a` si cuenta como verdadero, y `b` en caso contrario.
- `a and b` da `a` si cuenta como falso, y `b` en caso contrario.

<div class="runner">

```lua
print(nil or "predeterminado")
print("valor" or "predeterminado")

print(nil and "siguiente")
print("valor" and "siguiente")
```

</div>

Con `true` y `false`, esto da exactamente las tablas que vimos antes.

## Valores por defecto con `or`

Por eso `or` puede dar un valor por defecto a una variable, como hicimos con los argumentos opcionales:

<div class="runner">

```lua
local nombre = nil
nombre = nombre or "Invitado"

print(nombre)
```

</div>

Si `nombre` tiene un valor, se conserva. Si `nombre` es `nil`, pasa a ser `"Invitado"`.

## Elegir entre dos valores

Usando `and` y `or` juntos, podemos elegir entre dos valores según una condición, en una sola línea:

<div class="runner">

```lua
local score = 70
local resultado = score > 50 and "¡Ganas!" or "¡Pierdes!"

print(resultado)
```

</div>

Esto es lo mismo que:

```lua
local resultado
if score > 50 then
    resultado = "¡Ganas!"
else
    resultado = "¡Pierdes!"
end
```

<div class="note">

Este truco no funciona si el primer valor es `false` o `nil`: `condition and false or "otro"` siempre da `"otro"`. En ese caso, usa un `if` normal.

</div>

## `and` se detiene en cuanto puede

Cuando el primer valor de `and` cuenta como falso, Lua ni siquiera mira el segundo. ¿Recuerdas el error que tuvimos al principio del curso al comparar `nil` con un número? `and` nos permite evitarlo:

<div class="runner">

```lua
local intento_usuario = nil

if intento_usuario ~= nil and intento_usuario > 10 then
    print("El intento es mayor que 10")
end

print("¡Ningún error!")
```

</div>

Como `intento_usuario ~= nil` es `false`, `intento_usuario > 10` nunca se ejecuta. Del mismo modo, `or` no mira su segundo valor cuando el primero cuenta como verdadero.

## `nil` en las tablas

Una clave que no existe en una tabla da `nil`, sin ningún error. Y para quitar una clave de una tabla, le asignas `nil`:

<div class="runner">

```lua
local jugador = { nombre = "Bob", score = 42 }

print(jugador.edad)

jugador.score = nil
print(jugador.score)
```

</div>

<div class="note">

Esta es la forma correcta de quitar una clave como `score`, pero no un valor de una lista: como vimos, crearía un hueco. Para las listas, usa `table.remove`.

</div>
