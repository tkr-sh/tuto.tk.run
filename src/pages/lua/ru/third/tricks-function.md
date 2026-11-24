## Встраиваемые функции

До сих пор мы писали функции так:

```lua
function double(number)
    return number * 2
end
```

Это сахар. На самом деле Lua создаёт функцию без имени и сохраняет её в переменной с именем `double`:

```lua
double = function(number)
    return number * 2
end
```

Обе версии абсолютно одинаковы. Вот почему мы могли передать `tonumber` в `validated_read`: функция — это просто значение, хранящееся в переменной, как число или строка.


## Вызов функций с помощью `:`

Чтобы перевести строку в нижний регистр, мы писали `string.lower(user_answer)`.
Но когда вы используете функцию из библиотеки `string` или `table`, которая принимает в качестве первого аргумента `string` или `table`, вы можете записать это с помощью следующего синтаксиса: `user_answer:lower()`:

<div class="runner">

```lua
local answer = "YES"

print(string.lower(answer))
print(answer:lower())
```

</div>

`:` берёт значение слева от себя и передаёт его функции в качестве первого аргумента. Для строки Lua ищет функцию в таблице `string`, поэтому `answer:lower()` превращается в `string.lower(answer)`.

<div class="note">

Это не работает со строкой, записанной прямо в коде: `"YES":lower()` — это синтаксическая ошибка, потому что Lua принимает `:` только после имени переменной или после круглых скобок. Чтобы это заработало, поместите строку в круглые скобки:

```lua
print(("YES"):lower())
-- Это вызывает ошибку:
print("YES":lower())
```

</div>

## Без круглых скобок

Когда функция вызывается с одним аргументом, и этот аргумент — это `string` или `table`, записанные прямо в коде, круглые скобки необязательны:

<div class="runner">

```lua
print "Привет!"
print("Привет!")
```

</div>

Вы увидите это в коде других людей. В этом курсе мы сохраняем круглые скобки, потому что они делают очевидным, что вызывается функция.

## Возврат нескольких значений

Функция может возвращать больше одного значения. Вы разделяете их с помощью `,` после `return`, а получаете в нескольких переменных, также разделённых с помощью `,`:

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

Если вы укажете меньше переменных, чем есть значений, лишние значения просто отбрасываются. Если больше, лишние переменные будут `nil`:

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

Чтобы сохранить только первое значение `function`, поместите вызов в круглые скобки:

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

## Недостающие и лишние аргументы

Lua не проверяет количество аргументов, которые вы передаёте `function`. Недостающий аргумент равен `nil`, а неожиданный аргумент игнорируется:

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

## Необязательные аргументы

Поскольку недостающий аргумент равен `nil`, мы можем проверить его и вместо него использовать значение по умолчанию. Это делает аргумент необязательным:

<div class="runner">

```lua
function greet(name, greeting)
    if greeting == nil then
        greeting = "Привет"
    end

    print(greeting .. ", " .. name .. "!")
end

greet("Bob")
greet("Bob", "Добрый вечер")
```
