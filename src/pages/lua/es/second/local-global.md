Hasta ahora, solo hemos declarado variables de Lua con esta sintaxis:

```lua
mi_variable = "valor"
```

Y funciona, claramente, como hemos visto. Pero puede que no siempre sea la mejor forma de hacerlo.
Tomemos este escenario:

<div class="runner">

```lua
function mi_funcion(a)
    b = a * 2
    return a + b
end

print(mi_funcion(2))
print(b)
```
</div>

`b` sigue estando definida _fuera_ de la función. Lo cual, a veces, puede ser lo que quieres. Pero la mayoría de las veces no es el comportamiento esperado. La mayoría de las veces, quieres crear una variable que solo exista dentro de una función, de un bucle, de un ámbito concreto.

Para definir un valor que solo exista en un ámbito concreto, podemos simplemente poner la palabra clave `local` antes del nombre de la variable, así:

```lua
local mi_variable = "valor"
```

Con esto, la variable existirá hasta que llegue al final del bloque (`end`, `repeat ... until`, ...) en el que fue definida, o al final del archivo si fue definida fuera de un bloque. Con esto, podemos arreglar nuestro ejemplo:

<div class="runner">

```lua
function mi_funcion(a)
    local b = a * 2
    return a + b
end

print(mi_funcion(2))
print(b)
```

</div>

<div class="note">

Los valores de `for` son `local` por defecto

<div class="runner">

```lua
for a = 0, 3 do
    local b = a * 2
end

print(a)

for k, v in ipairs({1, 2, 3}) do
    local b = k + v
end

print(k)
print(v)
```
</div>
</div>


<div class="note">

Declarar una variable y asignarle un valor son dos cosas distintas:

- **Declarar** crea la variable con el ámbito de `local`: `local mi_variable`
- **Asignar** da un valor a una variable: `mi_variable = "valor"`

Escribir `local mi_variable = "valor"` simplemente hace las dos cosas a la vez.

También puedes declarar una variable sin darle un valor. Contendrá `nil` hasta que le asignes algo:

```lua
local intento_usuario
print(intento_usuario) -- nil

intento_usuario = 42
print(intento_usuario) -- 42
```

Escribe `local` una sola vez por variable: un segundo `local intento_usuario = 42` crearía una nueva variable (posiblemente en un nuevo ámbito) en lugar de cambiar la primera.

</div>

Y podemos terminar nuestro juego con este ejemplo final:

<div class="runner">

```lua
function leer_validado(mensaje_peticion, mensaje_error, transformar, condicion)
    local entrada_usuario

    repeat 
        print(mensaje_peticion)
        entrada_usuario = transformar(io.read())
        local es_valido = condicion(entrada_usuario)

        if not es_valido then
            print(mensaje_error)
        end
    until es_valido

    return entrada_usuario
end

local scores = {}

repeat 
	local numero_de_intentos = 0
	local numero_a_adivinar = math.random(1, 100)

	print("¡Intenta adivinar el número aleatorio entre 1 y 100!")

	repeat
        local intento_usuario = leer_validado(
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

    local respuesta_usuario = leer_validado(
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
