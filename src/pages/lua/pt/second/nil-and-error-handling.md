A primeira coisa que você talvez tenha notado é que nosso jogo atualmente *trava* quando inserimos um número que não é válido:

<div class="runner">

```lua
chute_do_usuario = tonumber(io.read())

if 10 < chute_do_usuario then
	print("O número é mais baixo!")
end
```

</div>

o que, se você não digitar um número, vai te dar:

```
Error: [string "chute_do_usuario = tonumber(io.read())..."]:3: attempt to compare number with nil
```

Mas o que isso significa?

O conceito central aqui é `nil`. Ao apresentar os tipos de Lua, vimos brevemente o tipo `nil`:

> `nil`: Vazio. O objetivo de `nil` é dizer que você não tem um valor.

<div style="height: 3px"></div>

E é _exatamente_ isso que `tonumber` faz:

- Se `io.read()` é uma string que representa um número como `"3.14"` ou `"42"`, o Lua sabe como convertê-la de um tipo `string` para um tipo `number`.
- Caso contrário, se `io.read()` é uma string que não se parece com um `number`, `tonumber` simplesmente diz «não sei como converter isto em número, vamos retornar `nil`».

Portanto, se em `io.read()` não passarmos uma representação correta de um número, teremos simplesmente `nil`:

<div class="runner">

```lua
chute_do_usuario = tonumber("123abc")
print(chute_do_usuario)
```

</div>

## Ok, mas então por que isso gera um erro?

Boa pergunta! Por enquanto, `chute_do_usuario` é apenas `nil`. O erro ocorre quando tentamos comparar com o operador `<` um `number` com `nil` nesta linha:

```lua
if 10 < chute_do_usuario then
```

E o erro é bem descrito na mensagem de erro:

```
Error: [string "chute_do_usuario = tonumber(io.read())..."]:3: attempt to compare number with nil
```

- `:3` indica a linha em que o erro ocorre; neste caso, nossa linha de comparação.
- `attempt to compare number with nil` é o motivo pelo qual houve um erro.

E isso faz todo o sentido! O que significa comparar, em termos de maior ou menor, nada com um número? É `false`? É `true`? Não, a pergunta simplesmente não faz nenhum sentido. Então o Lua gera um erro.

## Tratando o erro
A maneira mais rápida de tratar isso é simplesmente verificar se `chute_do_usuario` é `nil`, e isso pode ser feito com o operador `==`:

```lua
if chute_do_usuario == nil then
	print("Este não é um número válido")
end
```

Mas, neste caso, não queremos apenas exibir algo e continuar o loop, porque isso contaria como uma tentativa por causa de:

```lua
numero_de_tentativas = numero_de_tentativas + 1
```

Uma maneira simples de fazer isso seria pedir `chute_do_usuario` até que não seja mais `nil` com o operador `~=`, ou seja, até que seja um `number` válido:

```lua
repeat
	chute_do_usuario = tonumber(io.read())
until chute_do_usuario ~= nil
```

Também podemos adicionar uma mensagem de erro caso não seja, com:

```lua
repeat
	chute_do_usuario = tonumber(io.read())
	if chute_do_usuario == nil then 
		print("Este não é um número válido!")
	end
until chute_do_usuario ~= nil
```

O que nos dá este programa completo:


<div class="runner">

```lua
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
```

</div>
