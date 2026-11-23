A useful piece of information that we need while working with tables is "how many elements does it have?" For example if I want to get the last element!

This is made possible with a special operator: `#`. By writing `#` and then the name of table or the table directly, you can have the total size of the table:


<div class="runner">

```lua
my_table = { "a", "b", "c" }

print(#my_table)
my_table[4] = "d"
print(#my_table)
```

</div>

This is particularly useful if I want to know the element of a table with the "last" index, or if I want to add a new element at the end: `my_table[#my_table]` is the value that has its key being the total table length, meaning the last in most cases!

<div class="runner">

```lua
my_table = { "a", "b", "c" }

print(my_table[#my_table])
my_table[#my_table + 1] = "d"
print(my_table[#my_table])
```

</div>


### Edge cases

The `#` operator is only reliable when the keys of a table are the whole numbers `1, 2, 3, ...` with no gaps starting from `1`. Keys that are strings or booleans are not counted, and if a number is missing in the middle, the result is unpredictable:

<div class="runner">

```lua
-- Reliable: the keys are 1, 2, 3
print(#{ "a", "b", "c" }) -- 3
print(#{ [1] = "a", [2] = "b", [3] = "c" }) -- 3

-- Not counted: the keys are strings
print(#{ ["a"] = 1, ["b"] = 2, ["c"] = 3 }) -- 0

-- Unreliable: there is a gap at index 2
print(#{ [1] = "a", [3] = "b", [4] = "c" }) -- 1 or 4, don't rely on it
```

</div>
