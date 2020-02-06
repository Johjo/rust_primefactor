#[test]
fn should_return_empty_list_with_number_below_or_equal_to_one() {
    assert_eq!(primefactor(0), []);
    assert_eq!(primefactor(1), []);
}

#[test]
fn should_return_number_when_is_prime() {
    assert_eq!(primefactor(2), [2]);
    assert_eq!(primefactor(3), [3]);
}


fn primefactor(number: u32) -> Vec<u32> {
    if number > 1 {
        return [number].to_vec();
    }
    [].to_vec()
}