fn encode<T: Clone+Eq>(list: &[T]) -> ~[(uint, T)] {
    let mut it = list.iter();
    let mut result = ~[];
    let mut l = 1;
    loop {
        match it.nth(l - 1) {
            Some(e) => {
                l = it.take_while(|&a| *a == *e).len() + 1;
                result.push((l, e.clone()))
            },
            None    => break
        }
    }
    result
}

fn main() {
    let list =
        ~['a', 'a', 'a', 'a', 'b', 'c', 'c', 'a', 'a', 'd', 'e', 'e', 'e', 'e'];

    println!("{:?}", encode(list));
}
