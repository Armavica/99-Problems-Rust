enum Group<T> {
    One(T),
    Many(uint, T)
}

fn decode<T: Clone>(list: &[Group<T>]) -> ~[T] {
    list.flat_map(|e|
                match *e {
                    One(ref a)     => {
                        let l = ~[a.clone()];
                        l
                    }
                    Many(n, ref a) => {
                        let mut l = ~[];
                        for _ in range(0, n) { l.push(a.clone()) }
                        l
                    }
                })
}

fn main() {
    let list =
~[Many(4, 'a'), One('b'), Many(2, 'c'), Many(2, 'a'), One('d'), Many(4, 'e')];

    println!("{:?}", decode(list));
}
