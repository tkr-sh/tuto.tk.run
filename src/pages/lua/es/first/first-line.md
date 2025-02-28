```lua
numero_de_intentos = 0
```

Esta es la primera línea que vemos en el programa.

Normalmente, cuando quieres entender qué hace un programa, lo lees línea por línea, como si estuvieras leyendo un texto en inglés normal, así que empezamos aquí.


## ¿Genial, pero qué hace esta línea?

En los lenguajes de programación, a menudo tienes el concepto de una "variable". Una variable es como una caja etiquetada que contiene algunos datos. Una vez que creas una variable, puedes leer su contenido - equivalente a abrir la caja - y incluso cambiarlo (de ahí el nombre "variable") - equivalente a poner un nuevo valor en la caja.

### Pero ¿qué tipo de dato puede contener?

En Lua hay 6[^1] tipos de datos:
- `number`: Un número como en matemáticas. Puedes declarar un número simplemente escribiéndolo: `123`. <br/> **Nota**: para números con decimales (por ejemplo, `0.5`), usa `.` (punto) y no `,` (coma).
- `boolean`: O `true` o `false`. (Por ejemplo, "¿Es 2 igual a 3?" => `false`).
- `nil`: Vacío. El objetivo de `nil` es decir que no tienes un valor.
- `string`: Un grupo de letras, números o símbolos (por ejemplo, `"Hola!"`). Para declarar una cadena, debes poner comillas (`'` o `"`) alrededor del contenido (por ejemplo, para tener la cadena `"Hola!"`, debes escribir `"Hola!"`).
- `table`: Lo veremos más adelante.
- `function`: Lo veremos más adelante.

Cuando quieras asignar algún dato a una variable, simplemente escribe:
```lua
el_nombre_de_la_variable = el_valor
```
`el_nombre_de_la_variable` puede ser cualquier cosa. Excepto `true`, `false`, `nil` (y otros que veremos más adelante)

`el_valor` un tipo válido como se describe anteriormente.

Aquí hay algunos ejemplos:
```lua
un_numero_negativo = -1
un_gran_numero = 100000000
nombre_del_abuelo = "Juan"
sin_valor = nil
esta_lloviendo = false
```

También puedes asignar el contenido de una variable a otra variable:

```lua
nombre_del_abuelo = "Juan"
mi_amigo_abuelo_nombre = nombre_del_abuelo
```

En este contexto, tanto `nombre_del_abuelo` como `mi_amigo_abuelo_nombre` serán la cadena `"Juan"`.

## Volviendo a nuestra primera línea

```lua
numero_de_intentos = 0
```

Entonces, esta línea simplemente pone el número `0` en la variable `numero_de_intentos`. Y esto tiene sentido. Porque al principio del programa, el usuario no ha intentado adivinar el número. Y como indica el nombre `numero_de_intentos`, esta variable contendrá en el futuro el número de intentos que hizo el usuario.


[^1]: 

<div class="for-technical">

Bueno, en realidad hay 8 tipos pero por ahora solo 6 son importantes :). <https://www.lua.org/pil/2.html>

</div>

