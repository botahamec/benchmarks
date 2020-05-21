
let a = 0;
let b = 1;
let c = 0;
for (let i = 0; i < 100; i++) {
	let c = a + b;
	a = b;
	b = c;
	console.log(c);
}