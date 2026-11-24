## Кавычки

Строку можно записать с помощью двойных кавычек `"..."` или одинарных `'...'`. Оба варианта дают в точности одну и ту же строку. Это полезно, когда ваш текст содержит один из этих двух символов:

<div class="runner">

```lua
print("Сегодня хороший день")
print('Он сказал "привет"')
```

</div>

## Многострочные строки

Вы не можете просто добавить перенос строки в середину строки `"..."`, вот так:

<div class="runner">

```lua
print("строка 1
строка 2")
```
</div>

Lua просто не позволяет иметь такой перенос строки в строке, когда она записана с помощью `"` или `'`.

ОДНАКО! Вы можете сделать это, заменив `"` или `'` на `[[`/`]]`

<div class="runner">

```lua
print([[строка1
строка2]])
```

</div>

## Длина

Оператор `#`, который мы использовали для таблиц, также работает со строками:

<div class="runner">

```lua
print(#"hello")
print(#"hello world")
```

</div>

## `string.sub`
Функция `string.sub` позволяет создать новую `string` из части этой `string`, указав начальный и конечный индекс:

<div class="runner">

```lua
print(("hello"):sub(2, 4))
-- То же самое, что
-- print(string.sub("hello", 2, 4))
```

</div>

## `string.rep`
Функция `string.rep` позволяет повторить `string` несколько раз:

<div class="runner">

```lua
print(("hello"):rep(3))
```

</div>

## `string.reverse`
Функция `string.reverse`, как следует из названия, переворачивает `string`:

<div class="runner">

```lua
print(("hello"):reverse())
```

</div>

## `string.gsub`
Функция `string.gsub` заменяет одну `string` на другую и также выводит, сколько раз произошла замена:

<div class="runner">

```lua
print(("hello"):gsub("lo", "d"))
print(("hello"):gsub("l", "f"))
```

</div>

---

Все функции `string` вы можете посмотреть здесь: <https://tkr-sh.github.io/lua-std-doc/lua_std/struct.string.html>
