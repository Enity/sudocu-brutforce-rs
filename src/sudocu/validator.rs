pub fn check_not_exists(value: u8, indexes_arr: &[usize], arr: &[u8]) -> bool {
    for i in indexes_arr {
        if arr[*i] == value {
            return false
        }
    }
    true
}