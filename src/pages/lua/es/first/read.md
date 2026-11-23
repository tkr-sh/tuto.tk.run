```lua
suposicion_usuario = tonumber(read())
```

Ok, este código puede parecer extraño (¿una función dentro de otra?), pero se puede ver así:

```lua
valor_leido = read()
suposicion_usuario = tonumber(valor_leido)
```

Lo que sucede es lo siguiente:
1. Lua quiere calcular el valor de la derecha 
2. Lua quiere ejecutar `tonumber`, pero para eso necesita que todas sus entradas tengan un valor
3. Por lo tanto, cuando ve `read()`, intenta ejecutarlo de inmediato para que la función produzca un valor
4. Una vez que `read()` se ha ejecutado y ha devuelto un valor, `tonumber` ya puede ejecutarse.

### Ok, pero ¿qué hace `read()`?

`read()` pide al usuario una cadena y la captura.

**Antes de ejecutar el ejemplo:** no desactives las ventanas emergentes de este sitio, de lo contrario no podrás volver a usar `read()`.



<div class="runner">

```lua
print(read())
```

</div>

### ¿Y qué hay de `tonumber`?

Como `read()` obtiene un `string` del usuario, no podemos compararlo con nuestro número aleatorio, porque no se puede comparar un `string` que podría contener caracteres no numéricos con un `number` (por ejemplo, `"x4%$A" > 3` no tiene sentido).

Por lo tanto, necesitamos convertir la entrada del usuario a un número, y eso es exactamente lo que hace `tonumber`.

Esta función devolverá `nil` si su entrada no es un número válido.

<div class="runner">

```lua
print(tonumber("No es un número") == nil)
```

</div>

Y no puedes sumar `nil` + `number`, así que ingresa un número válido.

<div class="runner">

```lua
print(tonumber(read()) + 1)
```

</div>

Esto leerá el `string` del usuario, lo convertirá a `number` y le sumará uno a ese número; y si no es un número válido, `tonumber(read())` será un valor `nil`.
