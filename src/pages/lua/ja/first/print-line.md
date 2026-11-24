```lua
print("1 から 100 までのランダムな数を当ててみてください！")
```

この行も関数の呼び出しです！ `print` という名前があり、その名前の後ろに括弧が付いているのでわかりますね！

この関数は文字列を最初で唯一の引数として受け取ります。でも……私たちは出力を変数に入れたことはありません。

## なぜ？

`print` は値を返さないからです。`print` は、受け取ったデータをユーザーに表示する特殊な関数です。

たとえば、前の例に戻ってみましょう。

```lua
the_best_number = 40 + 2
my_grandpa_age = the_best_number * 2
```

`my_grandpa_age` の値が何かをどうやって知ればいいのでしょうか？ 頭の中で計算して推測することもできますが、プログラムがもっと複雑になると、いつもそうできるとは限りません。

そこで、次のようにするだけで

```lua
print(my_grandpa_age)
```

変数 `my_grandpa_age` の中身を表示できます。この場合：`84`

## それがわかったので

このとても便利な関数を知ったところで、次を紹介します。_“✨️**Lua ランナー**✨️”_ と _“✨️**インタラクティブ Lua エディター**✨️”_ です。ここで自分のプログラムを実行できます。

<div class="runner">

```lua
print("こんにちは、世界！")
```

</div>

この例は、「Run」をクリックするたびに再実行されることがわかります。

<div class="runner">

```lua
print(random(1, 100))
```

</div>

そして最後に、おじいさんの年齢を正しく知っていることを確認できます。

<div class="runner">

```lua
the_best_number = 40 + 2
print(the_best_number)
my_grandpa_age = the_best_number * 2
print(my_grandpa_age)
```

</div>
