Agora que sabemos como:

1. gerar um número aleatório
2. obter entrada do usuário
3. fazer laços

Precisamos ajudar o usuário dizendo se ele/ela está abaixo ou acima do número correto.

Para isso, vamos usar uma nova palavra-chave, que você encontrará frequentemente usando: `if`

O que `if` faz é basicamente executar código dependendo de uma condição. Laços verificam a condição para fazer laço, e `if` verifica a condição apenas para rodar uma vez.

Então, por exemplo, podemos verificar se o usuário digita `42`, e se sim, podemos saudá-lo/la com uma mensagem especial.
<div class="runner">

```lua
número_bom = 42

if número_bom == tonumber(read()) then
    print("Boa escolha!")
end
```

</div>

Então em nosso jogo

```lua
if número_para_adivinhar < chute_do_usuário then
    print("O número é mais baixo!")
end

if número_para_adivinhar > chute_do_usuário then
    print("O número é mais alto!")
end
```

verificamos se o número é mais baixo/mais alto, e dependendo disso, imprimimos uma mensagem para ajudar o usuário.

Se o usuário encontrar o número correto, irá parar o laço devido a:

```lua
until chute_do_usuário == número_para_adivinhar
```

<div class="runner">

```lua
número_para_adivinhar = 42
chute_do_usuário = tonumber(read())

if número_para_adivinhar < chute_do_usuário then
    print("O número é mais baixo!")
end

if número_para_adivinhar > chute_do_usuário then
    print("O número é mais alto!")
end

if número_para_adivinhar == chute_do_usuário then
    print("Correto!")
end
```

</div>

E com isso feito, apenas precisamos fazer um bom final...
