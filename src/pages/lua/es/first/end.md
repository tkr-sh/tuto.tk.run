Entonces, ahora solo queda imprimir el número de intentos.

Anteriormente incrementamos este valor justo después de obtener los datos del usuario con

```lua
numero_de_intentos = numero_de_intentos + 1
```

Pero ahora, queremos imprimir un mensaje de felicitación y al mismo tiempo, ¿cuántos intentos llevó?

Es hora de aprender el último operador antes del final de este primer juego:

`..`: Concatena dos valores.

<div class="runner">

```lua
print("Ha pasado " .. 4 .. " años")
```

</div>

Con eso en mente, podemos crear un mensaje agradable:

```lua
print("Ganaste en " .. numero_de_intentos .. " intentos!")
```

Y aquí está FINALMENTE nuestro juego:

---

<div class="runner">

```lua
numero_de_intentos = 0
numero_a_adivinar = random(1, 100)

print("Intenta adivinar el número aleatorio entre 1 y 100!")

repeat
	guess_usuario = tonumber(read())
	numero_de_intentos = numero_de_intentos + 1

	if numero_a_adivinar < guess_usuario then
		print("El número es más bajo!")
	end

	if numero_a_adivinar > guess_usuario then
		print("El número es más alto!")
	end
until guess_usuario == numero_a_adivinar

print("Ganaste en " .. numero_de_intentos .. " intentos!")
```

</div>

---

## ¡WOW!.

¡Wow! ¿No es así?

Estás listo para convertirte en programador ahora :) !

## ¿Se acabó?

Sí y no.

Lamentablemente, simplifiqué una cosa: usualmente no puedes ejecutar `read` y `random` de esa manera. Lua gusta de poner las funciones "integradas" en algunas categorías: `math` (para funciones matemáticas), `io` (para interacciones de entrada/salida), `os` (para interactuar con tu sistema operativo), ...

Y por lo tanto, cuando quieras utilizar funciones específicas que son parte de una categoría, debes llamarlas como `categoria.la_funcion()`

En nuestro caso, `random` es parte de la categoría `math`, y `read` es parte de la categoría `io`. `print` y `tonumber` son globales, por lo que no necesitas especificar nada

<div class="runner">

```lua
numero_de_intentos = 0
numero_a_adivinar = math.random(1, 100)

print("Intenta adivinar el número aleatorio entre 1 y 100!")

repeat
	guess_usuario = tonumber(io.read())
	numero_de_intentos = numero_de_intentos + 1

	if numero_a_adivinar < guess_usuario then
		print("El número es más bajo!")
	end

	if numero_a_adivinar > guess_usuario then
		print("El número es más alto!")
	end
until guess_usuario == numero_a_adivinar

print("Ganaste en " .. numero_de_intentos .. " intentos!")
```

</div>

## ¡Felicidades, has terminado tu primer programa Lua! 

Ahora que tienes la base, vamos a tratar de mejorar nuestro juego actual para agregar nuevas funcionalidades, y vamos a aprender cosas nuevas en el camino.
