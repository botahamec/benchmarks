
let a = 0;
let b = 1;
let c = 0;
for (let i = 0; i < 40; i++) {
	c = a + b;
	a = b;
	b = c;
	console.log(c);
}