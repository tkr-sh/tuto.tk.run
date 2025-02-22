For the first time, we won't be focusing at a single line! The current line is just

```lua
repeat
```

but in itself, it doesn't do anything: it goes with `until`.

```lua
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
```

But first of all... what are `repeat` and `until` and why are their so special ? Aren't they just some weird variables ?

You might remember the definition of a variable

> `the_name_of_the_variable` can be anything. Excepted `true`, `false`, `nil` (and other that we'll see later)

And the later... is now!

## Keywords

`repeat` and `until` are what we call "keywords". Keywords are special name, that instead of having a value (like variables) or doing some computation (like functions), are instructions that tells Lua to take certain decisions.

In our example of `repeat`/`until`, everything that is between `repeat` and `until` will be repeated until a certain condition is met. When the execution arrives to `until`, it checks if it wants to rerun, and if it wants, the code goes back to `repeat`, and repeats.

This kind of behaviour - to repeat some code until _something_ - is called a "loop" and it's a really common concept in programming.

Most of the time, you want to exit your loop at a certain point...

### But how to know when to exit ?

When using loops like `until`, you must provide a `boolean` (`true` or `false`) to it, to tell it if you want to continue looping.

And most of the time, you do that with relational operators:

- `==`: Check if two values are equals. **NOT TO BE CONFUSED WITH 1 `=`**. 1 `=` is to assign a value, 2 `=` (`==`) is to check that 2 values are equals. If they are the same the value is `true` else `false`
- `~=`: Check that two values are different. If they are different the value is `true` else `false`
- `>`: Check that the left value is greater than the right one. If it is the value is `true` else `false`
- `<`: Check that the left value is less than the right one. If it is the value is `true` else `false`
- `>=`: Check that the left value is greater than **OR EQUAL** to the right one. If it is the value is `true` else `false`
- `<=`: Check that the left value is less than **OR EQUAL** to the right one. If it is the value is `true` else `false`


<div class="runner">

```lua
print(1984 == 2025)
print(1984 ~= 2025)
print(1984 < 2025)
print(2025 <= 2025)
print(2025 < 2025)
print(1984 > 2025)
```

</div>

### Kind of loops

- `repeat`/`until`: will therefore repeat some code until a certain condition is met, and stop once this condition is met

<div class="runner">

```lua
age = 0
repeat
    print(age)
    age = age + 1
until age == 4
```
</div>

- `while`: While loops are very similar to `repeat`/`until` loop, but it's in the other way around! You loop while the condition is met! After the condition, you use the `do` keyword to start executing the code, and you loop back to the start when you see the `end` keyword. In most situation where you can use `repeat` you can also use `while`. It's mostly a personal taste.

<div class="runner">

```lua
age = 0
while age ~= 4 do
    print(age)
    age = age + 1
end
```
</div>

- `for`: The last loop is the `for` loop and it's kind of diferent, because you already know how many times you want to iterate. And you define from which number to which number you want to iterate. At the end of an iteration, it will automatically increment `age` by `1`!

<div class="runner">

```lua
for age = 0, 3 do
    print(age)
end
```
</div>


## Back to our case

In our case, we want to keep asking the user for a number until it founds the correct one. Therefore we 
```lua
repeat
    ...
until user_guess == number_to_guess
```
