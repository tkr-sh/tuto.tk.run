Ahora que sabemos cómo

1. generar un número aleatorio
2. obtener la entrada del usuario
3. hacer bucles

Necesitamos ayudar al usuario diciéndole si su número es menor o mayor que el correcto.

Para eso, vamos a usar una nueva palabra clave que utilizarás con frecuencia: `if`

Lo que hace `if` es básicamente ejecutar código dependiendo de una condición. Los bucles comprueban la condición para repetir, y `if` comprueba la condición solo para ejecutar una vez.

Así que por ejemplo, podemos comprobar si el usuario ingresa `42`, y si es así, podemos saludarlo con un mensaje especial.
<div class="runner">

```lua
numero_bueno = 42

if numero_bueno == tonumber(leer()) then
    print("¡Buena elección!")
end
```

</div>

Entonces en nuestro juego

```lua
if numero_a_adivinar < intento_del_usuario then
    print("El número es menor!")
end

if numero_a_adivinar > intento_del_usuario then
    print("El número es mayor!")
end
```

comprobamos si el número es menor/mayor, y dependiendo de eso, imprimimos un mensaje para ayudar al usuario.

Si el usuario encuentra el número correcto se detiene el bucle debido a:

```lua
hasta intento_del_usuario == numero_a_adivinar
```

<div class="runner">

```lua
numero_a_adivinar = 42
intento_del_usuario = tonumber(leer())

if numero_a_adivinar < intento_del_usuario then
    print("El número es menor!")
end

if numero_a_adivinar > intento_del_usuario then
    print("El número es mayor!")
end

if numero_a_adivinar == intento_del_usuario then
    print("¡Correcto!")
end
```

</div>

Y con eso hecho, solo tenemos que hacer un buen final...
