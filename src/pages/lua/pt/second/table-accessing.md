Uma informação útil de que precisamos ao trabalhar com tabelas é «quantos elementos ela tem?». Por exemplo, se eu quiser obter o último elemento!

Isso é possível graças a um operador especial: `#`. Escrevendo `#` e depois o nome da tabela ou a tabela diretamente, você pode obter o tamanho total da tabela:


<div class="runner">

```lua
minha_tabela = { "a", "b", "c" }

print(#minha_tabela)
minha_tabela[4] = "d"
print(#minha_tabela)
```

</div>

Isso é especialmente útil se eu quiser conhecer o elemento de uma tabela com o índice «último», ou se eu quiser adicionar um novo elemento no final: `minha_tabela[#minha_tabela]` é o valor cuja chave é o comprimento total da tabela, ou seja, o último na maioria dos casos!

<div class="runner">

```lua
minha_tabela = { "a", "b", "c" }

print(minha_tabela[#minha_tabela])
minha_tabela[#minha_tabela + 1] = "d"
print(minha_tabela[#minha_tabela])
```

</div>


### Casos extremos

O operador `#` só é confiável quando as chaves de uma tabela são os números inteiros `1, 2, 3, ...` sem lacunas a partir de `1`. Chaves que são strings ou booleanos não são contadas, e se faltar um número no meio, o resultado é imprevisível:

<div class="runner">

```lua
-- Confiável: as chaves são 1, 2, 3
print(#{ "a", "b", "c" }) -- 3
print(#{ [1] = "a", [2] = "b", [3] = "c" }) -- 3

-- Não contadas: as chaves são strings
print(#{ ["a"] = 1, ["b"] = 2, ["c"] = 3 }) -- 0

-- Não confiável: há uma lacuna no índice 2
print(#{ [1] = "a", [3] = "b", [4] = "c" }) -- 1 ou 4, não confie nisso
```

</div>
