for page in $(fd -t f . en/); do
    echo "$page"
    prompt='You'\''re a profesionnal translater.
You need to traduct the following text from english to French.
The following text is an introduction to the Lua programming language. Please, stay focus on the original traduction.

What needs to be translated in the code:
- The variable names
- The strings
What MUSNT be translated:
- keywords (if, then, until, while, end, etc)
- nil
- false/true
- builtin-functions like `print`, `read`, `random` etc

Here is the following text to be translated

-----
'$(cat "$page")
    name_of_new="$(sed 's/\ben\b/fr/g' <<< "$page")"
    mkdir -p "$(dirname $name_of_new)"
    ollama run llama3.3:70b "$prompt" | tee $name_of_new
done
