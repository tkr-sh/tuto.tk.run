```lua
print("试着猜一个 1 到 100 之间的随机数！")
```

这一行也是对函数的调用！我们可以看出来，因为这里有名字 `print`，而且名字后面跟着括号！

这个函数接收一个字符串作为它第一个也是唯一的参数。但是……我们从未把输出放进变量里。

## 为什么？

因为 `print` 不返回值。`print` 是一个特殊函数，它会把它收到的数据显示给用户。

例如，让我们回到之前的例子：

```lua
the_best_number = 40 + 2
my_grandpa_age = the_best_number * 2
```

我怎么知道 `my_grandpa_age` 的值是多少？我可以在脑子里计算来猜，但当程序变得更复杂时，这并不总是可行。

因此，我们可以直接这样做

```lua
print(my_grandpa_age)
```

这会显示变量 `my_grandpa_age` 的内容。在我们的例子中：`84`

## 既然你现在知道了

既然你已经知道了这个非常有用的函数，那么我向你介绍 _“✨️**Lua 运行器**✨️”_ 和 _“✨️**交互式 Lua 编辑器**✨️”_，你可以在上面运行自己的程序：

<div class="runner">

```lua
print("你好，世界！")
```

</div>

你可以看到，每次点击“Run”时，这个例子都会重新运行

<div class="runner">

```lua
print(random(1, 100))
```

</div>

最后，我们可以确认我们正确知道爷爷的年龄了：

<div class="runner">

```lua
the_best_number = 40 + 2
print(the_best_number)
my_grandpa_age = the_best_number * 2
print(my_grandpa_age)
```

</div>
