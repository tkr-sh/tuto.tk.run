## Funções em linha

Até agora, escrevíamos as funções assim:

```lua
function dobro(number)
    return number * 2
end
```

Isto é açúcar. O que o Lua realmente faz é criar uma função sem nome e guardá-la numa variável chamada `dobro`:

```lua
dobro = function(number)
    return number * 2
end
```

As duas versões são exatamente iguais. É também por isso que podíamos passar `tonumber` para `ler_validado`: uma função é apenas um valor guardado numa variável, como um `number` ou uma `string`.


## Chamar funções com `:`

Para pôr uma string em minúsculas, escrevíamos `string.lower(resposta_do_usuario)`.
Mas, quando você usa uma função da biblioteca `string` ou `table` que recebe como primeiro argumento uma `string` ou uma `table`, pode escrevê-la com a seguinte sintaxe: `resposta_do_usuario:lower()`:

<div class="runner">

```lua
local resposta = "SIM"

print(string.lower(resposta))
print(resposta:lower())
```

</div>

O `:` pega no valor à sua esquerda e entrega-o à função como primeiro argumento. Para uma string, o Lua procura a função na tabela `string`, por isso `resposta:lower()` torna-se `string.lower(resposta)`.

<div class="note">

Isto não funciona com uma string escrita diretamente no código: `"SIM":lower()` é um erro de sintaxe, porque o Lua só aceita `:` depois de um nome de variável ou depois de parênteses. Para que funcione, ponha a string entre parênteses:

```lua
print(("SIM"):lower())
-- Isto dá um erro:
print("SIM":lower())
```

</div>

## Sem parênteses

Quando uma função é chamada com um único argumento, e esse argumento é uma `string` ou uma `table` escrita diretamente no código, os parênteses são opcionais:

<div class="runner">

```lua
print "Olá!"
print("Olá!")
```

</div>

Você verá isto no código de outras pessoas. Neste curso mantemos os parênteses, porque deixam óbvio que uma função está sendo chamada.

## Retornar vários valores

Uma função pode retornar mais de um valor. Você os separa com `,` depois do `return`, e os recebe em várias variáveis, também separadas com `,`:

<div class="runner">

```lua
function primeiro_e_ultimo(lista)
    return lista[1], lista[#lista]
end

local primeiro, ultimo = primeiro_e_ultimo({ "a", "b", "c", "d" })

print(primeiro)
print(ultimo)
```

</div>

Se você der menos variáveis do que valores, os valores extras são simplesmente descartados. Se der mais, as variáveis extras são `nil`:

<div class="runner">

```lua
function primeiro_e_ultimo(lista)
    return lista[1], lista[#lista]
end

local apenas_primeiro = primeiro_e_ultimo({ "a", "b", "c", "d" })
local primeiro, ultimo, outro = primeiro_e_ultimo({ "a", "b", "c", "d" })

print(apenas_primeiro)
print(primeiro, ultimo, outro)
```

</div>

<div class="note">

Para manter apenas o primeiro valor retornado por uma função, ponha a chamada entre parênteses:

<div class="runner">

```lua
function primeiro_e_ultimo(lista)
    return lista[1], lista[#lista]
end

print( primeiro_e_ultimo({ "a", "b", "c", "d" }) )
print((primeiro_e_ultimo({ "a", "b", "c", "d" })))
```
</div>

</div>

## Argumentos faltantes e extras

O Lua não verifica o número de argumentos que você dá a uma função. Um argumento faltante é `nil`, e um argumento inesperado é ignorado:

<div class="runner">

```lua
function mostrar(a, b)
    print(a, b)
end

mostrar(1, 2)
mostrar(1)
mostrar(1, 2, 3)
```

</div>

## Argumentos opcionais

Como um argumento faltante é `nil`, podemos verificá-lo e usar um valor padrão em vez disso. Isso torna o argumento opcional:

<div class="runner">

```lua
function cumprimentar(nome, saudacao)
    if saudacao == nil then
        saudacao = "Olá"
    end

    print(saudacao .. ", " .. nome .. "!")
end

cumprimentar("Bob")
cumprimentar("Bob", "Boa noite")
```

</div>
