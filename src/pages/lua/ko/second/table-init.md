이제 우리가 게임을 끝낼 때까지 점수 기록을 가질 수 있다면 어떨까요? 일어난 모든 것의 기록을? 어떻게 하면 될까요?

첫 번째 접근 방식은 다음과 같이 여러 변수를 정의하는 것일 수 있습니다.

```lua 
score_first_game = nil
score_second_game = nil
score_third_game = nil
...
```

하지만 이것은 올바른 접근 방식이 아닙니다. 사용자가 여러 번 플레이하면 지속할 수 없습니다. 또한 루프와 깔끔하게 연결하기 어렵습니다.

이를 해결하려면 새로운 데이터 타입 `table`이 필요합니다.

## 테이블이란?

`table`은 Lua의 타입으로, 임의의 길이의 데이터를 저장할 수 있습니다. 각 Lua 테이블은 다음과 같이 `[key] = value` 형식으로 값에 키를 할당합니다.

```lua
my_table = {
    ["a"] = 7,
    ["b"] = 8,
    ["c"] = 4,
}
```

테이블의 값에 접근하려면 키를 참조해 `my_table[my_key]`라고 씁니다. 예를 들어:

<div class="runner">

```lua
my_table = { ["a"] = 7, ["b"] = 8, ["c"] = 4 }

print(my_table["b"])
```

</div>

그리고 존재하지 않는 키에 값을 할당해 테이블에 새 값을 간단히 추가할 수 있습니다.

<div class="runner">

```lua
my_table = { ["a"] = 7, ["b"] = 8, ["c"] = 4 }

my_table["d"] = 5
print(my_table["d"])
```

</div>

키를 쓰지 않을 수도 있습니다. 이 경우 키는 1부터 시작해 이후 증가합니다.


<div class="runner">

```lua
my_table = { "a", "b", "c", "z" }
-- 다음과 같음
-- my_table = { [1] = "a", [2] = "b", [3] = "c", [4] = "z" }

print(my_table[2])
print(my_table[4])
```

</div>
