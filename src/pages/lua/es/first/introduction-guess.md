Este es el primer programa que vamos a estudiar. ¡No te preocupes, lo vamos a hacer MUY despacio, así que está bien si no entiendes nada todavía!

```lua
numero_de_intentos = 0
numero_a_adivinar = random(1, 100)

print("¡Trata de adivinar el número aleatorio entre 1 y 100!")

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

Lo que este programa hace es crear el juego de "adivina el número":

```
¡Trata de adivinar el número aleatorio entre 1 y 100!
50
¡El número es más alto!
75
¡El número es más bajo!
62
¡Ganaste en 3 intentos!
```

