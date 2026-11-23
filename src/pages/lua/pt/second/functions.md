Você deve ter notado que fazemos um bloco de código semelhante duas vezes para validar a entrada:

```lua
repeat
    print("Tente adivinhar um número:")
    chute_do_usuario = tonumber(io.read())
    if chute_do_usuario == nil then 
        print("Este não é um número válido!")
    end
until chute_do_usuario ~= nil
```

```lua
repeat 
    print("Você quer continuar? sim / não")
    resposta_do_usuario = string.lower(io.read())
    if resposta_do_usuario ~= "sim" and resposta_do_usuario ~= "não" then
        print("Esta não é uma resposta válida.")
    end
until resposta_do_usuario == "sim" or resposta_do_usuario == "não"
```

Há claramente um padrão comum duplicado que vou tentar ilustrar com uma sintaxe pseudo/inválida delimitada por `<` e `>`.

```lua
repeat 
    print(<pergunta>)
    entrada_do_usuario = <transformar>(io.read())
    if <condicao_inversa> then
        print(<mensagem_erro>)
    end
until <condicao>
```

## Fundamentos de uma função

Quando você escreve um programa, muitas vezes existe uma regra a seguir: escrever as coisas de forma modular, para que sejam úteis no futuro e genéricas o suficiente para lidar com a maioria dos casos, e é exatamente isso que as funções nos ajudam a alcançar.

Uma função é basicamente um conjunto de instruções que um programa executará. Uma função pode receber parâmetros e retornar um valor. Vamos pegar esta função como exemplo:

```lua
function dobro(meu_numero)
    return meu_numero * 2
end
```

O nome desta função é `dobro` e ela recebe um único parâmetro chamado `meu_numero`. O resultado desta função, o que ela retorna, é um único valor: `meu_numero * 2`. Você pode chamar uma função especificando o nome e depois, entre parênteses e separados por `,`, os argumentos, assim `minha_funcao(valor1, valor2)`. No nosso caso:

<div class="runner">

```lua
function dobro(meu_numero)
    return meu_numero * 2
end

print(dobro(42))
```

</div>

## Voltando ao nosso jogo!

Então, vamos otimizar o tratamento da entrada! A primeira coisa que devemos observar é de que a nossa função depende. Isso é necessário para saber quais argumentos ela precisa. No nosso caso:
* Mensagem de pedido, a mensagem que diz ao usuário para inserir um valor
* Mensagem de erro, a mensagem a exibir quando a entrada não é válida
* Transformação, uma transformação que aplicamos à string que lemos do usuário (`tonumber`, `string.lower`, `string.upper`, ...)
* Condição, uma verificação para saber se a entrada corresponde ao que esperamos após aplicar a transformação.

`mensagem_pedido` e `mensagem_erro` são bem simples, são apenas strings.<br/>

`transformar` e `condicao` são um pouco mais complicados.<br>
`transformar` recebe a entrada do usuário e devolve uma versão modificada dela. Este é claramente o caso de uma função: algo que recebe uma entrada e devolve algo.<br/>
`condicao` também é muito parecida: recebe como argumento o resultado de `transformar` e retorna um booleano (`true`/`false`) conforme o resultado. Este é claramente mais um caso em que uma `function` é necessária.

```lua
function ler_validado(mensagem_pedido, mensagem_erro, transformar, condicao)
    repeat 
        print(mensagem_pedido)
        entrada_do_usuario = transformar(io.read())
        e_valido = condicao(entrada_do_usuario)

        if not e_valido then
            print(mensagem_erro)
        end
    until e_valido

    return entrada_do_usuario
end
```

Observe o `not`! A palavra-chave `not` inverte um booleano: `true` => `false`, `false` => `true`.

No nosso caso, podemos simplesmente escrever isto para o tratamento da entrada do número pelo usuário:

```lua
function e_nao_nil(arg) 
    return arg ~= nil
end

ler_validado(
    "Tente adivinhar um número:",
    "Este não é um número válido",
    tonumber,
    e_nao_nil
)
```

Isso produzirá o mesmo que:

```lua
repeat
    print("Tente adivinhar um número:")
    chute_do_usuario = tonumber(io.read())
    e_valido = chute_do_usuario ~= nil
    if not e_valido then 
        print("Este não é um número válido!")
    end
until e_valido
```

## Funções em linha

Observe que definimos `e_nao_nil` apenas para usá-la uma vez. Em vez de declarar uma função com nome quando não precisamos dela, o Lua nos permite simplesmente remover o nome quando não queremos usá-la em outro lugar, assim:

```lua
local dobro = function(numero)
    return numero * 2
end
```

No nosso caso, podemos usá-la para fazer:

```lua
ler_validado(
    "Tente adivinhar um número:",
    "Este não é um número válido",
    tonumber,
    function(nb) return nb ~= nil end
)
```

O que nos dá nosso programa final:

<div class="runner">

```lua
function ler_validado(mensagem_pedido, mensagem_erro, transformar, condicao)
    repeat 
        print(mensagem_pedido)
        entrada_do_usuario = transformar(io.read())
        e_valido = condicao(entrada_do_usuario)

        if not e_valido then
            print(mensagem_erro)
        end
    until e_valido

    return entrada_do_usuario
end

repeat 
	numero_de_tentativas = 0
	numero_para_adivinhar = math.random(1, 100)

	print("Tente adivinhar o número aleatório entre 1 e 100!")

	repeat
        chute_do_usuario = ler_validado(
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

    resposta_do_usuario = ler_validado(
        "Você quer continuar? sim / não",
        "Esta não é uma resposta válida.",
        string.lower,
        function(resposta) return resposta == "sim" or resposta == "não" end
    )
until resposta_do_usuario == "não"

print("Tchau!")
```

</div>
