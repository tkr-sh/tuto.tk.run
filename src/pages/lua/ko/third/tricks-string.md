## 따옴표

문자열은 큰따옴표 `"..."` 또는 작은따옴표 `'...'`로 쓸 수 있습니다. 둘 다 정확히 같은 문자열이 됩니다. 텍스트에 둘 중 하나의 따옴표가 포함되어 있을 때 유용합니다.

<div class="runner">

```lua
print("오늘은 날씨가 좋습니다")
print('그가 "안녕"이라고 말했다')
```

</div>

## 여러 줄 문자열

다음처럼 `"..."` 문자열 중간에 단순히 줄바꿈을 넣을 수는 없습니다.

<div class="runner">

```lua
print("1번째 줄
2번째 줄")
```
</div>

`"` 또는 `'`로 문자열을 쓸 때 Lua는 그런 줄바꿈을 허용하지 않습니다.

하지만! `"` 또는 `'`를 `[[`/`]]`로 바꾸면 가능합니다.

<div class="runner">

```lua
print([[1번째 줄
2번째 줄]])
```

</div>

## 길이

테이블에 사용한 `#` 연산자는 문자열에도 작동합니다.

<div class="runner">

```lua
print(#"hello")
print(#"hello world")
```

</div>

## `string.sub`
`string.sub` 함수는 시작과 끝 인덱스를 지정해 `string`의 일부로 새로운 `string`을 만들 수 있습니다.

<div class="runner">

```lua
print(("hello"):sub(2, 4))
-- 다음과 같음
-- print(string.sub("hello", 2, 4))
```

</div>

## `string.rep`
`string.rep` 함수는 `string`을 여러 번 반복할 수 있습니다.

<div class="runner">

```lua
print(("hello"):rep(3))
```

</div>

## `string.reverse`
`string.reverse` 함수는 이름 그대로 `string`을 뒤집습니다.

<div class="runner">

```lua
print(("hello"):reverse())
```

</div>

## `string.gsub`
`string.gsub` 함수는 어떤 `string`을 다른 `string`으로 바꾸고, 몇 번 바뀌었는지도 출력합니다.

<div class="runner">

```lua
print(("hello"):gsub("lo", "d"))
print(("hello"):gsub("l", "f"))
```

</div>

---

모든 `string` 함수는 여기에서 확인할 수 있습니다: <https://tkr-sh.github.io/lua-std-doc/lua_std/struct.string.html>
