处理表的另一个关键操作是遍历它们。假设在我们的游戏中有一个 `scores` 表（每个 `score` 是用户找到数字所用的 `number_of_tries`）。你如何显示每局游戏的每个成绩？

## `for index`
首先，你可能还记得我们之前简单讨论过的 `for` 关键字：

<blockquote>

`for`：最后一种循环是 `for` 循环，它有些不同，因为你已经知道要迭代多少次。你定义从哪个数字迭代到哪个数字。一次迭代结束时，它会自动把 `age` 增加 `1`！

<div class="runner">

```lua
for age = 0, 3 do
    print(age)
end
```
</div>

</blockquote>

如果我们把这个和我们新学到的 `#` 运算符联系起来，就可以成功地遍历一个表！

<div class="runner">

```lua
scores = {
    [1] = 7,
    [2] = 8,
    [3] = 4,
}

for key = 1, #scores do
    print(key .. ": " .. scores[key])
end
```
</div>

这非常好用！但总是写 `scores[key]` 来访问值可能有些冗余，尤其是当你并不关心键，或者不打算再使用这个表时。幸运的是，Lua 还有另一种处理方式。

## `ipairs`

`ipairs` 是一个函数，我们会把它和 `for` 关键字一起使用，从而无需引用表名就能快速访问键和值！它的语法与之前的 `for` 循环略有不同，尽管它仍然使用 `for` 关键字：

```lua
for my_key, my_value in ipairs(my_table) do 
    -- 做点什么！
end
```

在这个例子中，我们假设有一个名为 `my_table` 的 `table`。`for` 和 `end` 之间的每一行代码都会执行，其中 `my_key` 和 `my_value` 分别是 `my_table` 的键和对应的值。

和之前的 `for` 循环一样，它是按升序排列的。也就是说，它从 `1` 开始，每次迭代增加 `1`。但是，当键是 `string` 或者数字之间有缺口时，它就不起作用了。如果是这种情况，我们应该使用 `pairs`。

## `pairs`

`pairs` 和 `ipairs` 类似，区别在于它可以处理不是从 `1` 开始连续整数的键。缺点是，键出现的顺序是不可靠的：

<div class="runner">

```lua
my_table = {
    [3] = 7,
    ["name"] = 8,
    [false] = 4,
}

for key, value in pairs(my_table) do
    print(key)
    print(value)
end
```

</div>

键/值打印的顺序是不可靠的，每次运行可能都不一样。不过，它的优点是允许我们遍历每一个表！

## 回到我们的游戏
在我们的情况下，我们只想记录每局游戏的 `number_of_tries`。因此，不需要自定义键，我们只需让索引从 `1` 开始，每次增加 `1`，对应游戏局数即可！
