// <https://bartoszmilewski.com/2014/12/05/categories-great-and-small/>

#![allow(unused)]

trait Monoid<T> {
    fn mempty(v: T) -> T;

    fn mappend(v1: T, v2: T) -> T;
}

struct AddM<T>(T, T);

impl Monoid<u32> for AddM<u32> {
    fn mempty(v: u32) -> u32 {
        0
    }

    fn mappend(v1: u32, v2: u32) -> u32 {
        v1 + v2
    }
}
