E se pudéssemos ter várias partidas em uma única execução, e deixar o usuário decidir quando parar?

Para tornar o jogo repetível, simplesmente envolvemos o programa em outro loop `repeat ... until` assim:

```lua
repeat
	...
	print("Você ganhou em ... tentativas!")

	repeat 
		print("Você quer continuar? sim / não")
		resposta_do_usuario = io.read()
	until resposta_do_usuario == "sim" or resposta_do_usuario == "não"
until resposta_do_usuario == "não"

print("Tchau!")
```

Usamos uma palavra-chave especial `or`, que combina duas condições e é `true` se pelo menos uma delas for verdadeira.

Digamos, por exemplo, que temos um booleano `a` que pode ser `true` ou `false`, e outro booleano `b` que também pode ser `true` ou `false`. A expressão `a or b` será, portanto:

|a|b|a or b|
|-|-|------|
|`true`|`true`|`true`|
|`true`|`false`|`true`|
|`false`|`true`|`true`|
|`false`|`false`|`false`|

`a or b` só é `false` quando `a` e `b` são `false`.
É o oposto do operador `and`, que exige que `a` e `b` sejam `true` para ser `true`.

|a|b|a and b|
|-|-|-------|
|`true`|`true`|`true`|
|`true`|`false`|`false`|
|`false`|`true`|`false`|
|`false`|`false`|`false`|


## Maiúsculas e minúsculas

Em Lua, uma string com letras maiúsculas como `"A"`, `"B"`, `"C"`, ... não é a mesma que `"a"`, `"b"`, `"c"`, .... Isso significa que, se um usuário digitar `"Sim"`, não vai funcionar e o usuário refará o loop `repeat` mesmo tendo digitado `"Sim"`. Provavelmente não é isso que queremos...

Felizmente, o Lua tem funções que nos permitem converter uma string para tudo maiúsculo ou tudo minúsculo: `string.upper` e `string.lower`

<div class="runner">

```lua
print(string.lower("Abc")) -- abc
print(string.upper("Abc")) -- ABC
```

</div>

O que nos permite fazer o seguinte

<div class="runner">

```lua
repeat 
	print("Você quer continuar? sim / não")
	resposta_do_usuario = string.lower(io.read())
	if resposta_do_usuario ~= "sim" and resposta_do_usuario ~= "não" then
		print("Esta não é uma resposta válida.")
	end
until resposta_do_usuario == "sim" or resposta_do_usuario == "não"
```

</div>

Com isso, `resposta_do_usuario` estará sempre em minúsculas graças a `string.lower()`.
Também adicionamos uma verificação para exibir uma mensagem se a resposta não for nem `"sim"` nem `"não"`. 
O que nos dá este programa completo:

<div class="runner">

```lua
repeat 
	numero_de_tentativas = 0
	numero_para_adivinhar = math.random(1, 100)

	print("Tente adivinhar o número aleatório entre 1 e 100!")

	repeat
		repeat
			chute_do_usuario = tonumber(io.read())
			if chute_do_usuario == nil then 
				print("Este não é um número válido!")
			end
		until chute_do_usuario ~= nil
		numero_de_tentativas = numero_de_tentativas + 1

		if numero_para_adivinhar < chute_do_usuario then
			print("O número é mais baixo!")
		end

		if numero_para_adivinhar > chute_do_usuario then
			print("O número é mais alto!")
		end
	until chute_do_usuario == numero_para_adivinhar

	print("Você ganhou em " .. numero_de_tentativas .. " tentativas!")

	repeat 
		print("Você quer continuar? sim / não")
		resposta_do_usuario = string.lower(io.read())
		if resposta_do_usuario ~= "sim" and resposta_do_usuario ~= "não" then
			print("Esta não é uma resposta válida.")
		end
	until resposta_do_usuario == "sim" or resposta_do_usuario == "não"
until resposta_do_usuario == "não"

print("Tchau!")
```

</div>
