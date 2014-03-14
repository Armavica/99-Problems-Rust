#[deriving(Eq)]
enum Node<T> {
    One(T),
    Many(uint, T)
}

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

fn encode<T: Clone>(list: ~[~[T]]) -> ~[Node<T>] {
    list.map(|e|
             match e.len() {
                 1 => One(e[0].clone()),
                 n => Many(n, e[0].clone())
             })
}


fn main() {
    let list =
        ~['a', 'a', 'a', 'a', 'b', 'c', 'c', 'a', 'a', 'd', 'e', 'e', 'e', 'e'];

    assert!(encode(pack(list)) == ~[Many(4, 'a'),
                                    One('b'),
                                    Many(2, 'c'),
                                    Many(2, 'a'),
                                    One('d'),
                                    Many(4, 'e')])
}
