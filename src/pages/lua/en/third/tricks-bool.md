## What counts as `true`?

In a condition, Lua doesn't only accept `true` and `false`: any value can be used. The rule is simple: `false` and `nil` count as false, and **everything else** counts as true.

Be careful if you know another programming language: in Lua, `0` and the empty string `""` count as true!

<div class="runner">

```lua
if 0 then
    print("0 counts as true")
end

if "" then
    print("An empty string counts as true")
end

if nil then
    print("This is never printed")
end
```

</div>

## Checking for `nil` without `== nil`

Thanks to this rule, we can check that a variable has a value by using it directly as a condition:

<div class="runner">

```lua
local name = nil

if name then
    print("Hello " .. name)
else
    print("There is no name")
end
```

</div>

`if name then` is a shorter way to write `if name ~= nil then`, and `if not name then` is a shorter way to write `if name == nil then`.

<div class="note">

These are not exactly the same when the variable can contain `false`: `false` is not `nil`, but it also counts as false in a condition.

</div>

## `not`

The `not` keyword always gives a real boolean. It gives `true` for `false` and `nil`, and `false` for everything else:

<div class="runner">

```lua
print(not true)
print(not nil)
print(not 0)
print(not "hello")
```

</div>

## `or` and `and` give back one of their values

We saw `or` and `and` with booleans, but they work with any value, and they don't always give a boolean: they give back one of their two values.

- `a or b` gives `a` if it counts as true, and `b` otherwise.
- `a and b` gives `a` if it counts as false, and `b` otherwise.

<div class="runner">

```lua
print(nil or "default")
print("value" or "default")

print(nil and "next")
print("value" and "next")
```

</div>

With `true` and `false`, this gives exactly the tables that we saw before.

## Default values with `or`

This is why `or` can give a default value to a variable, like we did for optional arguments:

<div class="runner">

```lua
local name = nil
name = name or "Guest"

print(name)
```

</div>

If `name` has a value, it is kept. If `name` is `nil`, it becomes `"Guest"`.

## Choosing between two values

By using `and` and `or` together, we can choose between two values with a condition, in a single line:

<div class="runner">

```lua
local score = 70
local result = score > 50 and "You win!" or "You lose!"

print(result)
```

</div>

This is the same as:

```lua
local result
if score > 50 then
    result = "You win!"
else
    result = "You lose!"
end
```

<div class="note">

This trick doesn't work if the first value is `false` or `nil`: `condition and false or "other"` always gives `"other"`. In this case, use a normal `if`.

</div>

## `and` stops as soon as it can

When the first value of `and` counts as false, Lua doesn't even look at the second one. Remember the error that we had at the beginning of the course when comparing `nil` with a number? `and` lets us avoid it:

<div class="runner">

```lua
local user_guess = nil

if user_guess ~= nil and user_guess > 10 then
    print("The guess is bigger than 10")
end

print("No error!")
```

</div>

Since `user_guess ~= nil` is `false`, `user_guess > 10` is never executed. In the same way, `or` doesn't look at its second value when the first one counts as true.

## `nil` in tables

A key that doesn't exist in a table gives `nil`, without any error. And to remove a key from a table, you assign `nil` to it:

<div class="runner">

```lua
local player = { name = "Bob", score = 42 }

print(player.age)

player.score = nil
print(player.score)
```

</div>

<div class="note">

This is the right way to remove a key like `score`, but not a value in a list: as we saw, it would create a hole. For lists, use `table.remove`.

</div>
