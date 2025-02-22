```lua
number_of_tries = 0
```

This is the first line that we see in the program.

Usually, when you want to understand what a program is doing, you'll read it line by line, like you would read a normal english text.


## Ok cool, but what is this line doing ?

In programming languages you often have the concept of a "variable". A variable is like a labeled box that holds some data. Once you create a variable, you can read the content that it has - equivalent to opening a box - and you can even change it (since the name "variable") - equivalent to putting a new value in the box.

### But which data you may ask ?

In Lua there are 6[^1] types of data:
- `number`: A number like in mathematics! You can declare a number by just writing it: `123`.<br/> **Note**: for numbers with decimals (e.g. `0.5`), use `.` (dot) and not `,` (comma)
- `boolean`: Either `true` or `false`. (e.g. "Is 2 equals to 3 ?" => `false`).
- `nil`: Empty. The goal of `nil` is to say that you don't have a value.
- `string`: A group of letters, numbers, or symbols (e.g. `Hello!`). To declare a string, you must put quotes (`'` or `"`) around the content (e.g. to have the string `Hello!` you should write `"Hello!"`)
- `table`: We'll see it later!
- `function`: We'll see it later!

When you want to assign some data to a variable, you just have to write:
```lua
the_name_of_the_variable = the_value
```
`the_name_of_the_variable` can be anything. Excepted `true`, `false`, `nil` (and other that we'll see later)

`the_value` a valid type as described above.

Here are some examples:
```lua
a_negative_number = -1
a_big_number = 100000000
grandpa_name = "John"
no_value = nil
is_it_raining = false
```

You can also assign the content of a variable to another variable:

```lua
grandpa_name = "John"
my_grandpa_friend_name = grandpa_name
```

In this context, both `grandpa_name` and `my_grandpa_friend_name` will be the string `John`.

## Back to our first line

```lua
number_of_tries = 0
```

So, this line is just putting the number `0` into the variable `number_of_tries`. And this actually make sens! Because at the start of the program, the user hasn't tried to guess the number! And as the name `number_of_tries` indicates, this variable will contains in the future the number of tries that the user  made!



[^1]: Well in reality there are 8 types but for now, only 6 are importants :). <https://www.lua.org/pil/2.html>


<button id="toggle-code" class="lua-button"/>

<div id="hidden-code">

```lua
number_of_tries = 0
number_to_guess = random(1, 100)

print("Try to guess the random number between 1 and 100!")

repeat
	user_guess = tonumber(read())
	number_of_tries = number_of_tries + 1

	if number_to_guess < user_guess then
		print("The number is lower!")
	end

	if number_to_guess > user_guess then
		print("The number is higher!")
	end
until user_guess == number_to_guess

print("You won in " .. number_of_tries .. " tries!")
```

</div>
