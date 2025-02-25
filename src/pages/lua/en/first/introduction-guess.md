This is the first program that we will study! Don't worry we will go through it VERY slowly, so it's ok if you don't understand anything yet!

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

What this program does is creating the game of "find the number":

```
Try to guess the random number between 1 and 100!
50
The number is higher
75
The number is lower
62
You won in 3 tries!
```
