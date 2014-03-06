fn gray(n: uint) -> ~[~str] {
    match n {
        0 => ~[],
        1 => ~[~"0", ~"1"],
        _ => {
            let prev = gray(n-1);
            let zero = prev.iter().map(|s| (~"0").append(*s));
            let one = prev.rev_iter().map(|s| (~"1").append(*s));
            zero.chain(one).collect()
        }
    }
}

fn main() {
    assert!(gray(1) == ~[~"0", ~"1"]);
    assert!(gray(2) == ~[~"00", ~"01", ~"11", ~"10"]);
    assert!(gray(3) == ~[~"000", ~"001", ~"011", ~"010", ~"110", ~"111", ~"101", ~"100"]);
}
