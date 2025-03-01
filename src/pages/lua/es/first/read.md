```lua
suposicion_usuario = tonumber(read())
```

Ok, así que este código puede parecer extraño (una función dentro de otra?), pero se puede ver en:

```lua
valor_leido = read()
suposicion_usuario = tonumber(valor_leido)
```

Lo que sucede es lo siguiente:
1. Lua quiere calcular el valor correcto 
2. Lua quiere ejecutar `tonumber`, pero para eso, necesita que todas sus entradas tengan valores
3. Por lo tanto, cuando vea `read()`, intentará ejecutarlo de inmediato para que la función produzca un valor
4. Después de que `read()` se ejecute y devuelva un valor, `tonumber` puede ejecutarse ahora.

### Ok, pero ¿qué hace `read()` ?

`read()` pide al usuario una cadena y la captura.

**Antes de ejecutar el ejemplo:** no desactives la ventana emergente para este sitio, de lo contrario, no podrás volver a `read()`.



<div class="runner">

```lua
print(read())
```

</div>

### Y ¿qué hay sobre `tonumber` ?

Como `read()` obtiene una `cadena` del usuario, no podemos compararla con nuestro número aleatorio, porque no se puede comparar una `cadena` que podría tener caracteres no numéricos con un `número` (por ejemplo, `"x4%$A" > 3` no tiene sentido)

Por lo tanto, necesitamos convertir la entrada del usuario a un número y eso es exactamente lo que hace `tonumber`.

Esta función devolverá `nil` si su entrada no es un número válido.

<div class="runner">

```lua
print(tonumber("No es un número") == nil)
```

</div>

Y no puedes sumar `nil` + `número`, así que ingresa un número válido.

<div class="runner">

```lua
print(tonumber(read()) + 1)
```

</div>

Leerá la `cadena` del usuario, la convertirá a `número` y le sumará uno a este número, y si no es un número válido, `tonumber(leer())` será un valor `nil`.
