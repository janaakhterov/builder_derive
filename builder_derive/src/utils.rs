use proc_macro2::TokenStream as TS;

pub(crate) fn unzip(vec: Vec<(TS, TS, TS, TS)>) -> (Vec<TS>, Vec<TS>, Vec<TS>, Vec<TS>) {
    let mut first: Vec<TS> = Vec::new();
    let mut second: Vec<TS> = Vec::new();
    let mut third: Vec<TS> = Vec::new();
    let mut forth: Vec<TS> = Vec::new();

    for (a, b ,c ,d) in vec {
        first.push(a);
        second.push(b);
        third.push(c);
        forth.push(d);
    }

    (first, second, third, forth)
}
