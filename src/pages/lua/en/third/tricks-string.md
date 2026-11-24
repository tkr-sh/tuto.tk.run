## Quotes

A string can be written with double quotes `"..."` or single quotes `'...'`. Both give exactly the same string. This is useful when your text contains one of the two:

<div class="runner">

```lua
print("It's a nice day")
print('He said "hello"')
```

</div>

## Multi-line strings

You can't simply add a return to line in the middle of a `"..."` string like so:

<div class="runner">

```lua
print("line 1
line 2")
```
</div>

Lua just doesn't allow to have a return to line like that in a string when writting it with `"` or `'`.

THO! You can do it by replacing `"` or `'` by `[[`/`]]`

<div class="runner">

```lua
print([[line1
line2]])
```

</div>

## Length

The `#` operator that we used on tables also works on strings:

<div class="runner">

```lua
print(#"hello")
print(#"hello world")
```

</div>

## `string.sub`
The `string.sub` function allows to create a new `string`, from a part of this the `string` by specifying, the start and end index:

<div class="runner">

```lua
print(("hello"):sub(2, 4))
-- Same as
-- print(string.sub("hello", 2, 4))
```

</div>

## `string.rep`
The `string.rep` function allows to repeat a `string` multiple times:

<div class="runner">

```lua
print(("hello"):rep(3))
```

</div>

## `string.reverse`
The `string.reverse` function, as it's name apply, reverses a `string`:

<div class="runner">

```lua
print(("hello"):reverse())
```

</div>

## `string.gsub`
The `string.gsub` function replaces a `string` by another, and also prints how many time it has been replaced:

<div class="runner">

```lua
print(("hello"):gsub("lo", "d"))
print(("hello"):gsub("l", "f"))
```

</div>

---

You can check all the `string` functions here: <https://tkr-sh.github.io/lua-std-doc/lua_std/struct.string.html>
