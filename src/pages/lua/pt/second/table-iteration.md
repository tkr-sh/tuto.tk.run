Outra chave para trabalhar com tabelas é percorrê-las. Digamos que temos, no nosso jogo, a tabela `scores` (sendo um `score` o `numero_de_tentativas` que um usuário precisou para encontrar o número). Como mostrar cada pontuação de cada partida?

## `for` com índice
Primeiro, você talvez se lembre da palavra-chave `for` sobre a qual falamos rapidamente antes:

<blockquote>

`for`: o último loop é o loop `for`, e ele é um pouco diferente, porque você já sabe quantas vezes quer iterar. E você define de qual número até qual número quer iterar. No final de cada iteração, ele incrementará automaticamente `age` em `1`!

<div class="runner">

```lua
for age = 0, 3 do
    print(age)
end
```
</div>

</blockquote>

Se ligarmos isso ao nosso novo conhecimento do operador `#`, podemos percorrer uma tabela com sucesso!

<div class="runner">

```lua
scores = {
    [1] = 7,
    [2] = 8,
    [3] = 4,
}

for chave = 1, #scores do
    print(chave .. ": " .. scores[chave])
end
```
</div>

Isso funciona muito bem! Mas pode ser um pouco redundante escrever sempre `scores[chave]` para acessar o valor, especialmente se você não se importa com a chave ou se não pretende usar mais a tabela. Felizmente, o Lua tem outra forma de fazer isso.

## `ipairs`

`ipairs` é uma função que usaremos com a palavra-chave `for` para acessar rapidamente a chave e o valor sem ter que referenciar o nome da tabela! Sua sintaxe é um pouco diferente do loop `for` anterior, embora ainda use a palavra-chave `for`:

```lua
for minha_chave, meu_valor in ipairs(minha_tabela) do 
    -- Faça algo!
end
```

Neste exemplo, supomos que temos uma `table` chamada `minha_tabela`. Cada linha de código entre o `for` e o `end` será executada com `minha_chave` e `meu_valor` sendo a chave e o valor associado de `minha_tabela`.

Como o loop `for` anterior, ela é ordenada de forma ascendente. Ou seja, começa em `1` e aumenta em `1` a cada iteração. Mas não funciona quando as chaves são `string`s ou se houver lacunas entre os números. Se for esse o nosso caso, deveríamos usar `pairs`.

## `pairs`

`pairs` é como `ipairs`, com a diferença de que funciona com chaves que não são inteiros consecutivos a partir de `1`. A desvantagem é que a ordem em que as chaves chegam é imprevisível:

<div class="runner">

```lua
minha_tabela = {
    [3] = 7,
    ["name"] = 8,
    [false] = 4,
}

for chave, valor in pairs(minha_tabela) do
    print(chave)
    print(valor)
end
```

</div>

A ordem em que as chaves/valores são exibidas é imprevisível e pode variar de uma execução para outra. No entanto, tem a vantagem de nos permitir percorrer qualquer tabela!

## Voltando ao nosso jogo
No nosso caso, só queremos manter o `numero_de_tentativas` de cada partida. Portanto, não precisamos de chaves personalizadas: podemos simplesmente ter índices começando em `1` e aumentando em `1` a cada vez, correspondendo ao número da partida!
