## Inline functions

Until now, we wrote functions like this:

```lua
function double(number)
    return number * 2
end
```

This is sugar. What Lua really does is create a function without a name and store it in a variable called `double`:

```lua
double = function(number)
    return number * 2
end
```

Both versions are exactly the same. This is also why we could give `tonumber` to `validated_read`: a function is just a value stored in a variable, like a number or a string.


## Calling functions with `:`

To put a string in lowercase, we wrote `string.lower(user_answer)`.
But, when you do use a function from the `string` or `table` library, that takes as first argument a `string` or `table`, you can write this with the following syntax: `user_answer:lower()`:

<div class="runner">

```lua
local answer = "YES"

print(string.lower(answer))
print(answer:lower())
```

</div>

The `:` takes the value on its left and gives it to the function as its first argument. For a string, Lua looks for the function in the `string` table, so `answer:lower()` becomes `string.lower(answer)`.

<div class="note">

This doesn't work on a string written directly in the code: `"YES":lower()` is a syntax error, because Lua only accepts `:` after a variable name or after parentheses. To make it work, put the string between parentheses:

```lua
print(("YES"):lower())
-- This errors:
print("YES":lower())
```

</div>

## Without parentheses

When a function is called with a single argument, and this argument is a `string` or a `table` written directly in the code, the parentheses are optional:

<div class="runner">

```lua
print "Hello!"
print("Hello!")
```

</div>

You will see this in other people's code. In this course we keep the parentheses, because they make it obvious that a function is being called.

## Returning several values

A function can return more than one value. You separate them with `,` after the `return`, and you receive them in several variables, also separated with `,`:

<div class="runner">

```lua
function first_and_last(list)
    return list[1], list[#list]
end

local first, last = first_and_last({ "a", "b", "c", "d" })

print(first)
print(last)
```

</div>

If you give fewer variables than there are values, the extra values are simply dropped. If you give more, the extra variables are `nil`:

<div class="runner">

```lua
function first_and_last(list)
    return list[1], list[#list]
end

local only_first = first_and_last({ "a", "b", "c", "d" })
local first, last, other = first_and_last({ "a", "b", "c", "d" })

print(only_first)
print(first, last, other)
```

</div>

<div class="note">

To keep only the first value of a `function`, put the call between parentheses:

<div class="runner">

```lua
function first_and_last(list)
    return list[1], list[#list]
end

print( first_and_last({ "a", "b", "c", "d" }) )
print((first_and_last({ "a", "b", "c", "d" })))
```
</div>

</div>

## Missing and extra arguments

Lua doesn't check the number of arguments that you give to a `function`. An argument that is missing is `nil`, and an argument that is not expected is ignored:

<div class="runner">

```lua
function show(a, b)
    print(a, b)
end

show(1, 2)
show(1)
show(1, 2, 3)
```

</div>

## Optional arguments

Since a missing argument is `nil`, we can check for it and use a default value instead. This makes the argument optional:

<div class="runner">

```lua
function greet(name, greeting)
    if greeting == nil then
        greeting = "Hello"
    end

    print(greeting .. ", " .. name .. "!")
end

greet("Bob")
greet("Bob", "Good evening")
```

