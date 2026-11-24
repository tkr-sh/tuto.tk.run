现在我们知道了如何

1. 生成一个随机数字
2. 获取用户输入
3. 循环

我们需要帮助用户，告诉他们猜的数字比秘密数字更小还是更大。

为此，我们会使用一个新关键字，你以后会经常用到它：`if`

`if` 所做的，基本上就是根据条件执行代码。循环检查条件是为了重复执行，而 `if` 检查条件只是为了执行一次。

例如，我们可以检查用户输入的是不是 `42`，如果是，就用一条特别的消息问候他/她。
<div class="runner">

```lua
good_number = 42

if good_number == tonumber(read()) then
    print("好选择！")
end
```

</div>

所以在我们的游戏中

```lua
if number_to_guess < user_guess then
    print("这个数字更小！")
end

if number_to_guess > user_guess then
    print("这个数字更大！")
end
```

我们检查数字是更小还是更大，并据此打印一条消息来帮助用户。

如果用户找到了正确的数字，循环会因为下面这行而停止：

```lua
until user_guess == number_to_guess
```

<div class="runner">

```lua
number_to_guess = 42
user_guess = tonumber(read())

if number_to_guess < user_guess then
    print("这个数字更小！")
end

if number_to_guess > user_guess then
    print("这个数字更大！")
end

if number_to_guess == user_guess then
    print("正确！")
end
```

</div>

完成这些之后，我们只需要做一个好的结尾……
