#![cfg_attr(test, feature(plugin))]
#![cfg_attr(test, plugin(quickcheck_macros))]

#[cfg(test)]
extern crate quickcheck;

pub fn reverse<T: Clone>(xs: &[T]) -> Vec<T> {
    let mut rev = vec![];
    for x in xs {
        rev.insert(0, x.clone())
    }
    rev
}

#[cfg(test)]
mod tests {
    use super::*;

    #[quickcheck]
    fn double_reversal_is_identity(xs: Vec<isize>) -> bool {
        xs == reverse(&reverse(&xs))
    }
}
