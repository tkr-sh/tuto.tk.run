Entonces, ahora lo único que queda es mostrar el número de intentos.

Antes lo incrementamos justo después de obtener los datos del usuario con

```lua
numero_de_intentos = numero_de_intentos + 1
```

Pero ahora, quisiéramos mostrar un mensaje de felicitación y, al mismo tiempo, cuántos intentos hizo falta.

Es hora de aprender el último operador antes del final de este primer juego:

`..`: Concatena 2 valores.

<div class="runner">

```lua
print("Han pasado " .. 4 .. " años")
```

</div>

Con eso en mente, podemos crear un mensaje agradable:

```lua
print("¡Ganaste en " .. numero_de_intentos .. " intentos!")
```

Y aquí está POR FIN nuestro juego:

---

<div class="runner">

```lua
numero_de_intentos = 0
numero_a_adivinar = random(1, 100)

print("¡Intenta adivinar el número aleatorio entre 1 y 100!")

repeat
	intento_usuario = tonumber(read())
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

---

## ¡WOW!

¡Wow! ¿A que sí?

¡Ya estás listo para convertirte en programador! :) 

## ¿Se acabó?

Sí y no.

Lamentablemente, simplifiqué una cosa: normalmente no puedes ejecutar `read` y `random` de esa manera. A Lua le gusta agrupar las funciones «integradas» en algunas categorías: `math` (para las funciones matemáticas), `io` (para las interacciones de entrada/salida), `os` (para interactuar con tu sistema operativo), ...

Y por lo tanto, cuando quieras utilizar funciones específicas que forman parte de una categoría, debes llamarlas como `categoria.la_funcion()`

En nuestro caso, `random` forma parte de la categoría `math`, y `read` forma parte de la categoría `io`. `print` y `tonumber` son globales, así que no necesitas especificar nada.

<div class="runner">

```lua
numero_de_intentos = 0
numero_a_adivinar = math.random(1, 100)

print("¡Intenta adivinar el número aleatorio entre 1 y 100!")

repeat
	intento_usuario = tonumber(io.read())
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

## ¡Felicidades, has terminado tu primer programa Lua! 🎉

Ahora que tienes las bases, vamos a intentar mejorar nuestro juego actual para añadir nuevas funcionalidades, y aprenderemos cosas nuevas por el camino.
