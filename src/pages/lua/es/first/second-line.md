```lua
numero_a_adivinar = random(1, 100)
```

Ok, así que acabamos de ver que se pueden poner algunos datos en una variable... pero ¿qué es eso?

Para entender qué hace esto, necesitamos entender las `function`s. Este era uno de los tipos de datos que no expliqué durante la primera línea, así que aquí va:

Puedes ver las funciones como una especie de máquina que recibe algunos datos, hace algo con ellos y luego te da un resultado.

## Pero ¿qué hace con ellos?

Bueno, en Lua hay algunos operadores que permiten hacer cosas especiales:

- `+`: Suma 2 números. (p. ej.: `1 + 2` => `3`)
- `-`: Resta un número de otro. (p. ej.: `3 - 2` => `1`)
- `*`: Multiplica 2 números. (p. ej.: `3 * 2` => `6`)
- `/`: Divide un número entre otro. (p. ej.: `6 / 2` => `3`)

Por lo tanto, podemos hacer las siguientes cosas con nuestras variables
```lua
el_mejor_numero = 40 + 2
la_edad_de_mi_abuelo = el_mejor_numero * 2
```

`el_mejor_numero` será 42 y `la_edad_de_mi_abuelo` será 84.

## ¡Volvamos a nuestras funciones!

Por ejemplo, una función podría tomar un número y devolverte el número `+ 1`.

Una función, al igual que una variable, se puede usar cuando conoces su nombre. ¡PERO! Cuando quieres que haga su trabajo y te dé el resultado, debes poner paréntesis después del nombre (p. ej. `mi_funcion()`). Esto ejecutará una función llamada `mi_funcion`. Pero esta función no tiene ninguna entrada. Para añadir una entrada, puedes poner los valores que quieras entre los paréntesis y separarlos por `,`.

### Ejemplo

```lua
dia = 4
mes = "Abril"
anio = 1984
calcular_fecha_de_manana(dia, mes, anio)
```
que es lo mismo que
```lua
calcular_fecha_de_manana(4, "Abril", 1984)
```

## ¡Volvamos a nuestra segunda línea!

```lua
numero_a_adivinar = random(1, 100)
```

Así que esta línea ejecuta la función `random` con los siguientes argumentos: `1` y `100`, y coloca el resultado del cálculo en `numero_a_adivinar`. Nunca definimos qué hace `random`. Pero Lua incluye algunas funciones «integradas» para ayudar a los desarrolladores. ¡Y en nuestro caso, hay una función `random` que toma 2 números como entrada y genera un número aleatorio entre el primer número (en nuestro caso `1`) y el segundo número (en nuestro caso `100`)!

¡Perfecto! ¡Ya hemos generado el número que el usuario tiene que adivinar!
