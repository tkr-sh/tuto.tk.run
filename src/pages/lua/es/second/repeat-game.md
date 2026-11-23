¿Y si pudiéramos tener varias partidas en una sola ejecución y dejar que el usuario decida cuándo parar?

Para hacer que el juego sea repetible, simplemente envolvemos el programa en otro bucle `repeat ... until` así:

```lua
repeat
	...
	print("¡Ganaste en ... intentos!")

	repeat 
		print("¿Quieres continuar? sí / no")
		respuesta_usuario = io.read()
	until respuesta_usuario == "sí" or respuesta_usuario == "no"
until respuesta_usuario == "no"

print("¡Adiós!")
```

Usamos una palabra clave especial `or`, que combina dos condiciones y es `true` si al menos una de ellas es verdadera.

Digamos, por ejemplo, que tenemos un booleano `a` que puede ser `true` o `false`, y otro booleano `b` que también puede ser `true` o `false`. La expresión `a or b` será por lo tanto:

|a|b|a or b|
|-|-|------|
|`true`|`true`|`true`|
|`true`|`false`|`true`|
|`false`|`true`|`true`|
|`false`|`false`|`false`|

`a or b` solo es `false` cuando `a` y `b` son `false`.
Es lo contrario del operador `and`, que exige que `a` y `b` sean `true` para valer `true`.

|a|b|a and b|
|-|-|-------|
|`true`|`true`|`true`|
|`true`|`false`|`false`|
|`false`|`true`|`false`|
|`false`|`false`|`false`|


## Mayúsculas y minúsculas

En Lua, un string con letras mayúsculas como `"A"`, `"B"`, `"C"`, ... no es el mismo que `"a"`, `"b"`, `"c"`, .... Esto significa que si un usuario escribe `"Sí"`, no funcionará y el usuario repetirá el bucle `repeat` aunque haya escrito `"Sí"`. Esto probablemente no es lo que queremos...

Por suerte, Lua tiene funciones que nos permiten convertir un string a todo mayúsculas o todo minúsculas: `string.upper` y `string.lower`

<div class="runner">

```lua
print(string.lower("Abc")) -- abc
print(string.upper("Abc")) -- ABC
```

</div>

Lo que nos permite hacer lo siguiente

<div class="runner">

```lua
repeat 
	print("¿Quieres continuar? sí / no")
	respuesta_usuario = string.lower(io.read())
	if respuesta_usuario ~= "sí" and respuesta_usuario ~= "no" then
		print("Esta no es una respuesta válida.")
	end
until respuesta_usuario == "sí" or respuesta_usuario == "no"
```

</div>

Con esto, `respuesta_usuario` siempre estará en minúsculas gracias a `string.lower()`.
También añadimos una comprobación para mostrar un mensaje si la respuesta no es ni `"sí"` ni `"no"`. 
Lo que nos da este programa completo:

<div class="runner">

```lua
repeat 
	numero_de_intentos = 0
	numero_a_adivinar = math.random(1, 100)

	print("¡Intenta adivinar el número aleatorio entre 1 y 100!")

	repeat
		repeat
			intento_usuario = tonumber(io.read())
			if intento_usuario == nil then 
				print("¡Esto no es un número válido!")
			end
		until intento_usuario ~= nil
		numero_de_intentos = numero_de_intentos + 1

		if numero_a_adivinar < intento_usuario then
			print("¡El número es más bajo!")
		end

		if numero_a_adivinar > intento_usuario then
			print("¡El número es más alto!")
		end
	until intento_usuario == numero_a_adivinar

	print("¡Ganaste en " .. numero_de_intentos .. " intentos!")

	repeat 
		print("¿Quieres continuar? sí / no")
		respuesta_usuario = string.lower(io.read())
		if respuesta_usuario ~= "sí" and respuesta_usuario ~= "no" then
			print("Esta no es una respuesta válida.")
		end
	until respuesta_usuario == "sí" or respuesta_usuario == "no"
until respuesta_usuario == "no"

print("¡Adiós!")
```

</div>
