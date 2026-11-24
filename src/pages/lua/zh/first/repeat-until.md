这是第一次我们不会只关注单独一行！当前这一行只是

```lua
repeat
```

但它本身并不做任何事情：它要和 `until` 一起使用。

```lua
repeat
	user_guess = tonumber(read())
	number_of_tries = number_of_tries + 1

	if number_to_guess < user_guess then
		print("这个数字更小！")
	end

	if number_to_guess > user_guess then
		print("这个数字更大！")
	end
until user_guess == number_to_guess
```

但首先……`repeat` 和 `until` 是什么，它们为什么这么特别？它们难道不是一些奇怪的变量吗？

你可能还记得变量的定义

> `the_name_of_the_variable` 可以是任何东西，但不能是 `true`、`false`、`nil`（以及我们稍后会看到的其他内容）

而“稍后”……就是现在！

## 关键字

`repeat` 和 `until` 就是我们所说的“关键字”。关键字是一些特殊名字，它们不是像变量那样拥有值，也不是像函数那样进行计算，而是告诉 Lua 做出某些决定的指令。

在我们的 `repeat`/`until` 例子中，`repeat` 和 `until` 之间的所有内容都会被重复执行，直到满足某个条件。当执行到 `until` 时，它会检查是否需要重新运行；如果需要，代码就会回到 `repeat`，并重复执行。

这种重复执行某些代码直到 _某件事_ 发生的行为叫做“循环”，它是编程中非常常见的概念。

大多数时候，你会希望在某个时刻退出循环……

### 但怎么知道什么时候退出？

使用像 `until` 这样的循环时，你必须给它一个 `boolean`（`true` 或 `false`），告诉它什么时候停止循环。

而大多数时候，你会用关系运算符来做到这一点：

- `==`：检查两个值是否相等。**不要和 1 个 `=` 混淆**。1 个 `=` 是用来赋值的，2 个 `=`（`==`）是用来检查两个值是否相等的。如果它们相同，值就是 `true`，否则是 `false`
- `~=`：检查两个值是否不同。如果它们不同，值就是 `true`，否则是 `false`
- `>`：检查左边的值是否大于右边的值。如果是，值就是 `true`，否则是 `false`
- `<`：检查左边的值是否小于右边的值。如果是，值就是 `true`，否则是 `false`
- `>=`：检查左边的值是否大于**或等于**右边的值。如果是，值就是 `true`，否则是 `false`
- `<=`：检查左边的值是否小于**或等于**右边的值。如果是，值就是 `true`，否则是 `false`


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

### 循环的种类

- `repeat`/`until`：会重复执行一些代码，直到满足某个条件，一旦满足就停止

<div class="runner">

```lua
age = 0
repeat
    print(age)
    age = age + 1
until age == 4
```
</div>

- `while`：while 循环和 `repeat`/`until` 循环非常相似，但方向相反！当条件满足时你就循环！在条件之后，你使用 `do` 关键字开始执行代码，当你看到 `end` 关键字时，就会回到开头继续循环。在大多数可以使用 `repeat` 的场景中，你也可以使用 `while`。这主要取决于个人喜好。

<div class="runner">

```lua
age = 0
while age ~= 4 do
    print(age)
    age = age + 1
end
```
</div>

- `for`：最后一种循环是 `for` 循环，它有些不同，因为你已经知道要迭代多少次。你定义从哪个数字迭代到哪个数字。一次迭代结束时，它会自动把 `age` 增加 `1`！

<div class="runner">

```lua
for age = 0, 3 do
    print(age)
end
```
</div>


## 回到我们的情况

在我们的情况下，我们想一直向用户询问数字，直到找到正确的那个。因此我们
```lua
repeat
    ...
until user_guess == number_to_guess
```
