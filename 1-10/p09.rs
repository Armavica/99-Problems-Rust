fn pack<T: Clone+Eq>(list: &[T]) -> ~[~[T]] {
    let mut it = list.iter();
    let mut result = ~[];
    let mut l = 1;
    loop {
        match it.nth(l - 1) {
            Some(e) => {
                let mut slice = ~[];
                slice.push(e.clone());
                for f in it.take_while(|&a| *a == *e) {
                    slice.push(f.clone());
                }
                l = slice.len();
                result.push(slice);
            },
            None    => break
        }
    }
    result
}

fn main() {
    let list =
        ~['a', 'a', 'a', 'a', 'b', 'c', 'c', 'a', 'a', 'd', 'e', 'e', 'e', 'e'];

    println!("{:?}", pack(list));
}
