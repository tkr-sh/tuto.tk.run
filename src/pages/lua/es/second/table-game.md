¡Ahora que sabemos todo esto, podemos hacer que funcione nuestro historial de partidas!

Para crear un historial de nuestras puntuaciones, podemos crear una nueva tabla `scores` al principio de nuestro programa y, cada vez que terminemos una partida, añadirle el `numero_de_intentos` como `scores[#scores + 1]`, es decir, el último elemento.

Una vez terminada la partida, ¡podemos simplemente recorrer la tabla con `ipairs` y mostrar todas nuestras puntuaciones!


Esto nos da nuestro programa final:

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

scores = {}

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

    scores[#scores + 1] = numero_de_intentos

    respuesta_usuario = leer_validado(
        "¿Quieres continuar? sí / no",
        "Esta no es una respuesta válida.",
        string.lower,
        function(respuesta) return respuesta == "sí" or respuesta == "no" end
    )
until respuesta_usuario == "no"

print("Puntuaciones:")

for numero_de_partida, score in ipairs(scores) do
    print("Partida " .. numero_de_partida .. ": ¡terminada en " .. score .. " intentos!")
end

print("¡Adiós!")
```

</div>
