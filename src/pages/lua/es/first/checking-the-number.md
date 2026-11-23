Ahora que sabemos cómo

1. generar un número aleatorio
2. obtener la entrada del usuario
3. hacer bucles

Necesitamos ayudar al usuario indicándole si su número es menor o mayor que el número secreto.

Para eso, vamos a usar una nueva palabra clave que utilizarás con frecuencia: `if`

Lo que hace `if` es básicamente ejecutar código dependiendo de una condición. Los bucles comprueban una condición para seguir repitiendo, y `if` comprueba una condición solo para ejecutar una vez.

Por ejemplo, podemos comprobar si el usuario ingresa `42`, y si es así, podemos saludarlo con un mensaje especial.
<div class="runner">

```lua
numero_bueno = 42

if numero_bueno == tonumber(read()) then
    print("¡Buena elección!")
end
```

</div>

Entonces en nuestro juego

```lua
if numero_a_adivinar < intento_usuario then
    print("¡El número es más bajo!")
end

if numero_a_adivinar > intento_usuario then
    print("¡El número es más alto!")
end
```

comprobamos si el número es menor o mayor, y dependiendo de eso, mostramos un mensaje para ayudar al usuario.

Si el usuario encuentra el número correcto, el bucle se detendrá gracias a:

```lua
until intento_usuario == numero_a_adivinar
```

<div class="runner">

```lua
numero_a_adivinar = 42
intento_usuario = tonumber(read())

if numero_a_adivinar < intento_usuario then
    print("¡El número es más bajo!")
end

if numero_a_adivinar > intento_usuario then
    print("¡El número es más alto!")
end

if numero_a_adivinar == intento_usuario then
    print("¡Correcto!")
end
```

</div>

Y con eso hecho, solo tenemos que crear un buen final...
