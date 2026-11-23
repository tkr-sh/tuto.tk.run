What if now, we could have a history of our scores, until we finish playing? A history of everything that occurred? How would we do that?

Your first approach might be to define multiple variables like so:

```lua 
score_first_game = nil
score_second_game = nil
score_third_game = nil
...
```

But this is not a correct approach. This is not sustainable if the user plays a lot of times. This is hard to really link cleanly with a loop.

To resolve this, we will need a new data type: `table`.

## What is a table?

A `table` is a Lua type that allows us to store some arbitrarily long or short amount of data. Each Lua table assigns a key to a value in the format `[key] = value`, like so:

```lua
my_table = {
    ["a"] = 7,
    ["b"] = 8,
    ["c"] = 4,
}
```

We can access the value of a table by referencing the key and doing `my_table[my_key]`. For example:

<div class="runner">

```lua
my_table = { ["a"] = 7, ["b"] = 8, ["c"] = 4 }

print(my_table["b"])
```

</div>

And we can simply add a new value to our table by assigning a non-existent key to a value like so:

<div class="runner">

```lua
my_table = { ["a"] = 7, ["b"] = 8, ["c"] = 4 }

my_table["d"] = 5
print(my_table["d"])
```

</div>

You can also write no key, and in this case, the keys will start from one and increase afterwards:


<div class="runner">

```lua
my_table = { "a", "b", "c", "z" }
-- Same as
-- my_table = { [1] = "a", [2] = "b", [3] = "c", [4] = "z" }

print(my_table[2])
print(my_table[4])
```

</div>
