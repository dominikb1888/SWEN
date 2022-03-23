set ignore (cat ignore); for name in (gh repo list DB-Teaching -L 150 --json name | jq -r '.[] | .name');if contains $name $ignore;echo . ;else; gh repo clone DB-Teaching/$name;end;end

