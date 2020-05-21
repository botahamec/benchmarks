const ARRAY = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17];

function binary_search(item: number): number {
	function search(item: number, left: number, right: number): number {

		if (right < left) return -1;

		let mid = Math.floor((left + right) / 2);

		if (ARRAY[mid] == item) return mid;
		else if (ARRAY[mid] > item) return search(item, left, mid - 1);
		else return search(item, mid + 1, right);
	}
	return search(item, 0, 17);
}

for (let i = 0; i <= 17; i++) {
	console.log(binary_search(i));
}