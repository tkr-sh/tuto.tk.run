Otra clave para trabajar con tablas es recorrerlas. Digamos que tenemos, en nuestro juego, la tabla `scores` (siendo un `score` el `numero_de_intentos` que un usuario necesitó para encontrar el número). ¿Cómo mostrar cada puntuación de cada partida?

## `for` con índice
Primero, quizás recuerdes la palabra clave `for` de la que hablamos rápidamente antes:

<blockquote>

`for`: el último bucle es el bucle `for`, y es algo diferente, porque ya sabes cuántas veces quieres iterar. Defines desde qué número hasta qué número quieres iterar. ¡Al final de cada iteración, incrementará automáticamente `age` en `1`!

<div class="runner">

```lua
for age = 0, 3 do
    print(age)
end
```
</div>

</blockquote>

Si relacionamos esto con nuestro nuevo conocimiento del operador `#`, ¡podemos recorrer una tabla con éxito!

<div class="runner">

```lua
scores = {
    [1] = 7,
    [2] = 8,
    [3] = 4,
}

for clave = 1, #scores do
    print(clave .. ": " .. scores[clave])
end
```
</div>

¡Esto funciona muy bien! Pero puede ser un poco redundante escribir siempre `scores[clave]` para acceder al valor, sobre todo si no te importa la clave, o si no piensas volver a usar la tabla. Por suerte, Lua tiene otra forma de hacerlo.

## `ipairs`

`ipairs` es una función que usaremos con la palabra clave `for` para acceder rápidamente a la clave y al valor sin tener que referenciar el nombre de la tabla. Su sintaxis es un poco distinta del bucle `for` anterior, aunque sigue usando la palabra clave `for`:

```lua
for mi_clave, mi_valor in ipairs(mi_tabla) do 
    -- ¡Haz algo!
end
```

En este ejemplo, suponemos que tenemos una `table` llamada `mi_tabla`. Cada línea de código entre el `for` y el `end` se ejecutará con `mi_clave` y `mi_valor` siendo la clave y el valor asociado de `mi_tabla`.

Como el bucle `for` anterior, está ordenado de forma ascendente. Es decir, empieza en `1` y aumenta en `1` en cada iteración. Pero no funciona cuando las claves son `string`s o si hay huecos entre los números. Si ese es nuestro caso, deberíamos usar `pairs`.

## `pairs`

`pairs` es como `ipairs`, con la diferencia de que funciona con claves que no son enteros consecutivos a partir de `1`. El inconveniente es que el orden en que llegan las claves es impredecible:

<div class="runner">

```lua
mi_tabla = {
    [3] = 7,
    ["name"] = 8,
    [false] = 4,
}

for clave, valor in pairs(mi_tabla) do
    print(clave)
    print(valor)
end
```

</div>

El orden en que se muestran las claves/valores es impredecible y puede variar de una ejecución a otra. Sin embargo, tiene la ventaja de permitirnos recorrer cualquier tabla.

## Volviendo a nuestro juego
En nuestro caso, solo queremos conservar el `numero_de_intentos` de cada partida. Por lo tanto, no necesitamos claves personalizadas: podemos tener índices que empiecen en `1` y aumenten en `1` cada vez, correspondiendo al número de partida.
