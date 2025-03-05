```lua
chute_do_usuario = tonumber(read())
```

Ok, então esse código pode parecer estranho (uma função dentro de outra?), mas pode ser visto da seguinte forma:

```lua
valor_lido = read()
chute_do_usuario = tonumber(valor_lido)
```

O que acontece é o seguinte:
1. Lua quer calcular o valor à direita 
2. Lua quer executar `tonumber`, mas para isso, precisa que todas as suas entradas tenham valores
3. Portanto, quando vê `read()`, tenta rodá-la instantaneamente para que a função produza um valor
4. Depois disso, `read()` é executada e retorna um valor, e então `tonumber` pode ser executado!

### Ok, mas o que é que `read()` faz?

`read()` pede ao usuário uma string e a captura.

**Antes de rodar o exemplo:** não desative a janela pop-up para este site, caso contrário você não poderá fazer leituras novamente!



<div class="runner">

```lua
print(read())
```

</div>

### E o que é `tonumber`?

Já que `read()` pega uma string do usuário, não podemos compará-la com nosso número aleatório, porque não se pode comparar uma string que pode ter caracteres não numéricos com um número (por exemplo, `"x4%$A" > 3` não faz sentido)

Portanto, precisamos converter a entrada do usuário para um número e é exatamente isso que `tonumber` faz!

Essa função irá retornar `nil` se sua entrada não for um número válido.

<div class="runner">

```lua
print(tonumber("Não é um número") == nil)
```

</div>

E você não pode adicionar `nil` + `número`, então entre com um número válido!

<div class="runner">

```lua
print(tonumber(read()) + 1)
```

</div>

Irão ler a string do usuário, convertê-la para número e adicionar um a esse número, e se não for um número válido, `tonumber(read())` será um valor `nil`.
