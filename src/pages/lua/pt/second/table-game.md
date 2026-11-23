Agora que sabemos tudo isso, podemos fazer nosso histórico de partidas funcionar!

Para criar um histórico das nossas pontuações, podemos criar uma nova tabela `scores` no início do nosso programa e, toda vez que terminarmos uma partida, adicionarmos o `numero_de_tentativas` a ela como `scores[#scores + 1]`, ou seja, o último elemento.

Quando o jogo terminar, podemos simplesmente percorrer a tabela com `ipairs` e exibir todas as nossas pontuações!


Isso nos dá nosso programa final:

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

scores = {}

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

    scores[#scores + 1] = numero_de_tentativas

    resposta_do_usuario = ler_validado(
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
