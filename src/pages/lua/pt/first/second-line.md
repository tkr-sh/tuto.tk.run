```lua
numero_para_adivinhar = random(1, 100)
```

Ok, então acabamos de ver que você pode colocar alguns dados em uma variável... mas o que é isso?

Para entender o que essa linha está fazendo, precisamos entender as `function`s. Esse era um dos tipos de dados que não expliquei durante a primeira linha, então aqui vai:

Você pode ver as funções como uma espécie de máquina que recebe algumas entradas, faz algo com elas e então devolve uma saída.

## Mas o que ela faz com elas?

Bem, em Lua existem alguns operadores que permitem fazer coisas especiais:

- `+`: Soma dois números. (por exemplo: `1 + 2` => `3`)
- `-`: Subtrai um número de outro. (por exemplo: `3 - 2` => `1`)
- `*`: Multiplica dois números. (por exemplo: `3 * 2` => `6`)
- `/`: Divide um número por outro. (por exemplo: `6 / 2` => `3`)

Portanto, podemos fazer as seguintes coisas com nossas variáveis:
```lua
o_melhor_numero = 40 + 2
idade_do_meu_avo = o_melhor_numero * 2
```
`o_melhor_numero` será 42 e `idade_do_meu_avo` será 84.

## Voltando às nossas funções!

Por exemplo, uma função pode receber um número e retornar o número `+ 1`.

Uma função, assim como uma variável, pode ser usada quando você sabe seu nome. MAS! Quando você quer que ela faça seu trabalho e retorne a saída, você precisa colocar parênteses após o nome (por exemplo: `minha_funcao()`). Isso executará uma função chamada `minha_funcao`. Mas essa função não tem nenhuma entrada! Para adicionar uma entrada a ela, você pode colocar os valores que deseja entre os parênteses e separá-los por `,`.

### Exemplo

```lua
dia = 4
mes = "Abril"
ano = 1984
calcular_data_de_amanha(dia, mes, ano)
```
que é o mesmo que
```lua
calcular_data_de_amanha(4, "Abril", 1984)
```

## Voltando à nossa segunda linha

```lua
numero_para_adivinhar = random(1, 100)
```

Então, essa linha está executando a função `random` com as entradas: `1` e `100`, e colocando a saída do cálculo em `numero_para_adivinhar`. Nós nunca definimos o que `random` fazia. Mas Lua inclui algumas funções «integradas» para ajudar os desenvolvedores! E no nosso caso, existe uma função `random` que recebe dois números como entrada e que gera um número aleatório entre o primeiro número (no nosso caso `1`) e o segundo número (no nosso caso `100`).

Perfeito! Agora já geramos o número que o usuário precisa adivinhar!
