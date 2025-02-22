```lua
number_to_guess = random(1, 100)
```

Ok so we just saw that you could put some data into a variable... but what is that ???

To understand what this is doing, we need to understand `function`s. This was one of the data type that I didn't explain during the first line, so here it is:

You can see functions to be a sort of machine that takes some input, does something with it, and then gives you an output.

## But what does it do with it ??

Well, in lua there are some operators that allow you to do some special things:

- `+`: Add 2 numbers. (e.g: `1 + 2` => `3`)
- `-`: Substract a number by another one. (e.g: `3 - 2` => `1`)
- `*`: Multiply 2 numbers. (e.g: `3 * 2` => `6`)
- `/`: Divies a number by another one. (e.g: `6 / 2` => `3`)

Therefore, we can do the following things with our variables
```lua
the_best_number = 40 + 2
my_grandpa_age = the_best_number * 2
```

`the_best_number` will be 42 and `my_grandpa_age` will be 84.

## Back to our functions!

For example, a function could take a number and return you the number `+ 1`.

A function, like a variable, can be used when you know it's name. BUT! When you want it to do it's machinery and give you the output, you must put parenthesis after the name (e.g. `my_function()`). This will execute a function called `my_function`. But this function doesn't have any input! To add input to it, you can put the values that you want between the parenthesis and separate them by `,`.

### Example

```lua
day = 4
month = "April"
year = 1984
calculate_tomorrows_date(day, month, year)
```
which is the same as
```lua
calculate_tomorrows_date(4, "April", 1984)
```

## Back to our second line

```lua
number_to_guess = random(1, 100)
```

So, this line is executing the function `random` with as input: `1` and `100`, and put the ouput of the computation into `number_to_guess`. We never defined what `random` did. But, Lua comes with some "built-in"s function to help developers! And in our case, there is a function `random` that wants 2 number in input and that generates a random number between the first number (in our case `1`) and the second number (in our case `2`).

Perfect! We now have generated the number that the user has to guess!
