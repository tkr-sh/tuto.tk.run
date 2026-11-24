## More operators

You already know `+`, `-`, `*` and `/`. Lua has three other operators for `number`s:

|Operator|What it does|
|-|-|
|`//`|Divides and removes the decimals|
|`%`|Gives the remainder of a division|
|`^`|Raises to a power|

<div class="runner">

```lua
print(7 / 2)
print(7 // 2)
print(7 % 2)
print(2 ^ 10)
```

</div>

The `%` operator is very useful to know if a `number` is even or odd: a `number` is even when the remainder of its division by `2` is `0`.

<div class="runner">

```lua
print(10 % 2 == 0)
print(7 % 2 == 0)
```

</div>

## Whole `number`s and decimal `number`s

Lua has two kinds of `number`s: whole `number`s like `5`, and decimal `number`s like `5.0` or `3.14`. Most of the time you don't need to think about it, but it explains why some results are printed with a `.0`:

<div class="runner">

```lua
print(10 / 2)
print(10 // 2)
print(5 == 5.0)
```

</div>

The `/` operator always gives a decimal `number`, even when the result is round. `5` and `5.0` are still equal.

<div class="note">

Decimal `number`s are not always exact! Computers can't store a `number` like `0.1` perfectly, so some comparisons give surprising results:

<div class="runner">

```lua
print(0.1 + 0.2 == 0.3)
```
</div>

Avoid using `==` on the result of a calculation with decimal `number`s.

</div>

## Other ways to write `number`s

Very big or very small `number`s can be written with `e`, which means "times 10 to the power of". You can also write `number`s in [hexadecimal](https://en.wikipedia.org/wiki/Hexadecimal) by starting with `0x`:

<div class="runner">

```lua
print(1e3)
print(2.5e-2)
print(0xFF)
```

</div>

## Converting `number`s and strings

The `tostring` function converts a `number` to a `string`, and `tonumber`, that we already used, does the opposite:

<div class="runner">

```lua
print(tostring(42) .. "!")
print(tonumber("42") + 1)
```

</div>

Lua also does the conversion by itself when it can: a `string` that looks like a `number` becomes a `number` in a calculation, and a `number` becomes a `string` with `..`.

<div class="runner">

```lua
print("10" + 5)
print(10 .. 20)
```

</div>

## `math.floor` and `math.ceil`
The `math.floor` function rounds a `number` down, and `math.ceil` rounds it up:

<div class="runner">

```lua
print(math.floor(3.7))
print(math.ceil(3.2))
```

</div>

To round to the nearest whole `number`, add `0.5` before using `math.floor`:

<div class="runner">

```lua
print(math.floor(3.7 + 0.5))
print(math.floor(3.2 + 0.5))
```

</div>

## `math.max` and `math.min`
The `math.max` function gives the biggest of the `number`s that you give it, and `math.min` the smallest:

<div class="runner">

```lua
print(math.max(4, 9, 2))
print(math.min(4, 9, 2))
```

</div>

## `math.abs`
The `math.abs` function removes the sign of a `number`. This is useful to get the distance between two `number`s:

<div class="runner">

```lua
print(math.abs(-5))
print(math.abs(3 - 10))
```

</div>

## `math.sqrt`
The `math.sqrt` function gives the square root of a `number`:

<div class="runner">

```lua
print(math.sqrt(16))
```

</div>

---

You can check all the `math` functions here: <https://tkr-sh.github.io/lua-std-doc/lua_std/math/index.html>
