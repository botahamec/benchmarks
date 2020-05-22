const ARRAY: [u8; 18] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17];

fn binary_search(item: u8) -> i8 {
    fn search(item: u8, left: usize, right: usize) -> i8 {
        let mid = (left + right) / 2;

        if right < left {
            -1
        } else if ARRAY[mid] == item {
            mid as i8
        } else if ARRAY[mid] > item {
            search(item, left, mid - 1)
        } else {
            search(item, mid + 1, right)
        }
    }

    search(item, 0, ARRAY.len())
}

fn main() {
    for i in 0..ARRAY.len() {
        println!("{}", binary_search(i as u8));
    }
}
