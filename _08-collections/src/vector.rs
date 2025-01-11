pub fn run() {
    let arr = [1, 2, 3];
    let mut v1 = Vec::from(arr);
    let v2 = vec![1, 2, 3];

    assert_eq!(v1, v2);

    let v3 = vec![2, 3, 4, 5, 6];

    for i in 0..5 {
        match v1.get(i) {
            Option::Some(n) => v1[i] = n + 1,
            Option::None => v1.push(i + 2)
        }
    }

    assert_eq!(v1, v3);
}