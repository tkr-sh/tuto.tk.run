```lua
print("Intenta adivinar el número aleatorio entre 1 y 100!")
```

Esta línea es también una llamada a una función. Podemos verlo ya que tenemos el nombre `print` pero con paréntesis después de su nombre.

Esta función toma una cadena como su primer y único parámetro. Pero... nunca ponemos la salida en una variable.

## ¿Por qué ?

Porque `print` no devuelve un valor. `print` es una función especial que muestra al usuario los datos que recibe en su entrada.

Por ejemplo, tomemos nuestro ejemplo anterior:

```lua
el_mejor_numero = 40 + 2
la_edad_de_mi_abuelo = el_mejor_numero * 2
```

¿Cómo sé qué es el valor de `la_edad_de_mi_abuelo`? Podría adivinar haciendo el cálculo en mi cabeza, pero cuando el programa se vuelve más complejo, esto no siempre es posible.

Por lo tanto, podemos simplemente hacer 

```lua
print(la_edad_de_mi_abuelo)
```

Lo que mostrará el contenido de la variable `la_edad_de_mi_abuelo`. En nuestro caso: `84`

## Ahora que sabes esto

Ahora que conoces esta función muy útil, te presento al _"✨️**Lua runner**✨️"_ & _"✨️**Editor interactivo de Lua**✨️"_, en el que puedes ejecutar tus propios programas:

<div class="runner">

```lua
print("Hola mundo!")
```

</div>

Puedes ver que esto se vuelve a ejecutar cada vez que haces clic en "Ejecutar" con este ejemplo

<div class="runner">

```lua
print(random(1, 100))
```

</div>

Y finalmente, podemos estar seguros de que sabemos la edad de nuestro abuelo correctamente:

<div class="runner">

```lua
el_mejor_numero = 40 + 2
print(el_mejor_numero)
la_edad_de_mi_abuelo = el_mejor_numero * 2
print(la_edad_de_mi_abuelo)
```

</div>
