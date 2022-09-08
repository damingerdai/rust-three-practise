use std::collections::HashMap;

fn main() {
    let mut a = [10, 30, 20, 30, 40, 50, 15, 20, 30];
    let v = test_one_logic(&mut a);
    println!("{:?}", v);
}

fn test_one_logic(a: &mut [i32; 9]) ->  Vec<i32> {
    let mut v: Vec<i32> = Vec::new();
    let mut middle_len = a.len() / 2;
    if a.len() & 2 == 0 {
        middle_len += 1;
    }
    let mut map = HashMap::new();
    for i in 0..a.len() {
        let count = map.entry(a[i]).or_insert(0);
        *count += 1;

        for j in 0..a.len() - i - 1 {
            if a[j] > a[j + 1] {
                a[j] = a[j] ^ a[j + 1];
                a[j + 1] = a[j] ^ a[j + 1];
                a[j] = a[j] ^ a[j + 1];
            }
        }
    }
    v.push(a[middle_len]);
    let mut mode_value = 0;
    let mut mode = a[0];
    for (key, value) in map {
        if value > mode_value {
            mode_value = value;
            mode = key;
        }
    }
    v.push(mode);

    return v;
}