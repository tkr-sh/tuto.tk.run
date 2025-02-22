Now that we know how to

1. generate a random number
2. get user input
3. loop

We need to help the user by telling it if he/she's lower or above the correct number.

For that, we'll use a new keyword, that you'll often find yourself using: `if`

What `if` is doing, is basically executing code depending on condition. Loops check for condition to loop, and `if` check for condition just to run once.

So for example, we can check if the user inputs `42`, and if so, we can greet him/her with a special message.
<div class="runner">

```lua
good_number = 42

if good_number == tonumber(read()) then
    print("Good choice!")
end
```

</div>

So in our game

```lua
if number_to_guess < user_guess then
    print("The number is lower!")
end

if number_to_guess > user_guess then
    print("The number is higher!")
end
```

we check if the number is lower/higher, and depending on that, we print a message to help the user.

If the user found the correct number it will stop the loop because of:

```lua
until user_guess == number_to_guess
```

<div class="runner">

```lua
number_to_guess = 42
user_guess = tonumber(read())

if number_to_guess < user_guess then
    print("The number is lower!")
end

if number_to_guess > user_guess then
    print("The number is higher!")
end

if number_to_guess == user_guess then
    print("Correct!")
end
```

</div>

And with that done, we just have to make a good ending...
