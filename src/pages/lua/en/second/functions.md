You might have noticed that we do a similar block of code twice for the validation input:

```lua
repeat
    print("Try to guess a number:")
    user_guess = tonumber(io.read())
    if user_guess == nil then 
        print("This is not a valid number!")
    end
until user_guess ~= nil
```

```lua
repeat 
    print("Do you want to continue? yes / no")
    user_answer = string.lower(io.read())
    if user_answer ~= "yes" and user_answer ~= "no" then
        print("This is not a valid answer.")
    end
until user_answer == "yes" or user_answer == "no"
```

There is a clear common duplicate pattern that I will try to illustrate with a pseudo/invalid syntax delimited with `<` and `>`.

```lua
repeat 
    print(<question>)
    user_xxx = <transform>(io.read())
    if <not condition> then
        print(<error_message>)
    end
until <condition>
```

## Basics of a function

When you write a program, there is often a rule to follow that is to write things in a modular way, so that it can be useful in the future and generic enough to handle the most cases, and this is exactly what functions are helping us to achieve.

A function is basically a set of instructions that a program will do. A function can take parameters and can return a value. Let's take this function for example:

```lua
function double(my_number)
    return my_number * 2
end
```

The name of this function is `double` and it takes a single parameter called `my_number`. The result of this function, what it returns is a single value: `my_number * 2`. You can call a function by specifying the name and then in parenthesis, separated with `,`, the arguments like so `my_function(value1, value2)`. In our case:

<div class="runner">

```lua
function double(my_number)
    return my_number * 2
end

print(double(42))
```

</div>

## Back to our game!

So, let's optimize our input handling! The first thing that we should look at is what does our function depend on? This is necessary to know which arguments it needs. In our case:
* Prompt message, the message saying to the user to enter a value
* Error message, the message to show when the input is not valid
* Transform, a transformation that we apply to the string we read from the user (`tonumber`, `string.lower`, `string.upper`, ...)
* Condition, a check to know if the input fits what we expect after applying transformation.

The `prompt_message` and `error_message` are kind of easy, they are just strings.<br/>

`transform` and `condition` are a bit more complicated.<br>
`transform` takes the user input and sends back a modified version of it. This is clearly the case of a function: something that takes something in input and send backs something.<br/>
`condition` is also really similar: it takes in argument the result of `transform` and returns a boolean (`true`/`false`) depending on the result. This is clearly again a case where a `function` is needed.

```lua
function validated_read(prompt_message, error_message, transform, condition)
    repeat 
        print(prompt_message)
        user_input = transform(io.read())
        is_valid = condition(user_input)

        if not is_valid then
            print(error_message)
        end
    until is_valid

    return user_input
end
```

Notice the `not`! The `not` keyword flips a boolean: `true` => `false`, `false` => `true`.

In our case we can just write this for the input handling of getting the number from the user:

```lua
function is_not_nil(arg) 
    return arg ~= nil
end

validated_read(
    "Try to guess a number:",
    "This is not a valid number",
    tonumber,
    is_not_nil
)
```

This will produce the same thing as:

```lua
repeat
    print("Try to guess a number:")
    user_guess = tonumber(io.read())
    is_valid = user_guess ~= nil
    if not is_valid then 
        print("This is not a valid number!")
    end
until is_valid
```

## Inlining functions

Notice that we defined `is_not_nil` only to use it once. Instead of declaring a function with a name when we don't need it, Lua allows us to just remove the name when we don't want to use it elsewhere, like so:

```lua
local double = function(number)
    return number * 2
end
```

In our case, we can use it to do:

```lua
validated_read(
    "Try to guess a number:",
    "This is not a valid number",
    tonumber,
    function(nb) return nb ~= nil end
)
```

Which gives us our final program:

<div class="runner">

```lua
function validated_read(prompt_message, error_message, transform, condition)
    repeat 
        print(prompt_message)
        user_input = transform(io.read())
        is_valid = condition(user_input)

        if not is_valid then
            print(error_message)
        end
    until is_valid

    return user_input
end

repeat 
	number_of_tries = 0
	number_to_guess = math.random(1, 100)

	print("Try to guess the random number between 1 and 100!")

	repeat
        user_guess = validated_read(
            "Enter a number between 1 and 100:",
            "This is not a valid number",
            tonumber,
            function(nb) return nb ~= nil end
        )
		number_of_tries = number_of_tries + 1

		if number_to_guess < user_guess then
			print("The number is lower!")
		end

		if number_to_guess > user_guess then
			print("The number is higher!")
		end
	until user_guess == number_to_guess

	print("You won in " .. number_of_tries .. " tries!")

    user_answer = validated_read(
        "Do you want to continue? yes / no",
        "This is not a valid answer.",
        string.lower,
        function(response) return response == "yes" or response == "no" end
    )
until user_answer == "no"

print("Goodbye!")
```

</div>
