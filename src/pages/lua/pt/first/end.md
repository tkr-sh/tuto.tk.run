Então, agora a única coisa que resta é exibir o número de tentativas.

Nós o incrementamos logo após obter os dados do usuário com

```lua
numero_de_tentativas = numero_de_tentativas + 1
```

Mas agora, gostaríamos de exibir uma mensagem de parabéns e, ao mesmo tempo, quantas tentativas foram necessárias.

É hora de aprender o último operador antes do fim deste primeiro jogo:

`..`: Concatena 2 valores.

<div class="runner">

```lua
print("Já se passaram " .. 4 .. " anos")
```

</div>

Com isso em mente, podemos criar uma mensagem legal:

```lua
print("Você ganhou em " .. numero_de_tentativas .. " tentativas!")
```

E aqui está FINALMENTE o nosso jogo:

---

<div class="runner">

```lua
numero_de_tentativas = 0
numero_para_adivinhar = random(1, 100)

print("Tente adivinhar o número aleatório entre 1 e 100!")

repeat
	chute_do_usuario = tonumber(read())
	numero_de_tentativas = numero_de_tentativas + 1

	if numero_para_adivinhar < chute_do_usuario then
		print("O número é mais baixo!")
	end

	if numero_para_adivinhar > chute_do_usuario then
		print("O número é mais alto!")
	end
until chute_do_usuario == numero_para_adivinhar

print("Você ganhou em " .. numero_de_tentativas .. " tentativas!")
```

</div>

---

## UAU.

Uau! Não é?

Você está pronto para se tornar um programador agora! :) 

## É o fim?

Sim e não.

Infelizmente, eu simplifiquei uma coisa: normalmente você não pode executar `read` e `random` assim. O Lua gosta de colocar as funções «integradas» em algumas categorias: `math` (para as funções matemáticas), `io` (para as interações de entrada/saída), `os` (para interagir com o seu sistema operacional), ...

E, portanto, quando você quer usar funções específicas que fazem parte de uma categoria, você precisa chamá-las como `categoria.a_funcao()`

No nosso caso, `random` faz parte da categoria `math`, e `read` faz parte da categoria `io`. `print` e `tonumber` são globais, então você não precisa especificar nada.

<div class="runner">

```lua
numero_de_tentativas = 0
numero_para_adivinhar = math.random(1, 100)

print("Tente adivinhar o número aleatório entre 1 e 100!")

repeat
	chute_do_usuario = tonumber(io.read())
	numero_de_tentativas = numero_de_tentativas + 1

	if numero_para_adivinhar < chute_do_usuario then
		print("O número é mais baixo!")
	end

	if numero_para_adivinhar > chute_do_usuario then
		print("O número é mais alto!")
	end
until chute_do_usuario == numero_para_adivinhar

print("Você ganhou em " .. numero_de_tentativas .. " tentativas!")
```

</div>

## Parabéns, você agora concluiu seu primeiro programa Lua! 🎉

Agora que você tem as bases, vamos tentar melhorar nosso jogo atual para adicionar novas funcionalidades e vamos aprender coisas novas pelo caminho!
