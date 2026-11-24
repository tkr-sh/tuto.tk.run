```lua
user_guess = tonumber(read())
```

좋아요, 이 코드는 이상해 보일 수 있습니다(함수 안에 함수?). 하지만 이렇게 볼 수 있습니다.

```lua
read_value = read()
user_guess = tonumber(read_value)
```

일어나는 일은 다음과 같습니다.
1. Lua는 오른쪽 값을 계산하려고 합니다
2. Lua는 `tonumber`를 실행하려고 하지만, 그러려면 모든 입력에 값이 있어야 합니다
3. 따라서 `read()`를 보는 순간, 함수가 값을 만들어 내도록 즉시 실행하려고 합니다
4. `read()`가 실행되어 값을 반환한 후에야 `tonumber`를 실행할 수 있습니다!

### 좋아요, 그런데 `read()`는 무엇을 하나요?

`read()`는 사용자에게 문자열을 요청하고 그것을 받아옵니다.

**예제를 실행하기 전에:** 이 사이트의 팝업을 비활성화하지 마세요. 그러면 다시 `read()`할 수 없게 됩니다!



<div class="runner">

```lua
print(read())
```

</div>

### 그럼 `tonumber`는요?

`read()`는 사용자로부터 `string`을 받기 때문에, 그것을 무작위 숫자와 비교할 수 없습니다. 숫자가 아닌 문자가 포함될 수 있는 `string`과 `number`를 비교할 수는 없기 때문입니다(예: `"x4%$A" > 3`은 말이 되지 않습니다).

따라서 사용자의 입력을 숫자로 변환해야 하며, 바로 그것이 `tonumber`가 하는 일입니다!

이 함수는 입력이 유효한 숫자가 아니면 `nil`을 반환합니다.

<div class="runner">

```lua
print(tonumber("숫자가 아님") == nil)
```

</div>

그리고 `nil` + `number`를 더할 수 없으므로, 유효한 숫자를 입력하세요!

<div class="runner">

```lua
print(tonumber(read()) + 1)
```

</div>

사용자로부터 `string`을 읽고, 그것을 `number`로 변환한 뒤 이 숫자에 1을 더합니다. 유효한 숫자가 아니면 `tonumber(read())`는 `nil` 값이 됩니다.
