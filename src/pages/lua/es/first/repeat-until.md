Por primera vez, no nos enfocaremos en una sola línea. La línea actual es solo

```lua
repeat
```

pero por sí misma, no hace nada: va con `hasta que`.

```lua
repeat
    intento_usuario = tonumber(read())
    numero_de_intentos = numero_de_intentos + 1

    if numero_a_adivinar < intento_usuario then
        imprimir("El número es más bajo!")
    end

    if numero_a_adivinar > intento_usuario then
        imprimir("El número es más alto!")
    end
until intento_usuario == numero_a_adivinar
```

Pero primero de todo... ¿qué son `repetir` y `hasta que` y por qué son tan especiales? ¿No son solo variables raras?

Quizás recuerdes la definición de una variable

> `el_nombre_de_la_variable` puede ser cualquier cosa. Excepto `true`, `false`, `nil` (y otros que veremos más adelante)

Y el resto... es ahora!

## Palabras clave

`repetir` y `hasta que` son lo que llamamos "palabras clave". Las palabras clave son nombres especiales, que en lugar de tener un valor (como variables) o hacer algún cálculo (como funciones), son instrucciones que le dicen a Lua que tome ciertas decisiones.

En nuestro ejemplo de `repetir`/`hasta que`, todo lo que está entre `repetir` y `hasta que` se repetirá hasta que se cumpla una condición determinada. Cuando la ejecución llega a `hasta que`, verifica si quiere volver a ejecutar, y si lo hace, el código vuelve a `repetir`, y se repite.

Este tipo de comportamiento - repetir algún código hasta _algo_ - se llama "bucle" y es un concepto muy común en programación.

La mayoría de las veces, quieres salir del bucle en un cierto punto...

### Pero ¿cómo saber cuándo salir?

Cuando usas bucles como `hasta que`, debes proporcionar un valor booleano (`true` o `false`) para indicar si quieres continuar con el bucle.

Y la mayoría de las veces, lo haces con operadores relacionales:

- `==`: Verifica si dos valores son iguales. **NO CONFUNDIR CON 1 `=`**. 1 `=` es para asignar un valor, 2 `=` (`==`) es para verificar que dos valores sean iguales. Si son iguales, el valor es `true` de lo contrario es `false`
- `~=`: Verifica si dos valores son diferentes. Si son diferentes, el valor es `true` de lo contrario es `false`
- `>`: Verifica si el valor izquierdo es mayor que el derecho. Si es mayor, el valor es `true` de lo contrario es `false`
- `<`: Verifica si el valor izquierdo es menor que el derecho. Si es menor, el valor es `true` de lo contrario es `false`
- `>=`: Verifica si el valor izquierdo es mayor o igual al derecho. Si es mayor o igual, el valor es `true` de lo contrario es `false`
- `<=`: Verifica si el valor izquierdo es menor o igual al derecho. Si es menor o igual, el valor es `true` de lo contrario es `false`

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

- `repeat`/`until`: repetirá algún código hasta que se cumpla una condición determinada, y se detendrá cuando se cumpla esa condición.

<div class="runner">

```lua
edad = 0
repeat
    imprimir(edad)
    edad = edad + 1
until edad == 4
```

</div>

- `while`: define un bucle que itera sobre una secuencia de valores.

<div class="runner">

```lua
while edad = 0, 3 do
    print(edad)
end
```

</div>

- `for`: El último bucle es el bucle `for` y es un poco diferente, porque ya sabe cuántas veces quiere iterar. Y define desde qué número hasta qué número quiere iterar. Al 
final de una iteración, automáticamente incrementará `edad` en `1`.

<div class="runner">

```lua
for edad = 0, 3 do
    print(edad)
end
```
</div>

## Volviendo a nuestro caso

En nuestro caso, queremos seguir pidiéndole al usuario un número hasta que encuentre el correcto. Por lo tanto usamos 

```lua
repeat
    ...
until intento_usuario == numero_a_adivinar
```

