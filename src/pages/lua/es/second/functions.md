Habrás notado que repetimos un bloque de código similar dos veces para validar la entrada:

```lua
repeat
    print("Intenta adivinar un número:")
    intento_usuario = tonumber(io.read())
    if intento_usuario == nil then 
        print("¡Esto no es un número válido!")
    end
until intento_usuario ~= nil
```

```lua
repeat 
    print("¿Quieres continuar? sí / no")
    respuesta_usuario = string.lower(io.read())
    if respuesta_usuario ~= "sí" and respuesta_usuario ~= "no" then
        print("Esta no es una respuesta válida.")
    end
until respuesta_usuario == "sí" or respuesta_usuario == "no"
```

Hay claramente un patrón común duplicado que intentaré ilustrar con una sintaxis pseudo/inválida delimitada por `<` y `>`.

```lua
repeat 
    print(<pregunta>)
    entrada_usuario = <transformar>(io.read())
    if <condicion_inversa> then
        print(<mensaje_error>)
    end
until <condicion>
```

## Conceptos básicos de una función

Cuando escribes un programa, a menudo hay una regla que seguir: escribir las cosas de forma modular, para que sean útiles en el futuro y lo bastante genéricas como para cubrir la mayoría de los casos, y eso es exactamente lo que las funciones nos ayudan a conseguir.

Una función es básicamente un conjunto de instrucciones que un programa ejecutará. Una función puede tomar parámetros y devolver un valor. Tomemos esta función, por ejemplo:

```lua
function doble(mi_numero)
    return mi_numero * 2
end
```

El nombre de esta función es `doble` y toma un único parámetro llamado `mi_numero`. El resultado de esta función, lo que devuelve, es un único valor: `mi_numero * 2`. Puedes llamar a una función especificando el nombre y luego, entre paréntesis y separados por `,`, los argumentos, así `mi_funcion(valor1, valor2)`. En nuestro caso:

<div class="runner">

```lua
function doble(mi_numero)
    return mi_numero * 2
end

print(doble(42))
```

</div>

## ¡Volvamos a nuestro juego!

¡Así que optimicemos el manejo de la entrada! Lo primero que debemos observar es de qué depende nuestra función. Esto es necesario para saber qué argumentos necesita. En nuestro caso:
* Mensaje de petición, el mensaje que le dice al usuario que introduzca un valor
* Mensaje de error, el mensaje que se muestra cuando la entrada no es válida
* Transformación, una transformación que aplicamos al string que leemos del usuario (`tonumber`, `string.lower`, `string.upper`, ...)
* Condición, una comprobación para saber si la entrada encaja con lo que esperamos después de aplicar la transformación.

`mensaje_peticion` y `mensaje_error` son bastante sencillos, son solo strings.<br/>

`transformar` y `condicion` son un poco más complicados.<br>
`transformar` toma la entrada del usuario y devuelve una versión modificada de ella. Este es claramente el caso de una función: algo que toma una entrada y devuelve algo.<br/>
`condicion` también es muy parecida: toma como argumento el resultado de `transformar` y devuelve un booleano (`true`/`false`) según el resultado. Este es claramente otro caso en el que se necesita una `function`.

```lua
function leer_validado(mensaje_peticion, mensaje_error, transformar, condicion)
    repeat 
        print(mensaje_peticion)
        entrada_usuario = transformar(io.read())
        es_valido = condicion(entrada_usuario)

        if not es_valido then
            print(mensaje_error)
        end
    until es_valido

    return entrada_usuario
end
```

¡Fíjate en el `not`! La palabra clave `not` invierte un booleano: `true` => `false`, `false` => `true`.

En nuestro caso podemos simplemente escribir esto para el manejo de la entrada del número por parte del usuario:

```lua
function es_no_nil(arg) 
    return arg ~= nil
end

leer_validado(
    "Intenta adivinar un número:",
    "Esto no es un número válido",
    tonumber,
    es_no_nil
)
```

Esto producirá lo mismo que:

```lua
repeat
    print("Intenta adivinar un número:")
    intento_usuario = tonumber(io.read())
    es_valido = intento_usuario ~= nil
    if not es_valido then 
        print("¡Esto no es un número válido!")
    end
until es_valido
```

## Funciones en línea

Fíjate en que definimos `es_no_nil` solo para usarla una vez. En lugar de declarar una función con nombre cuando no lo necesitamos, Lua nos permite simplemente quitar el nombre cuando no queremos usarla en otro sitio, así:

```lua
local doble = function(numero)
    return numero * 2
end
```

En nuestro caso, podemos usarla para hacer:

```lua
leer_validado(
    "Intenta adivinar un número:",
    "Esto no es un número válido",
    tonumber,
    function(nb) return nb ~= nil end
)
```

Lo que nos da nuestro programa final:

<div class="runner">

```lua
function leer_validado(mensaje_peticion, mensaje_error, transformar, condicion)
    repeat 
        print(mensaje_peticion)
        entrada_usuario = transformar(io.read())
        es_valido = condicion(entrada_usuario)

        if not es_valido then
            print(mensaje_error)
        end
    until es_valido

    return entrada_usuario
end

repeat 
	numero_de_intentos = 0
	numero_a_adivinar = math.random(1, 100)

	print("¡Intenta adivinar el número aleatorio entre 1 y 100!")

	repeat
        intento_usuario = leer_validado(
            "Introduce un número entre 1 y 100:",
            "Esto no es un número válido",
            tonumber,
            function(nb) return nb ~= nil end
        )
		numero_de_intentos = numero_de_intentos + 1

		if numero_a_adivinar < intento_usuario then
			print("¡El número es más bajo!")
		end

		if numero_a_adivinar > intento_usuario then
			print("¡El número es más alto!")
		end
	until intento_usuario == numero_a_adivinar

	print("¡Ganaste en " .. numero_de_intentos .. " intentos!")

    respuesta_usuario = leer_validado(
        "¿Quieres continuar? sí / no",
        "Esta no es una respuesta válida.",
        string.lower,
        function(respuesta) return respuesta == "sí" or respuesta == "no" end
    )
until respuesta_usuario == "no"

print("¡Adiós!")
```

</div>
