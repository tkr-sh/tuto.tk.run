## Funciones en línea

Hasta ahora, escribíamos las funciones así:

```lua
function doble(number)
    return number * 2
end
```

Esto es azúcar. Lo que Lua hace realmente es crear una función sin nombre y guardarla en una variable llamada `doble`:

```lua
doble = function(number)
    return number * 2
end
```

Ambas versiones son exactamente iguales. Por eso también podíamos pasar `tonumber` a `leer_validado`: una función no es más que un valor guardado en una variable, como un `number` o un `string`.


## Llamar funciones con `:`

Para poner una cadena en minúsculas, escribíamos `string.lower(respuesta_usuario)`.
Pero, cuando usas una función de la librería `string` o `table` que recibe como primer argumento un `string` o una `table`, puedes escribirla con la siguiente sintaxis: `respuesta_usuario:lower()`:

<div class="runner">

```lua
local respuesta = "SÍ"

print(string.lower(respuesta))
print(respuesta:lower())
```

</div>

El `:` toma el valor de su izquierda y se lo da a la función como primer argumento. Para una cadena, Lua busca la función en la tabla `string`, así que `respuesta:lower()` se convierte en `string.lower(respuesta)`.

<div class="note">

Esto no funciona con una cadena escrita directamente en el código: `"SÍ":lower()` es un error de sintaxis, porque Lua solo acepta `:` después de un nombre de variable o de unos paréntesis. Para que funcione, pon la cadena entre paréntesis:

```lua
print(("SÍ"):lower())
-- Esto da un error:
print("SÍ":lower())
```

</div>

## Sin paréntesis

Cuando se llama a una función con un solo argumento, y ese argumento es un `string` o una `table` escritos directamente en el código, los paréntesis son opcionales:

<div class="runner">

```lua
print "¡Hola!"
print("¡Hola!")
```

</div>

Verás esto en el código de otras personas. En este curso mantenemos los paréntesis, porque dejan claro que se está llamando a una función.

## Devolver varios valores

Una función puede devolver más de un valor. Los separas con `,` después del `return`, y los recibes en varias variables, también separadas con `,`:

<div class="runner">

```lua
function primero_y_ultimo(lista)
    return lista[1], lista[#lista]
end

local primero, ultimo = primero_y_ultimo({ "a", "b", "c", "d" })

print(primero)
print(ultimo)
```

</div>

Si das menos variables que valores, los valores sobrantes simplemente se descartan. Si das más, las variables sobrantes son `nil`:

<div class="runner">

```lua
function primero_y_ultimo(lista)
    return lista[1], lista[#lista]
end

local solo_primero = primero_y_ultimo({ "a", "b", "c", "d" })
local primero, ultimo, otro = primero_y_ultimo({ "a", "b", "c", "d" })

print(solo_primero)
print(primero, ultimo, otro)
```

</div>

<div class="note">

Para quedarte solo con el primer valor devuelto por una función, pon la llamada entre paréntesis:

<div class="runner">

```lua
function primero_y_ultimo(lista)
    return lista[1], lista[#lista]
end

print( primero_y_ultimo({ "a", "b", "c", "d" }) )
print((primero_y_ultimo({ "a", "b", "c", "d" })))
```
</div>

</div>

## Argumentos faltantes y sobrantes

Lua no comprueba el número de argumentos que le das a una función. Un argumento que falta es `nil`, y un argumento que no se espera se ignora:

<div class="runner">

```lua
function mostrar(a, b)
    print(a, b)
end

mostrar(1, 2)
mostrar(1)
mostrar(1, 2, 3)
```

</div>

## Argumentos opcionales

Como un argumento faltante es `nil`, podemos comprobarlo y usar un valor por defecto en su lugar. Esto hace que el argumento sea opcional:

<div class="runner">

```lua
function saludar(nombre, saludo)
    if saludo == nil then
        saludo = "Hola"
    end

    print(saludo .. ", " .. nombre .. "!")
end

saludar("Bob")
saludar("Bob", "Buenas noches")
```

</div>
