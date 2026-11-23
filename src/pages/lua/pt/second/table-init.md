E se agora pudéssemos ter um histórico das nossas pontuações, até terminarmos de jogar? Um histórico de tudo o que aconteceu? Como faríamos isso?

Sua primeira abordagem pode ser definir várias variáveis assim:

```lua 
pontuacao_primeira_partida = nil
pontuacao_segunda_partida = nil
pontuacao_terceira_partida = nil
...
```

Mas esta não é uma abordagem correta. Isso não é sustentável se o usuário jogar muitas vezes. É difícil de ligar de forma limpa com um loop.

Para resolver isso, precisaremos de um novo tipo de dado: `table`.

## O que é uma tabela?

Uma `table` é um tipo de Lua que nos permite armazenar uma quantidade de dados arbitrariamente longa ou curta. Cada tabela Lua associa uma chave a um valor no formato `[chave] = valor`, assim:

```lua
minha_tabela = {
    ["a"] = 7,
    ["b"] = 8,
    ["c"] = 4,
}
```

Podemos acessar o valor de uma tabela referenciando a chave e escrevendo `minha_tabela[minha_chave]`. Por exemplo:

<div class="runner">

```lua
minha_tabela = { ["a"] = 7, ["b"] = 8, ["c"] = 4 }

print(minha_tabela["b"])
```

</div>

E podemos simplesmente adicionar um novo valor à nossa tabela atribuindo uma chave inexistente a um valor, assim:

<div class="runner">

```lua
minha_tabela = { ["a"] = 7, ["b"] = 8, ["c"] = 4 }

minha_tabela["d"] = 5
print(minha_tabela["d"])
```

</div>

Você também pode não escrever nenhuma chave e, nesse caso, as chaves começarão em um e aumentarão depois:


<div class="runner">

```lua
minha_tabela = { "a", "b", "c", "z" }
-- Igual a
-- minha_tabela = { [1] = "a", [2] = "b", [3] = "c", [4] = "z" }

print(minha_tabela[2])
print(minha_tabela[4])
```

</div>
