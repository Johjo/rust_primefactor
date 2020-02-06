#[test]

fn should_return_empty_list_with_number_below_or_equal_to_one() {
    assert_eq!(primefactor(0), []);
    assert_eq!(primefactor(1), []);
}

fn primefactor(number: u32) -> Vec<u32> {
    [].to_vec()
}