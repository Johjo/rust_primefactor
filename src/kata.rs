pub fn primefactor(number: u32) -> Vec<u32> {
    primefactor_rec(number, [].to_vec(), 2)
}

fn primefactor_rec(number: u32, factors: Vec<u32>, candidate: u32) -> Vec<u32> {
    if number > 1 {
        if number % candidate == 0 {
            return primefactor_rec(number / candidate, [factors, [candidate].to_vec()].concat(), candidate);
        }
        return primefactor_rec(number, factors, candidate + 1);
    }
    factors
}
