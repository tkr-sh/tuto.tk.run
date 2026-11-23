Una información útil que necesitamos cuando trabajamos con tablas es «¿cuántos elementos tiene?». Por ejemplo, si quiero obtener el último elemento.

Esto es posible gracias a un operador especial: `#`. Escribiendo `#` y luego el nombre de la tabla o la tabla directamente, puedes obtener el tamaño total de la tabla:


<div class="runner">

```lua
mi_tabla = { "a", "b", "c" }

print(#mi_tabla)
mi_tabla[4] = "d"
print(#mi_tabla)
```

</div>

Esto es especialmente útil si quiero conocer el elemento de una tabla con el índice «último», o si quiero añadir un nuevo elemento al final: `mi_tabla[#mi_tabla]` es el valor cuya clave es la longitud total de la tabla, ¡es decir, el último en la mayoría de los casos!

<div class="runner">

```lua
mi_tabla = { "a", "b", "c" }

print(mi_tabla[#mi_tabla])
mi_tabla[#mi_tabla + 1] = "d"
print(mi_tabla[#mi_tabla])
```

</div>


### Casos límite

El operador `#` solo es fiable cuando las claves de una tabla son los números enteros `1, 2, 3, ...` sin huecos a partir de `1`. Las claves que son strings o booleanos no se cuentan, y si falta un número en medio, el resultado es impredecible:

<div class="runner">

```lua
-- Fiable: las claves son 1, 2, 3
print(#{ "a", "b", "c" }) -- 3
print(#{ [1] = "a", [2] = "b", [3] = "c" }) -- 3

-- No se cuentan: las claves son strings
print(#{ ["a"] = 1, ["b"] = 2, ["c"] = 3 }) -- 0

-- No fiable: hay un hueco en el índice 2
print(#{ [1] = "a", [3] = "b", [4] = "c" }) -- 1 o 4, no te fíes
```

</div>
