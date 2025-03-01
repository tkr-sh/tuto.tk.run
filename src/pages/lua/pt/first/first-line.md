Essa é a primeira linha que vemos no programa.

Normalmente, quando você quer entender o que um programa está fazendo, você irá lê-lo linha por linha, como se estivesse lendo um texto em inglês normal, então começamos aqui.


## Ok, mas o que essa linha está fazendo ?

Em linguagens de programação, você frequentemente tem o conceito de uma "variável". Uma variável é como uma caixa etiquetada que contém alguns dados. Uma vez que você crie uma variável, você pode ler o conteúdo que ela tem - equivalente a abrir uma caixa - e você até pode mudá-lo (já que o nome é "variável") - equivalente a colocar um novo valor na caixa.

### Mas quais dados você pergunta ?

Em Lua existem 6[^1] tipos de dados:
- `number` (número): Um número como em matemática! Você pode declarar um número apenas escrevendo-o: `123`.<br/> **Nota**: para números com decimais (por exemplo, `0.5`), use `.` (ponto) e não `,` (vírgula)
- `boolean` (booleano): Ou `true` ou `false`. (por exemplo, "2 é igual a 3?" => `false`)
- `nil` (nulo): Vazio. O objetivo de `nulo` é dizer que você não tem um valor.
- `string` (cadeia de caracteres): Um grupo de letras, números ou símbolos (por exemplo, `"Olá!"`). Para declarar uma cadeia de caracteres, você precisa colocar aspas (`'` ou `"` ) ao redor do conteúdo (por exemplo, para ter a cadeia de caracteres `"Olá!"`, você deve escrever `"Olá!"`)
- `table` (tabela): Vamos ver isso mais tarde!
- `function` (função): Vamos ver isso mais tarde!

Quando você quer atribuir alguns dados a uma variável, você simplesmente precisa escrever:
```lua
o_nome_da_variavel = o_valor
```
`o_nome_da_variavel` pode ser qualquer coisa. Exceto `true`, `false`, `nil` (e outros que vamos ver mais tarde)

`o_valor` um tipo válido como descrito acima.

Aqui estão alguns exemplos:
```lua
um_numero_negativo = -1
um_grande_numero = 100000000
nome_do_avô = "João"
sem_valor = nil
está_chovendo = false
```

Você também pode atribuir o conteúdo de uma variável a outra variável:

```lua
nome_do_avô = "João"
meu_amigo_avô_nome = nome_do_avô
```

Nesse contexto, tanto `nome_do_avô` quanto `meu_amigo_avô_nome` serão a cadeia de caracteres `"João"`.

## Voltando à nossa primeira linha

```lua
numero_de_tentativas = 0
```

Então, essa linha está apenas colocando o número `0` na variável `numero_de_tentativas`. E isso faz sentido! Porque no início do programa, o usuário ainda não tentou adivinhar o número! E como o nome `numero_de_tentativas` indica, essa variável conterá no futuro o número de tentativas que o usuário fez!



[^1]: 

<div class="for-technical">

Bem, na realidade existem 8 tipos, mas por agora apenas 6 são importantes :). <https://www.lua.org/pil/2.html>

</div>
