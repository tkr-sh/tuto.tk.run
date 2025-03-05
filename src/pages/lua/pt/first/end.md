Então, agora a única coisa que resta é imprimir o número de tentativas.

Anteriormente, incrementamos isso logo após obter os dados do usuário com

```lua
número_de_tentativas = número_de_tentativas + 1
```

Mas agora, gostaríamos de imprimir uma mensagem de congratulações e ao mesmo tempo, quantas tentativas levou?

É hora de aprender o último operador antes do fim desse primeiro jogo:

`..`: Concatena 2 valores. 

<div class="runner">

```lua
print("Levou " .. 4 .. " anos")
```

</div>

Com isso em mente, podemos criar uma mensagem agradável:

```lua
print("Você ganhou em " .. número_de_tentativas .. " tentativas!")
```

E aqui está FINALMENTE nosso jogo:

---

<div class="runner">

```lua
número_de_tentativas = 0
número_para_adivinhar = random(1, 100)

print("Tente adivinhar o número aleatório entre 1 e 100!")

repeat
        chute_do_usuário = tonumber(read())
        número_de_tentativas = número_de_tentativas + 1

        if número_para_adivinhar < chute_do_usuário then
                print("O número é mais baixo!")
        end

        if número_para_adivinhar > chute_do_usuário then
                print("O número é mais alto!")
        end
until chute_do_usuário == número_para_adivinhar

print("Você ganhou em " .. número_de_tentativas .. " tentativas!")
```

</div>

---

## UAU.

Uau! Não é?

Você está pronto para se tornar um programador agora :) !

## É o fim ?

Sim e não.

Infelizmente, eu simplifiquei demais uma coisa: geralmente você não pode executar `read` e `random` assim. Lua gosta de colocar funções "embutidas" em categorias específicas: `math` (para funções matemáticas), `io` (para interações de entrada/saída), `os` (para interagir com o seu sistema operacional), ...

E, portanto, quando você quer usar funções específicas que fazem parte de uma categoria, você precisa chamá-las como `categoria.a_função()`

No nosso caso, `random` faz parte da categoria `math`, e `read` faz parte da categoria `io`. `print` e `tonumber` são globais, então você não precisa especificar nada

<div class="runner">

```lua
número_de_tentativas = 0
número_para_adivinhar = math.random(1, 100)

print("Tente adivinhar o número aleatório entre 1 e 100!")

repeat
        chute_do_usuário = tonumber(io.read())
        número_de_tentativas = número_de_tentativas + 1

        if número_para_adivinhar < chute_do_usuário then
                print("O número é mais baixo!")
        end

        if número_para_adivinhar > chute_do_usuário then
                print("O número é mais alto!")
        end
until chute_do_usuário == número_para_adivinhar

print("Você ganhou em " .. número_de_tentativas .. " tentativas!")
```

</div>

## Parabéns, você agora finalizou seu primeiro programa Lua! 🎉

Agora que você tem a base, vamos tentar melhorar nosso jogo atual para adicionar novas funcionalidades e aprender coisas novas pelo caminho!
