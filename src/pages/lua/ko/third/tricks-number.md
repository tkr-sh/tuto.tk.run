## 더 많은 연산자

`+`, `-`, `*`, `/`는 이미 알고 있습니다. Lua에는 `number`를 위한 연산자가 세 가지 더 있습니다.

|연산자|하는 일|
|-|-|
|`//`|나눗셈을 하고 소수 부분을 제거합니다|
|`%`|나눗셈의 나머지를 돌려줍니다|
|`^`|거듭제곱합니다|

<div class="runner">

```lua
print(7 / 2)
print(7 // 2)
print(7 % 2)
print(2 ^ 10)
```

</div>

`%` 연산자는 `number`가 짝수인지 홀수인지 아는 데 매우 유용합니다. `number`를 `2`로 나눈 나머지가 `0`이면 짝수입니다.

<div class="runner">

```lua
print(10 % 2 == 0)
print(7 % 2 == 0)
```

</div>

## 정수 `number`와 소수 `number`

Lua에는 두 종류의 `number`가 있습니다. `5` 같은 정수 `number`와 `5.0`이나 `3.14` 같은 소수 `number`입니다. 대부분의 경우 이것을 생각할 필요가 없지만, 일부 결과가 왜 `.0`과 함께 출력되는지 설명해 줍니다.

<div class="runner">

```lua
print(10 / 2)
print(10 // 2)
print(5 == 5.0)
```

</div>

`/` 연산자는 결과가 딱 떨어져도 항상 소수 `number`를 돌려줍니다. `5`와 `5.0`은 여전히 같습니다.

<div class="note">

소수 `number`는 항상 정확하지는 않습니다! 컴퓨터는 `0.1` 같은 `number`를 완벽하게 저장할 수 없으므로 일부 비교는 놀라운 결과를 냅니다.

<div class="runner">

```lua
print(0.1 + 0.2 == 0.3)
```
</div>

소수 `number`를 사용한 계산 결과에는 `==`를 사용하지 마세요.

</div>

## `number`를 쓰는 다른 방법

아주 크거나 아주 작은 `number`는 `e`를 사용해 쓸 수 있습니다. 이것은 "10의 몇 제곱"을 의미합니다. `0x`로 시작해 [16진수](https://en.wikipedia.org/wiki/Hexadecimal)로 `number`를 쓸 수도 있습니다.

<div class="runner">

```lua
print(1e3)
print(2.5e-2)
print(0xFF)
```

</div>

## `number`와 문자열 변환

`tostring` 함수는 `number`를 `string`으로 변환하고, 이미 사용한 `tonumber`는 그 반대를 합니다.

<div class="runner">

```lua
print(tostring(42) .. "!")
print(tonumber("42") + 1)
```

</div>

Lua는 가능할 때 스스로 변환도 합니다. `number`처럼 보이는 `string`은 계산에서 `number`가 되고, `number`는 `..`를 통해 `string`이 됩니다.

<div class="runner">

```lua
print("10" + 5)
print(10 .. 20)
```

</div>

## `math.floor`와 `math.ceil`
`math.floor` 함수는 `number`를 내림하고, `math.ceil`은 올림합니다.

<div class="runner">

```lua
print(math.floor(3.7))
print(math.ceil(3.2))
```

</div>

가장 가까운 정수 `number`로 반올림하려면 `math.floor`를 사용하기 전에 `0.5`를 더하세요.

<div class="runner">

```lua
print(math.floor(3.7 + 0.5))
print(math.floor(3.2 + 0.5))
```

</div>

## `math.max`와 `math.min`
`math.max` 함수는 전달한 `number` 중 가장 큰 값을, `math.min`은 가장 작은 값을 돌려줍니다.

<div class="runner">

```lua
print(math.max(4, 9, 2))
print(math.min(4, 9, 2))
```

</div>

## `math.abs`
`math.abs` 함수는 `number`의 부호를 제거합니다. 이것은 두 `number` 사이의 거리를 구할 때 유용합니다.

<div class="runner">

```lua
print(math.abs(-5))
print(math.abs(3 - 10))
```

</div>

## `math.sqrt`
`math.sqrt` 함수는 `number`의 제곱근을 돌려줍니다.

<div class="runner">

```lua
print(math.sqrt(16))
```

</div>

---

모든 `math` 함수는 여기에서 확인할 수 있습니다: <https://tkr-sh.github.io/lua-std-doc/lua_std/math/index.html>
