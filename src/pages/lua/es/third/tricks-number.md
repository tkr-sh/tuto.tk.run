## Más operadores

Ya conoces `+`, `-`, `*` y `/`. Lua tiene otros tres operadores para `number`s:

|Operador|Qué hace|
|-|-|
|`//`|Divide y elimina los decimales|
|`%`|Da el resto de una división|
|`^`|Eleva a una potencia|

<div class="runner">

```lua
print(7 / 2)
print(7 // 2)
print(7 % 2)
print(2 ^ 10)
```

</div>

El operador `%` es muy útil para saber si un `number` es par o impar: un `number` es par cuando el resto de su división entre `2` es `0`.

<div class="runner">

```lua
print(10 % 2 == 0)
print(7 % 2 == 0)
```

</div>

## Números enteros y números decimales

Lua tiene dos tipos de `number`s: los números enteros como `5`, y los números decimales como `5.0` o `3.14`. La mayoría de las veces no necesitas pensar en ello, pero explica por qué algunos resultados se imprimen con un `.0`:

<div class="runner">

```lua
print(10 / 2)
print(10 // 2)
print(5 == 5.0)
```

</div>

El operador `/` siempre da un `number` decimal, incluso cuando el resultado es redondo. `5` y `5.0` siguen siendo iguales.

<div class="note">

¡Los `number`s decimales no siempre son exactos! Las computadoras no pueden almacenar perfectamente un `number` como `0.1`, así que algunas comparaciones dan resultados sorprendentes:

<div class="runner">

```lua
print(0.1 + 0.2 == 0.3)
```
</div>

Evita usar `==` sobre el resultado de un cálculo con `number`s decimales.

</div>

## Otras formas de escribir `number`s

Los `number`s muy grandes o muy pequeños se pueden escribir con `e`, que significa "por 10 elevado a". También puedes escribir `number`s en [hexadecimal](https://es.wikipedia.org/wiki/Sistema_hexadecimal) empezando por `0x`:

<div class="runner">

```lua
print(1e3)
print(2.5e-2)
print(0xFF)
```

</div>

## Convertir `number`s y cadenas

La función `tostring` convierte un `number` en `string`, y `tonumber`, que ya usamos, hace lo contrario:

<div class="runner">

```lua
print(tostring(42) .. "!")
print(tonumber("42") + 1)
```

</div>

Lua también hace la conversión por sí solo cuando puede: un `string` que parece un `number` se convierte en un `number` en un cálculo, y un `number` se convierte en un `string` con `..`.

<div class="runner">

```lua
print("10" + 5)
print(10 .. 20)
```

</div>

## `math.floor` y `math.ceil`
La función `math.floor` redondea un `number` hacia abajo, y `math.ceil` hacia arriba:

<div class="runner">

```lua
print(math.floor(3.7))
print(math.ceil(3.2))
```

</div>

Para redondear al `number` entero más cercano, suma `0.5` antes de usar `math.floor`:

<div class="runner">

```lua
print(math.floor(3.7 + 0.5))
print(math.floor(3.2 + 0.5))
```

</div>

## `math.max` y `math.min`
La función `math.max` da el mayor de los `number`s que le das, y `math.min` el menor:

<div class="runner">

```lua
print(math.max(4, 9, 2))
print(math.min(4, 9, 2))
```

</div>

## `math.abs`
La función `math.abs` quita el signo de un `number`. Esto es útil para obtener la distancia entre dos `number`s:

<div class="runner">

```lua
print(math.abs(-5))
print(math.abs(3 - 10))
```

</div>

## `math.sqrt`
La función `math.sqrt` da la raíz cuadrada de un `number`:

<div class="runner">

```lua
print(math.sqrt(16))
```

</div>

---

Puedes consultar todas las funciones `math` aquí: <https://tkr-sh.github.io/lua-std-doc/lua_std/math/index.html>
