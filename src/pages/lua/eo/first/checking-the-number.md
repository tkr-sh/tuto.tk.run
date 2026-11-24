Nun, kiam ni scias kiel

1. generi hazardan nombron
2. akiri enigon de la uzanto
3. bukli

Ni devas helpi la uzanton dirante al li/ŝi, ĉu lia/ŝia diveno estas pli malgranda aŭ pli granda ol la sekreta nombro.

Por tio, ni uzos novan ŝlosilvorton, kiun vi ofte trovos vin uzanta: `if`

Kion `if` faras, estas esence plenumi kodon depende de kondiĉo. Bukloj kontrolas kondiĉon por bukli, kaj `if` kontrolas kondiĉon nur por plenumi unufoje.

Do ekzemple, ni povas kontroli, ĉu la uzanto enigas `42`, kaj se jes, ni povas saluti lin/ŝin per speciala mesaĝo.
<div class="runner">

```lua
good_number = 42

if good_number == tonumber(read()) then
    print("Bona elekto!")
end
```

</div>

Do en nia ludo

```lua
if number_to_guess < user_guess then
    print("La nombro estas pli malgranda!")
end

if number_to_guess > user_guess then
    print("La nombro estas pli granda!")
end
```

ni kontrolas, ĉu la nombro estas pli malgranda/pli granda, kaj depende de tio ni presas mesaĝon por helpi la uzanton.

Se la uzanto trovis la ĝustan nombron, la buklo haltos pro:

```lua
until user_guess == number_to_guess
```

<div class="runner">

```lua
number_to_guess = 42
user_guess = tonumber(read())

if number_to_guess < user_guess then
    print("La nombro estas pli malgranda!")
end

if number_to_guess > user_guess then
    print("La nombro estas pli granda!")
end

if number_to_guess == user_guess then
    print("Ĝuste!")
end
```

</div>

Kaj nun, kiam tio estas farita, ni nur devas fari bonan finon...
