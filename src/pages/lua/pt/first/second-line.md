```lua
numero_para_adivinhar = random(1, 100)
```

Ok, então acabamos de ver que você pode colocar alguns dados em uma variável... mas o que é isso ???

Para entender o que essa linha está fazendo, precisamos entender `funções`. Essa era uma das tipos de dados que não expliquei durante a primeira linha, então aqui vai:

Você pode ver funções como uma espécie de máquina que leva algumas entradas, faz algo com elas e então retorna uma saída.

## Mas o que ela faz com elas ???

Bem, em Lua existem alguns operadores que permitem fazer coisas especiais:

- `+`: Soma dois números. (por exemplo: `1 + 2` => `3`)
- `-`: Subtrai um número de outro. (por exemplo: `3 - 2` => `1`)
- `*`: Multiplica dois números. (por exemplo: `3 * 2` => `6`)
- `/`: Divide um número por outro. (por exemplo: `6 / 2` => `3`)

Portanto, podemos fazer as seguintes coisas com nossas variáveis:
```lua
o_melhor_numero = 40 + 2
idade_do_meu_avô = o_melhor_numero * 2
```
`o_melhor_numero` será 42 e `idade_do_meu_avô` será 84.

## Voltando às nossas funções!

Por exemplo, uma função pode levar um número e retornar o número `+ 1`.

Uma função, assim como uma variável, pode ser usada quando você sabe seu nome. MAS! Quando você quer que ela faça sua "máquina" e retorne a saída, você precisa colocar parênteses após o nome (por exemplo: `minha_função()`). Isso executará uma função chamada `minha_função`. Mas essa função não tem nenhuma entrada! Para adicionar entrada a ela, você pode colocar os valores que deseja entre os parênteses e separá-los por `,`.

### Exemplo

```lua
dia = 4
mês = "Abril"
ano = 1984
calcular_data_de_amanhã(dia, mês, ano)
```
que é o mesmo que
```lua
calcular_data_de_amanhã(4, "Abril", 1984)
```

## Voltando à nossa segunda linha

```lua
numero_para_adivinhar = random(1, 100)
```

Então, essa linha está executando a função `aleatório` com as entradas: `1` e `100`, e colocando a saída do cálculo em `numero_para_adivinhar`. Nós nunca definimos o que `aleatório` fazia. Mas, Lua vem com algumas funções "internas" para ajudar desenvolvedores! E no nosso caso, há uma função `random` que quer dois números como entrada e que gera um número aleatório entre o primeiro número (no nosso caso `1`) e o segundo número (no nosso caso `100`).

Perfeito! Agora temos gerado o número que o usuário precisa adivinhar!
