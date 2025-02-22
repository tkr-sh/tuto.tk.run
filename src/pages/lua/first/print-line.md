```lua
print("Try to guess the random number between 1 and 100!")
```

This line is also a call to a function! We can see it since we have the name `print` but with parenthesis after it's name!

This function takes a string as its first and only parameter. But... we never put the output into a variable.

## Why ?

Because `print` doesn't return a value. `print` is a special function that display to the user the data that it receives in it's input.

For example, let's take back our previous example:

```lua
the_best_number = 40 + 2
my_grandpa_age = the_best_number * 2
```

How do I know what is the value of `my_grandpa_age` ? I could guess by doing the calculation in my head, but when the program becomes more complex, this isn't always possible.

Therefore, we can just do 

```lua
print(my_grandpa_age)
```

Which will display the content of the variable `my_grandpa_age`. In our case: `84`

## Now that you know that

Now that you know this very useful function, I introduce to you the _"✨️**Lua runner**✨️"_ & _"✨️**Interactive Lua editor**✨️"_, on which you can run your own programs:

<div class="runner">

```lua
print("Hello world!")
```

</div>

You can see that this is re-run everytime you click "Run" with this example

<div class="runner">

```lua
print(random(1, 100))
```

</div>

And finally, we can be sure that we know the age of our grandpa correctly:

<div class="runner">

```lua
the_best_number = 40 + 2
print(the_best_number)
my_grandpa_age = the_best_number * 2
print(my_grandpa_age)
```

</div>



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
