Até agora, só declaramos variáveis de Lua com esta sintaxe:

```lua
minha_variavel = "valor"
```

E funciona, claramente, como vimos. Mas pode não ser sempre a melhor forma de fazer isso.
Vamos pegar este cenário:

<div class="runner">

```lua
function minha_funcao(a)
    b = a * 2
    return a + b
end

print(minha_funcao(2))
print(b)
```
</div>

`b` ainda está definida _fora_ da função. O que, às vezes, pode ser o que você quer. Mas na maioria das vezes não é o comportamento esperado. Na maioria das vezes, você quer criar uma variável que só exista dentro de uma função, de um loop, de um escopo específico.

Para definir um valor que só exista em um escopo específico, podemos simplesmente colocar a palavra-chave `local` antes do nome da variável, assim:

```lua
local minha_variavel = "valor"
```

Com isso, a variável existirá até chegar ao fim do bloco (`end`, `repeat ... until`, ...) em que foi definida, ou ao fim do arquivo se foi definida fora de um bloco. Com isso, podemos corrigir nosso exemplo:

<div class="runner">

```lua
function minha_funcao(a)
    local b = a * 2
    return a + b
end

print(minha_funcao(2))
print(b)
```

</div>

<div class="note">

Os valores de `for` são `local` por padrão

<div class="runner">

```lua
for a = 0, 3 do
    local b = a * 2
end

print(a)

for k, v in ipairs({1, 2, 3}) do
    local b = k + v
end

print(k)
print(v)
```
</div>
</div>


<div class="note">

Declarar uma variável e atribuir um valor a ela são duas coisas diferentes:

- **Declarar** cria a variável com o escopo de `local`: `local minha_variavel`
- **Atribuir** dá um valor a uma variável: `minha_variavel = "valor"`

Escrever `local minha_variavel = "valor"` simplesmente faz as duas coisas ao mesmo tempo.

Você também pode declarar uma variável sem lhe dar um valor. Ela conterá `nil` até você atribuir algo a ela:

```lua
local chute_do_usuario
print(chute_do_usuario) -- nil

chute_do_usuario = 42
print(chute_do_usuario) -- 42
```

Escreva `local` apenas uma vez por variável: um segundo `local chute_do_usuario = 42` criaria uma nova variável (possivelmente em um novo escopo) em vez de alterar a primeira.

</div>

E podemos terminar nosso jogo com este exemplo final:

<div class="runner">

```lua
function ler_validado(mensagem_pedido, mensagem_erro, transformar, condicao)
    local entrada_do_usuario

    repeat 
        print(mensagem_pedido)
        entrada_do_usuario = transformar(io.read())
        local e_valido = condicao(entrada_do_usuario)

        if not e_valido then
            print(mensagem_erro)
        end
    until e_valido

    return entrada_do_usuario
end

local scores = {}

repeat 
	local numero_de_tentativas = 0
	local numero_para_adivinhar = math.random(1, 100)

	print("Tente adivinhar o número aleatório entre 1 e 100!")

	repeat
        local chute_do_usuario = ler_validado(
            "Insira um número entre 1 e 100:",
            "Este não é um número válido",
            tonumber,
            function(nb) return nb ~= nil end
        )
		numero_de_tentativas = numero_de_tentativas + 1

		if numero_para_adivinhar < chute_do_usuario then
			print("O número é mais baixo!")
		end

		if numero_para_adivinhar > chute_do_usuario then
			print("O número é mais alto!")
		end
	until chute_do_usuario == numero_para_adivinhar

	print("Você ganhou em " .. numero_de_tentativas .. " tentativas!")

    scores[#scores + 1] = numero_de_tentativas

    local resposta_do_usuario = ler_validado(
        "Você quer continuar? sim / não",
        "Esta não é uma resposta válida.",
        string.lower,
        function(resposta) return resposta == "sim" or resposta == "não" end
    )
until resposta_do_usuario == "não"

print("Pontuações:")

for numero_da_partida, score in ipairs(scores) do
    print("Partida " .. numero_da_partida .. ": concluída em " .. score .. " tentativas!")
end

print("Tchau!")
```

</div>
