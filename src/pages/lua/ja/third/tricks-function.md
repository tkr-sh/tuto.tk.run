## インライン関数

これまで、関数は次のように書いてきました。

```lua
function double(number)
    return number * 2
end
```

これは糖です。Lua が実際に行っているのは、名前のない関数を作り、それを `double` という変数に保存することです。

```lua
double = function(number)
    return number * 2
end
```

両方のバージョンはまったく同じです。これが、`tonumber` を `validated_read` に渡せた理由でもあります。関数は、数値や文字列と同じように、変数に保存された単なる値なのです。


## `:` で関数を呼び出す

文字列を小文字にするために、私たちは `string.lower(user_answer)` と書きました。
しかし、`string` や `table` ライブラリの関数で、最初の引数として `string` や `table` を受け取るものをを使うときは、次の構文で書けます：`user_answer:lower()`。

<div class="runner">

```lua
local answer = "YES"

print(string.lower(answer))
print(answer:lower())
```

</div>

`:` は左側の値を取り、それを最初の引数として関数に渡します。文字列の場合、Lua は `string` テーブルで関数を探すので、`answer:lower()` は `string.lower(answer)` になります。

<div class="note">

これはコードに直接書いた文字列には機能しません。`"YES":lower()` は構文エラーです。Lua は変数名または括弧の後ろの `:` しか受け付けないからです。機能させるには、文字列を括弧で囲んでください。

```lua
print(("YES"):lower())
-- これはエラーになります：
print("YES":lower())
```

</div>

## 括弧なし

関数が 1 つの引数で呼ばれ、その引数がコードに直接書かれた `string` または `table` である場合、括弧は省略できます。

<div class="runner">

```lua
print "Hello!"
print("Hello!")
```

</div>

これは他の人のコードで見かけるでしょう。このコースでは括弧を残します。関数が呼び出されていることが明確になるからです。

## 複数の値を返す

関数は 1 つ以上の値を返せます。`return` の後で `,` で区切り、複数の変数で受け取ります。これも `,` で区切ります。

<div class="runner">

```lua
function first_and_last(list)
    return list[1], list[#list]
end

local first, last = first_and_last({ "a", "b", "c", "d" })

print(first)
print(last)
```

</div>

変数の数が値より少ない場合、余分な値は単に捨てられます。変数の数が多い場合、余分な変数は `nil` になります。

<div class="runner">

```lua
function first_and_last(list)
    return list[1], list[#list]
end

local only_first = first_and_last({ "a", "b", "c", "d" })
local first, last, other = first_and_last({ "a", "b", "c", "d" })

print(only_first)
print(first, last, other)
```

</div>

<div class="note">

`function` の最初の値だけを保持するには、呼び出しを括弧で囲んでください。

<div class="runner">

```lua
function first_and_last(list)
    return list[1], list[#list]
end

print( first_and_last({ "a", "b", "c", "d" }) )
print((first_and_last({ "a", "b", "c", "d" })))
```
</div>

</div>

## 引数の不足と余分

Lua は `function` に渡す引数の数を確認しません。不足している引数は `nil` で、想定されていない引数は無視されます。

<div class="runner">

```lua
function show(a, b)
    print(a, b)
end

show(1, 2)
show(1)
show(1, 2, 3)
```

</div>

## オプション引数

不足している引数は `nil` なので、それを確認して代わりにデフォルト値を使えます。これにより引数がオプションになります。

<div class="runner">

```lua
function greet(name, greeting)
    if greeting == nil then
        greeting = "Hello"
    end

    print(greeting .. ", " .. name .. "!")
end

greet("Bob")
greet("Bob", "Good evening")
```
