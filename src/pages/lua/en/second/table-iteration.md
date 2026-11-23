Another key to working with tables is iterating through them. Let's say that we have in our game, the table `scores` (a `score` being the `number_of_tries` a user took to find the number). How do you show each score for each game?

## `for index`
First, you might remember the `for` keyword that we quickly talked about in the past:

<blockquote>

`for`: The last loop is the `for` loop and it's kind of different, because you already know how many times you want to iterate. And you define from which number to which number you want to iterate. At the end of an iteration, it will automatically increment `age` by `1`!

<div class="runner">

```lua
for age = 0, 3 do
    print(age)
end
```
</div>

</blockquote>

If we link this, with our new knowledge of the `#` operator, we can therefore iterate successfully through a table!

<div class="runner">

```lua
scores = {
    [1] = 7,
    [2] = 8,
    [3] = 4,
}

for key = 1, #scores do
    print(key .. ": " .. scores[key])
end
```
</div>

This works great! But it can be a bit redundant to always write `scores[key]` to access the value, especially if you don't care about the key, or if you don't plan to use the table anymore. Luckily Lua has another way to handle this.

## `ipairs`

`ipairs` is a function that we will use with the `for` keyword to be able to quickly access key and value without having to reference the table name! It's syntax is a bit different than the previous `for` loop even tho it still uses the `for` keyword:

```lua
for my_key, my_value in ipairs(my_table) do 
    -- Do something!
end
```

In this example, we suppose that we have a `table` called `my_table`. Every line of code that is between the `for` and the `end` will execute with `my_key` and `my_value` being the key and the associated value of `my_table`. 

Like the previous `for` loop, it is sorted in ascending order. Meaning that it starts at `1`, and increases by `1` at every iteration. But, it doesn't work when keys are `string`s or if there are gaps between numbers. If that's the case, we should use `pairs`.

## `pairs`

`pairs` is like `ipairs`, with the difference that it works with keys that are not consecutive integers starting from `1`. The drawback is that, the order in which the keys come is unreliable:

<div class="runner">

```lua
my_table = {
    [3] = 7,
    ["name"] = 8,
    [false] = 4,
}

for key, value in pairs(my_table) do
    print(key)
    print(value)
end
```

</div>

The order in which keys/values are printed is unreliable and might vary from one run to the next. Though, it has the advantage of allowing us to iterate through every table!

## Back to our game
In our case, we just want to keep the `number_of_tries` for every game. Therefore, no need for custom keys, we can just have indexes starting from `1`, and increasing by `1` every time, corresponding to the game number!
