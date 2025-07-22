pub type List = Box<dyn Fn(usize) -> usize>;

#[allow(non_snake_case)]
pub fn E(n: usize) -> usize {
    2*n
}

#[allow(non_snake_case)]
pub fn O(n: usize) -> usize {
    2*n - 1
}

#[allow(non_snake_case)]
pub fn T(mut n: usize) -> usize {
    for i in 1_usize.. {
        if n > i {
            n -= i;
        } else {
            return i;
        }
    }

    unreachable!()
}

pub fn combined(l1: List, l2: List) -> List {
    Box::new(move |n| l2(l1(l2(n))))
}

