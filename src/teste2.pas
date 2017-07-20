program lista02exercicio27(fezes);
var a,b,i,limite:Integer;
begin
writeln ('Digite um numero inteiro');
readln(a);

for i:= 1 to 10 do
begin
	while (a>i) do
	begin
		b:= b + a;
		a:= a - 1;
	end;
end;
i:=0;
limite:=5;
repeat
	i:=i+1;
until (i > limite);
if (i < a and i < b) then
	writeln(i,  '< ', a, ' e i < ', b);
else
begin
	if (i < a) then
		writeln(i, ' < ', a, ' e i > ', b);
	else 
		writeln(i, ' > ', a, ' ou i > ', b);
end;
i := i + 1;
end.
