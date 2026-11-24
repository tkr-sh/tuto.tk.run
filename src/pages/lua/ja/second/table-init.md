ここで、遊び終えるまでのスコアの履歴を持てたらどうでしょう？ 起きたことすべての履歴を？ どうやって実現すればいいでしょうか？

最初に思いつくのは、次のように複数の変数を定義することかもしれません。

```lua 
score_first_game = nil
score_second_game = nil
score_third_game = nil
...
```

しかし、これは正しい方法ではありません。ユーザーが何度も遊ぶと持続できません。また、ループときれいに関連付けるのが難しくなります。

これを解決するには、新しいデータ型 `table` が必要です。

## テーブルとは？

`table` は Lua の型で、任意の長さのデータを保存できます。各 Lua テーブルは、次のように `[key] = value` の形式で値にキーを割り当てます。

```lua
my_table = {
    ["a"] = 7,
    ["b"] = 8,
    ["c"] = 4,
}
```

テーブルの値にアクセスするには、キーを参照して `my_table[my_key]` と書きます。たとえば：

<div class="runner">

```lua
my_table = { ["a"] = 7, ["b"] = 8, ["c"] = 4 }

print(my_table["b"])
```

</div>

また、存在しないキーに値を代入することで、簡単にテーブルに新しい値を追加できます。

<div class="runner">

```lua
my_table = { ["a"] = 7, ["b"] = 8, ["c"] = 4 }

my_table["d"] = 5
print(my_table["d"])
```

</div>

キーを書かないこともできます。その場合、キーは 1 から始まり、その後増えていきます。


<div class="runner">

```lua
my_table = { "a", "b", "c", "z" }
-- 次と同じ
-- my_table = { [1] = "a", [2] = "b", [3] = "c", [4] = "z" }

print(my_table[2])
print(my_table[4])
```

</div>
