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

#[test]
fn should_return_multiple_numbers_when_is_not_prime() {
    assert_eq!(primefactor(4), [2, 2]);
    assert_eq!(primefactor(6), [2, 3]);
    assert_eq!(primefactor(8), [2, 2, 2]);
}



fn primefactor(number: u32) -> Vec<u32> {
    primefactor_rec(number, [].to_vec())
}

fn primefactor_rec(number: u32, factors: Vec<u32>) -> Vec<u32> {
    if number > 1 {
        if number % 2 == 0 {
            return primefactor_rec(number / 2, [factors, [2].to_vec()].concat());
        }
        return [factors, [number].to_vec()].concat();
    }
    factors
}