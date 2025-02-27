Este é o primeiro programa que estudaremos! Não se preocupe, vamos passar por ele MUITO lentamente, então está tudo bem se você não entender nada ainda!

```lua
numero_de_tentativas = 0
numero_para_adivinhar = random(1, 100)

print("Tente adivinhar o número aleatório entre 1 e 100!")

repeat
        chute_do_usuario = tonumber(read())
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

O que este programa faz é criar o jogo de "encontre o número":

```
Tente adivinhar o número aleatório entre 1 e 100!
50
O número é mais alto!
75
O número é mais baixo!
62
Você ganhou em 3 tentativas!
```
