```lua
number_of_tries = 0
```

这是我们在程序中看到的第一行。

通常，当你想理解一个程序在做什么时，你会像阅读普通英文文本一样逐行阅读，所以我们从这里开始。


## 好酷，但这一行在做什么？

在编程语言中，你经常会遇到“变量”这个概念。变量就像一个贴了标签的盒子，里面存放着一些数据。创建变量后，你可以读取它的内容——相当于打开盒子——甚至可以修改它（正如“变量”这个名字所示）——相当于往盒子里放入一个新值。

### 但你可能会问，是哪些数据？

在 Lua 中，目前你需要了解 6[^1] 种数据类型：
- `number`：就像数学中的数字！你可以直接写出一个数字来声明它：`123`。<br/> **注意**：对于带小数的数字（例如 `0.5`），请使用 `.`（点）而不是 `,`（逗号）
- `boolean`：要么是 `true`，要么是 `false`。（例如“2 等于 3 吗？”=> `false`）。
- `nil`：空。`nil` 的作用是表示你没有值。
- `string`：一组字母、数字或符号（例如 `你好！`）。要声明字符串，你必须在内容两边加上引号（`'` 或 `"`）（例如要得到字符串 `你好！`，你应该写 `"你好！"`）
- `table`：我们稍后会看到！
- `function`：我们稍后会看到！

当你想把一些数据赋给变量时，只需写：
```lua
the_name_of_the_variable = the_value
```
`the_name_of_the_variable` 可以是任何东西，但不能是 `true`、`false`、`nil`（以及我们稍后会看到的其他内容）

`the_value` 是上面描述的有效值。

以下是一些例子：
```lua
a_negative_number = -1
a_big_number = 100000000
grandpa_name = "约翰"
no_value = nil
is_it_raining = false
```

你也可以把一个变量的内容赋给另一个变量：

```lua
grandpa_name = "约翰"
my_grandpa_friend_name = grandpa_name
```

在这种情况下，`grandpa_name` 和 `my_grandpa_friend_name` 都会是字符串 `约翰`。

## 回到我们的第一行

```lua
number_of_tries = 0
```

所以，这一行只是把数字 `0` 放进变量 `number_of_tries` 中。这其实很合理！因为在程序开始时，用户还没有尝试猜数字！而且正如 `number_of_tries` 这个名字所示，这个变量以后会保存用户尝试的次数！



[^1]: 

<div class="for-technical">

其实有 8 种类型，但目前只有 6 种是重要的 :)。<https://www.lua.org/pil/2.html>

</div>
