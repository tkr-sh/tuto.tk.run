```lua
user_guess = tonumber(read())
```

Ok, so this code might seems weird (a function in a function?), but it can be seen at:

```lua
read_value = read()
user_guess = tonumber(read_value)
```

What happens is the following:
1. Lua wants to compute the right value 
2. Lua wants to execute `tonumber`, but for that, it needs all of it inputs to have values
3. Therefore, when it sees `read()`, it instanly try to run it so the function produces the value
4. After that `read()` is ran and has returned a value, `tonumber` can now be executed!

### Ok, but what is `read()` doing ?

`read()` asks the user for a string and captures it.

**Before running the example:** when you'll run it, it will continuously try to read a `string` from you. So, after you entered your first `input`, you should click on `Cancel` in your browser pop-up or press `Escape`.



<div class="runner">

```lua
print(read())
```

</div>

### And what about `tonumber` ?

Since `read()` gets a `string` from the user, we cannot compare it to our random number, because you cannot compare a `string` that could have non digit characters to a `number` (e.g. `"x4%$A" > 3` doesn't make sense)

Therefore, we need to convert the input of the user to a number and this is exactly what `tonumber` is doing!

This function will return `nil` if it's input is not a valid number.

<div class="runner">

```lua
print(tonumber("Not a number") == nil)
```

</div>

And you cannot add `nil` + `number`, so enter a valid number!

<div class="runner">

```lua
print(tonumber(read()) + 1)
```

</div>

Will read the `string` from the user, convert it to `number` and add one to this number, and if it's not a valid number, `tonumber(read())` will be a `nil` value.
