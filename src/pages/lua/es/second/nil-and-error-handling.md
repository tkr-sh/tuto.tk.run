Lo primero que quizás hayas notado es que nuestro juego *falla* actualmente cuando introducimos un número que no es válido:

<div class="runner">

```lua
intento_usuario = tonumber(io.read())

if 10 < intento_usuario then
	print("¡El número es más bajo!")
end
```

</div>

lo cual, si no escribes un número, te dará:

```
Error: [string "intento_usuario = tonumber(io.read())..."]:3: attempt to compare number with nil
```

Pero ¿qué significa esto?

El concepto clave aquí es `nil`. Al presentar los tipos de Lua, vimos brevemente el tipo `nil`:

> `nil`: Vacío. El objetivo de `nil` es decir que no tienes un valor.

<div style="height: 3px"></div>

Y esto es _exactamente_ lo que hace `tonumber`:

- Si `io.read()` es un string que representa un número como `"3.14"` o `"42"`, Lua sabe convertirlo de un tipo `string` a un tipo `number`.
- Si no, si `io.read()` es un string que no se parece a un `number`, `tonumber` simplemente dice «no sé cómo convertir esto a un número, devolvamos `nil`».

Por lo tanto, si en `io.read()` no introducimos una representación correcta de un número, simplemente tendremos `nil`:

<div class="runner">

```lua
intento_usuario = tonumber("123abc")
print(intento_usuario)
```

</div>

## Ok, pero entonces ¿por qué da un error?

¡Buena pregunta! Por ahora, `intento_usuario` es simplemente `nil`. El error ocurre cuando intentamos comparar con el operador `<` un `number` con `nil` en esta línea:

```lua
if 10 < intento_usuario then
```

Y el error se describe bien en el mensaje de error:

```
Error: [string "intento_usuario = tonumber(io.read())..."]:3: attempt to compare number with nil
```

- `:3` indica la línea en la que ocurre el error; en este caso, nuestra línea de comparación.
- `attempt to compare number with nil` es la razón por la que ha habido un error.

¡Y esto tiene todo el sentido! ¿Qué significa comparar, en términos de mayor o menor, nada con un número? ¿Es `false`? ¿Es `true`? No, la pregunta simplemente no tiene ningún sentido. Así que Lua da un error.

## Manejar el error
La forma más rápida de manejar esto es simplemente comprobar si `intento_usuario` es `nil`, y esto se puede hacer con el operador `==`:

```lua
if intento_usuario == nil then
	print("Esto no es un número válido")
end
```

Pero en este caso, no queremos simplemente mostrar algo y continuar el bucle, porque lo contaría como un intento debido a:

```lua
numero_de_intentos = numero_de_intentos + 1
```

Una forma sencilla de hacerlo sería pedir `intento_usuario` hasta que no sea `nil` con el operador `~=`, es decir, hasta que sea un `number` válido:

```lua
repeat
	intento_usuario = tonumber(io.read())
until intento_usuario ~= nil
```

También podemos añadir un mensaje de error en caso de que no lo sea, con:

```lua
repeat
	intento_usuario = tonumber(io.read())
	if intento_usuario == nil then 
		print("¡Esto no es un número válido!")
	end
until intento_usuario ~= nil
```

Lo que nos da este programa completo:


<div class="runner">

```lua
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
```

</div>
