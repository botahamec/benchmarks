fn main() {

    let array : Vec<usize> = vec![
    	86,
        13,
        82,
        94,
        3,
        74,
        77,
        26,
        97,
        24,
        32,
        50,
        75,
        58,
        47,
        2,
        37,
        36,
        53,
        60,
        31,
        42,
        62,
        73,
        54,
        95,
        98,
        55,
        7,
        27,
        14,
        1,
        11,
        19,
        33,
        68,
        87,
        4,
        67,
        63,
        66,
        93,
        84,
        23,
        59,
        71,
        41,
        28,
        64,
        16,
        61,
        5,
        15,
        85,
        35,
        69,
        0,
        57,
        79,
        25,
        88,
        38,
        34,
        65,
        18,
        48,
        49,
        12,
        92,
        8,
        44,
        72,
        81,
        9,
        29,
        70,
        21,
        46,
        43,
        90,
        83,
        10,
        91,
        22,
        39,
        96,
        80,
        45,
        51,
        6,
        89,
        14,
        56,
        20,
        78,
        17,
        76,
        40,
        52,
    	30,
    	99,
    ];

    let mut old_array = array.clone();
    let mut new_array : Vec<usize> = Vec::with_capacity(101);

    for _i in 0..(array.len() - 1) {
        let mut min_num = 1_000;
        let mut min_index = 1_000;
        for j in 0..old_array.len() {
            if old_array[j] < min_num {
                min_num = old_array[j];
                min_index = j;
            }
        }
        new_array.push(min_num);
        old_array.remove(min_index);
    }

    println!("{:#?}", new_array);
}
