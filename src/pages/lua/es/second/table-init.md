¿Y si ahora pudiéramos tener un historial de nuestras puntuaciones, hasta que terminemos de jugar? ¿Un historial de todo lo que ha ocurrido? ¿Cómo lo haríamos?

Tu primer instinto podría ser definir varias variables así:

```lua 
puntuacion_primera_partida = nil
puntuacion_segunda_partida = nil
puntuacion_tercera_partida = nil
...
```

Pero esto no es un buen enfoque. No es sostenible si el usuario juega muchas veces. Es difícil de vincular limpiamente con un bucle.

Para resolverlo, necesitaremos un nuevo tipo de dato: `table`.

## ¿Qué es una tabla?

Una `table` es un tipo de Lua que nos permite almacenar una cantidad de datos arbitrariamente larga o corta. Cada tabla de Lua asigna una clave a un valor con el formato `[clave] = valor`, así:

```lua
mi_tabla = {
    ["a"] = 7,
    ["b"] = 8,
    ["c"] = 4,
}
```

Podemos acceder al valor de una tabla referenciando la clave y escribiendo `mi_tabla[mi_clave]`. Por ejemplo:

<div class="runner">

```lua
mi_tabla = { ["a"] = 7, ["b"] = 8, ["c"] = 4 }

print(mi_tabla["b"])
```

</div>

Y podemos simplemente añadir un nuevo valor a nuestra tabla asignando una clave inexistente a un valor, así:

<div class="runner">

```lua
mi_tabla = { ["a"] = 7, ["b"] = 8, ["c"] = 4 }

mi_tabla["d"] = 5
print(mi_tabla["d"])
```

</div>

También puedes no escribir ninguna clave y, en ese caso, las claves empezarán en uno y aumentarán después:


<div class="runner">

```lua
mi_tabla = { "a", "b", "c", "z" }
-- Igual que
-- mi_tabla = { [1] = "a", [2] = "b", [3] = "c", [4] = "z" }

print(mi_tabla[2])
print(mi_tabla[4])
```

</div>
