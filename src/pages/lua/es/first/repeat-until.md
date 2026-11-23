Por primera vez, no nos vamos a centrar en una sola línea. La línea actual es solo

```lua
repeat
```

pero por sí sola no hace nada: va junto con `until`.

```lua
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
```

Pero antes de todo... ¿qué son `repeat` y `until` y por qué son tan especiales? ¿No son solo variables raras?

Quizás recuerdes la definición de una variable

> `el_nombre_de_la_variable` puede ser cualquier cosa. Excepto `true`, `false`, `nil` (y otros que veremos más adelante)

¡Y ese «más adelante»... es ahora!

## Palabras clave

`repeat` y `until` son lo que llamamos «palabras clave». Las palabras clave son nombres especiales que, en lugar de tener un valor (como las variables) o hacer algún cálculo (como las funciones), son instrucciones que le dicen a Lua que tome ciertas decisiones.

En nuestro ejemplo de `repeat`/`until`, todo lo que está entre `repeat` y `until` se repetirá hasta que se cumpla una condición determinada. Cuando la ejecución llega a `until`, comprueba si debe volver a ejecutarse, y si es así, el código vuelve a `repeat` y se repite.

Este tipo de comportamiento - repetir código hasta que _algo_ ocurra - se llama «bucle» y es un concepto muy común en programación.

La mayoría de las veces, quieres salir de tu bucle en algún momento...

### Pero ¿cómo saber cuándo salir?

Cuando usas bucles como `until`, debes proporcionarle un `boolean` (`true` o `false`) para indicarle cuándo dejar de repetir.

Y la mayoría de las veces lo haces con operadores relacionales:

- `==`: Comprueba si dos valores son iguales. **NO CONFUNDIR CON 1 `=`**. 1 `=` sirve para asignar un valor, 2 `=` (`==`) sirven para comprobar que 2 valores son iguales. Si son iguales, el valor es `true`; de lo contrario, es `false`
- `~=`: Comprueba que dos valores son diferentes. Si son diferentes, el valor es `true`; de lo contrario, es `false`
- `>`: Comprueba que el valor de la izquierda es mayor que el de la derecha. Si lo es, el valor es `true`; de lo contrario, es `false`
- `<`: Comprueba que el valor de la izquierda es menor que el de la derecha. Si lo es, el valor es `true`; de lo contrario, es `false`
- `>=`: Comprueba que el valor de la izquierda es mayor **O IGUAL** que el de la derecha. Si lo es, el valor es `true`; de lo contrario, es `false`
- `<=`: Comprueba que el valor de la izquierda es menor **O IGUAL** que el de la derecha. Si lo es, el valor es `true`; de lo contrario, es `false`

<div class="runner">

```lua
print(1984 == 2025)
print(1984 ~= 2025)
print(1984 < 2025)
print(2025 <= 2025)
print(2025 < 2025)
print(1984 > 2025)
```

</div>

### Tipos de bucles

- `repeat`/`until`: repetirá código hasta que se cumpla una condición determinada, y se detendrá en cuanto se cumpla esa condición

<div class="runner">

```lua
edad = 0
repeat
    print(edad)
    edad = edad + 1
until edad == 4
```
</div>

- `while`: los bucles `while` son muy similares a los bucles `repeat`/`until`, ¡pero al revés! Repites mientras se cumpla la condición. Después de la condición, usas la palabra clave `do` para empezar a ejecutar el código, y vuelves al inicio cuando encuentras la palabra clave `end`. En la mayoría de las situaciones en las que puedes usar `repeat`, también puedes usar `while`. Es sobre todo una cuestión de gusto personal.

<div class="runner">

```lua
edad = 0
while edad ~= 4 do
    print(edad)
    edad = edad + 1
end
```
</div>

- `for`: el último bucle es el bucle `for`, y es algo diferente, porque ya sabes cuántas veces quieres iterar. Defines desde qué número hasta qué número quieres iterar. ¡Al final de cada iteración, incrementará automáticamente `edad` en `1`!

<div class="runner">

```lua
for edad = 0, 3 do
    print(edad)
end
```
</div>

## Volviendo a nuestro caso

En nuestro caso, queremos seguir pidiéndole al usuario un número hasta que encuentre el correcto. Por lo tanto escribimos
```lua
repeat
    ...
until intento_usuario == numero_a_adivinar
```
