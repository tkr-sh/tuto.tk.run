```lua
print("Tente adivinhar o número aleatório entre 1 e 100!")
```

Essa linha também é uma chamada à uma função! Nós podemos ver isso desde que temos o nome `print` mas com parênteses após o seu nome!

Essa função leva uma string como seu primeiro e único parâmetro. Mas... nós nunca colocamos a saída em uma variável.

## Por quê ?

Porque `print` não retorna um valor. `print` é uma função especial que exibe ao usuário os dados que ela recebe em sua entrada.

Por exemplo, vamos pegar de volta o nosso exemplo anterior:

```lua
o_melhor_numero = 40 + 2
idade_do_meu_avô = o_melhor_numero * 2
```

Como eu sei qual é o valor de `idade_do_meu_avô` ? Eu poderia adivinhar fazendo o cálculo na minha cabeça, mas quando o programa se torna mais complexo, isso não é sempre possível.

Portanto, nós podemos simplesmente fazer 

```lua
print(idade_do_meu_avô)
```

Que exibirá o conteúdo da variável `idade_do_meu_avô`. No nosso caso: `84`

## Agora que você sabe disso

Agora que você sabe dessa função muito útil, eu apresento a você o _"✨️**Executador de Lua**✨️"_ & _"✨️**Editor interativo de Lua**✨️"_, no qual você pode executar seus próprios programas:

<div class="runner">

```lua
print("Olá mundo!")
```

</div>

Você pode ver que isso é re-executado todas as vezes que você clica em "Executar" com esse exemplo

<div class="runner">

```lua
print(random(1, 100))
```

</div>

E finalmente, nós podemos ter certeza de que sabemos a idade do nosso avô corretamente:

<div class="runner">

```lua
o_melhor_numero = 40 + 2
print(o_melhor_numero)
idade_do_meu_avô = o_melhor_numero * 2
print(idade_do_meu_avô)
```

</div>
