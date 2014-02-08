enum Bool_expr {
    Var(~str),
    Not(~Bool_expr),
    And(~Bool_expr, ~Bool_expr),
    Or(~Bool_expr, ~Bool_expr)
}

fn table2(a: ~str, b: ~str, e: Bool_expr) -> ~[(bool, bool, bool)] {
    match e {
        Var(ref v) if *v == a => ~[ (false, false, false),
                                    (false,  true, false),
                                    ( true, false,  true),
                                    ( true,  true,  true)],
        Var(ref v) if *v == b => ~[ (false, false, false),
                                    (false,  true,  true),
                                    ( true, false, false),
                                    ( true,  true,  true)],
        Var(v)                => fail!(format!("Variable `{}` unknown", v)),
        Not(~e)               => table2(a, b, e).move_iter().map(
                                     |(a, b, r)| (a, b, !r)
                                 ).to_owned_vec(),
        And(~e, ~f)           => table2(a.clone(), b.clone(), e).move_iter().zip(
                                 table2(a, b, f).move_iter()).map(
                                     |((a, b, i), (_, _, j))| (a, b, i && j)
                                 ).to_owned_vec(),
        Or(~e, ~f)            => table2(a.clone(), b.clone(), e).move_iter().zip(
                                 table2(a, b, f).move_iter()).map(
                                     |((a, b, i), (_, _, j))| (a, b, i || j)
                                 ).to_owned_vec()
    }
}

fn main() {
    println!("{:?}", table2(~"a", ~"b", And(~Var(~"a"), ~Or(~Var(~"a"), ~Var(~"b")))));
}
