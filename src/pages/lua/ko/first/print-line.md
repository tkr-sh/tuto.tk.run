```lua
print("1에서 100 사이의 무작위 숫자를 맞혀 보세요!")
```

이 줄도 함수 호출입니다! `print`라는 이름이 있고 그 이름 뒤에 괄호가 있기 때문에 알 수 있습니다!

이 함수는 문자열을 첫 번째이자 유일한 매개변수로 받습니다. 그런데…… 우리는 출력을 변수에 넣은 적이 없습니다.

## 왜?

`print`는 값을 반환하지 않기 때문입니다. `print`는 받은 데이터를 사용자에게 보여 주는 특별한 함수입니다.

예를 들어, 이전 예시를 다시 가져와 봅시다.

```lua
the_best_number = 40 + 2
my_grandpa_age = the_best_number * 2
```

`my_grandpa_age`의 값이 무엇인지 어떻게 알 수 있을까요? 머릿속으로 계산해서 추측할 수도 있지만, 프로그램이 더 복잡해지면 항상 가능한 것은 아닙니다.

따라서 그냥 이렇게 하면 됩니다.

```lua
print(my_grandpa_age)
```

그러면 변수 `my_grandpa_age`의 내용이 표시됩니다. 우리의 경우: `84`

## 이제 이것을 알았으니

이 아주 유용한 함수를 알게 되었으니, 이제 소개합니다. _“✨️**Lua 러너**✨️”_ 와 _“✨️**대화형 Lua 편집기**✨️”_ 입니다. 여기서 자신만의 프로그램을 실행할 수 있습니다.

<div class="runner">

```lua
print("안녕, 세상!")
```

</div>

이 예시는 "Run"을 클릭할 때마다 다시 실행되는 것을 볼 수 있습니다.

<div class="runner">

```lua
print(random(1, 100))
```

</div>

그리고 마지막으로, 할아버지의 나이를 정확히 알고 있는지 확인할 수 있습니다.

<div class="runner">

```lua
the_best_number = 40 + 2
print(the_best_number)
my_grandpa_age = the_best_number * 2
print(my_grandpa_age)
```

</div>
