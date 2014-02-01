fn rang(from: int, to: int) -> ~[int] {
    if to >= from {
        range(from, to+1).to_owned_vec()
    } else {
        range(to, from+1).rev().to_owned_vec()
    }
}

fn main() {
    println!("{:?}", rang(4, 9));
    println!("{:?}", rang(9, 4));
}

