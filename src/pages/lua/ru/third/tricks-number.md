## Больше операторов

Вы уже знаете `+`, `-`, `*` и `/`. У Lua есть три других оператора для `number`:

|Оператор|Что он делает|
|-|-|
|`//`|Делит и отбрасывает дробную часть|
|`%`|Даёт остаток от деления|
|`^`|Возводит в степень|

<div class="runner">

```lua
print(7 / 2)
print(7 // 2)
print(7 % 2)
print(2 ^ 10)
```

</div>

Оператор `%` очень полезен, чтобы узнать, чётное `number` или нечётное: `number` чётное, когда остаток от его деления на `2` равен `0`.

<div class="runner">

```lua
print(10 % 2 == 0)
print(7 % 2 == 0)
```

</div>

## Целые `number` и десятичные `number`

В Lua есть два вида `number`: целые `number`, такие как `5`, и десятичные `number`, такие как `5.0` или `3.14`. В большинстве случаев вам не нужно об этом думать, но это объясняет, почему некоторые результаты выводятся с `.0`:

<div class="runner">

```lua
print(10 / 2)
print(10 // 2)
print(5 == 5.0)
```

</div>

Оператор `/` всегда даёт десятичное `number`, даже когда результат круглый. `5` и `5.0` при этом равны.

<div class="note">

Десятичные `number` не всегда точны! Компьютеры не могут идеально хранить такое `number`, как `0.1`, поэтому некоторые сравнения дают неожиданные результаты:

<div class="runner">

```lua
print(0.1 + 0.2 == 0.3)
```
</div>

Избегайте использования `==` для результата вычисления с десятичными `number`.

</div>

## Другие способы записи `number`

Очень большие или очень маленькие `number` можно записывать с помощью `e`, что означает «умножить на 10 в степени». Вы также можете записывать `number` в [шестнадцатеричной системе](https://en.wikipedia.org/wiki/Hexadecimal), начиная с `0x`:

<div class="runner">

```lua
print(1e3)
print(2.5e-2)
print(0xFF)
```

</div>

## Преобразование `number` и строк

Функция `tostring` преобразует `number` в `string`, а `tonumber`, который мы уже использовали, делает наоборот:

<div class="runner">

```lua
print(tostring(42) .. "!")
print(tonumber("42") + 1)
```

</div>

Lua также сам выполняет преобразование, когда может: `string`, которая выглядит как `number`, становится `number` в вычислении, а `number` становится `string` с помощью `..`.

<div class="runner">

```lua
print("10" + 5)
print(10 .. 20)
```

</div>

## `math.floor` и `math.ceil`
Функция `math.floor` округляет `number` вниз, а `math.ceil` — вверх:

<div class="runner">

```lua
print(math.floor(3.7))
print(math.ceil(3.2))
```

</div>

Чтобы округлить до ближайшего целого `number`, прибавьте `0.5` перед использованием `math.floor`:

<div class="runner">

```lua
print(math.floor(3.7 + 0.5))
print(math.floor(3.2 + 0.5))
```

</div>

## `math.max` и `math.min`
Функция `math.max` даёт наибольшее из `number`, которые вы ей передаёте, а `math.min` — наименьшее:

<div class="runner">

```lua
print(math.max(4, 9, 2))
print(math.min(4, 9, 2))
```

</div>

## `math.abs`
Функция `math.abs` убирает знак у `number`. Это полезно, чтобы получить расстояние между двумя `number`:

<div class="runner">

```lua
print(math.abs(-5))
print(math.abs(3 - 10))
```

</div>

## `math.sqrt`
Функция `math.sqrt` даёт квадратный корень из `number`:

<div class="runner">

```lua
print(math.sqrt(16))
```

</div>

---

Все функции `math` вы можете посмотреть здесь: <https://tkr-sh.github.io/lua-std-doc/lua_std/math/index.html>
