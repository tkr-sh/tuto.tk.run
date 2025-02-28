```lua
numero_a_adivinar = random(1, 100)
```

Ok, así que acabamos de ver que se puede poner algunos datos en una variable... pero ¿qué es eso ???

Para entender qué hace esto, necesitamos entender las `funciones`. Esto fue uno de los tipos de datos que no expliqué durante la primera línea, así que aquí está:

Puedes ver funciones como una especie de máquina que toma algunos datos de entrada, hace algo con ellos y luego te da un resultado.

## Pero ¿qué hace con ellos ?? 

Bueno, en Lua hay algunos operadores que permiten hacer cosas especiales:

- `+`: Suma 2 números. (e.g: `1 + 2` => `3`)
- `-`: Resta un número de otro. (e.g: `3 - 2` => `1`)
- `*`: Multiplica 2 números. (e.g: `3 * 2` => `6`)
- `/`: Divide un número entre otro. (e.g: `6 / 2` => `3`)

Por lo tanto, podemos hacer las siguientes cosas con nuestras variables
```lua
el_mejor_numero = 40 + 2
la_edad_de_mi_abuelo = el_mejor_numero * 2
```

`el_mejor_numero` será 42 y `la_edad_de_mi_abuelo` será 84.

## ¡Volvamos a nuestras funciones!

Por ejemplo, una función podría tomar un número y devolverte el número `+ 1`.

Una función, al igual que una variable, se puede usar cuando conoces su nombre. PERO! Cuando quieres que haga su "mecanismo" y te dé el resultado, debes poner paréntesis después del nombre (e.g. `mi_funcion()`). Esto ejecutará una función llamada `mi_funcion`. Pero esta función no tiene ninguna entrada! Para agregar entrada a ella, puedes poner los valores que deseas entre los paréntesis y separarlos por `,`.

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

Así que esta línea está ejecutando la función `random` con como entrada: `1` y `100`, y poniendo el resultado de la computación en `numero_a_adivinar`. Nunca definimos qué hace `random`. Pero ¡Lua viene con algunas funciones "integradas" para ayudar a los desarrolladores! Y en nuestro caso, hay una función `random` que quiere 2 números como entrada y que genera un número aleatorio entre el primer número (en nuestro caso `1`) y el segundo número (en nuestro caso `100`).

