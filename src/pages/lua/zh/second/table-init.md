如果现在我们可以拥有自己的成绩历史记录，直到我们玩完为止呢？记录发生过的一切？我们会怎么做？

你的第一反应可能是定义多个变量，像这样：

```lua 
score_first_game = nil
score_second_game = nil
score_third_game = nil
...
```

但这不是正确的做法。如果用户玩很多次，这就不可持续。而且很难把它和循环干净地联系起来。

为了解决这个问题，我们需要一种新的数据类型：`table`。

## 什么是表？

`table` 是 Lua 的一种类型，允许我们存储任意长短的数据。每个 Lua 表都会以 `[key] = value` 的格式给一个值分配一个键，像这样：

```lua
my_table = {
    ["a"] = 7,
    ["b"] = 8,
    ["c"] = 4,
}
```

我们可以通过引用键来访问表中的值，写法是 `my_table[my_key]`。例如：

<div class="runner">

```lua
my_table = { ["a"] = 7, ["b"] = 8, ["c"] = 4 }

print(my_table["b"])
```

</div>

我们也可以简单地通过给一个不存在的键赋值来向表中添加新值，像这样：

<div class="runner">

```lua
my_table = { ["a"] = 7, ["b"] = 8, ["c"] = 4 }

my_table["d"] = 5
print(my_table["d"])
```

</div>

你也可以不写键，在这种情况下，键会从 1 开始，然后依次递增：


<div class="runner">

```lua
my_table = { "a", "b", "c", "z" }
-- 等同于
-- my_table = { [1] = "a", [2] = "b", [3] = "c", [4] = "z" }

print(my_table[2])
print(my_table[4])
```

</div>
