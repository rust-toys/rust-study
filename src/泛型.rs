fn find_last_number(list: &Vec<i32>) -> i32 {
    let mut result = list[0];

    for &item in list {
        if item > result {
            result = item;
        }
    }
    result
}
