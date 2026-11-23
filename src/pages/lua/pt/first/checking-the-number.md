Agora que sabemos como:

1. gerar um número aleatório
2. obter a entrada do usuário
3. fazer laços

Precisamos ajudar o usuário dizendo se o palpite dele é menor ou maior que o número secreto.

Para isso, vamos usar uma nova palavra-chave que você vai usar com frequência: `if`

O que `if` faz é basicamente executar código dependendo de uma condição. Os laços verificam uma condição para continuar repetindo, e `if` verifica uma condição apenas para executar uma vez.

Então, por exemplo, podemos verificar se o usuário digita `42` e, se sim, podemos saudá-lo com uma mensagem especial.
<div class="runner">

```lua
numero_bom = 42

if numero_bom == tonumber(read()) then
    print("Boa escolha!")
end
```

</div>

Então, no nosso jogo

```lua
if numero_para_adivinhar < chute_do_usuario then
    print("O número é mais baixo!")
end

if numero_para_adivinhar > chute_do_usuario then
    print("O número é mais alto!")
end
```

verificamos se o número é mais baixo ou mais alto e, dependendo disso, exibimos uma mensagem para ajudar o usuário.

Se o usuário encontrar o número correto, o laço vai parar por causa de:

```lua
until chute_do_usuario == numero_para_adivinhar
```

<div class="runner">

```lua
numero_para_adivinhar = 42
chute_do_usuario = tonumber(read())

if numero_para_adivinhar < chute_do_usuario then
    print("O número é mais baixo!")
end

if numero_para_adivinhar > chute_do_usuario then
    print("O número é mais alto!")
end

if numero_para_adivinhar == chute_do_usuario then
    print("Correto!")
end
```

</div>

E, com isso feito, só precisamos criar um bom final...
