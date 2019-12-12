
use std::convert::TryFrom;
use std::result;
use std::fmt::Debug;
use std::slice::Iter;

#[derive(PartialEq, Eq, PartialOrd, Debug)]
pub struct NonEmpty<T>(Vec<T>);

impl<T> NonEmpty<T> {
    pub fn try_new(v: Vec<T>) -> result::Result<Self, Vec<T>> {
        if v.is_empty() {
            Err(v)
        } else {
            Ok(NonEmpty(v))
        }
    }

    pub fn get_vec(&self) -> &Vec<T> {
        &self.0
    }

    pub fn new(first: T, mut rest: Vec<T>) -> NonEmpty<T> {
        rest.insert(0, first);
        NonEmpty(rest)
    }

    pub fn single(first: T) -> NonEmpty<T> {
        NonEmpty(vec![first])
    }

    pub fn iter(&self) -> Iter<T> {
        self.0.iter()
    }

    pub fn new_or_err<E>(xs: Vec<T>, err: E) -> result::Result<Self, E> {
        if xs.is_empty() {
            Err(err)
        } else {
            Ok(NonEmpty(xs))
        }
    }

}

impl<T> TryFrom<Vec<T>> for NonEmpty<T> {
    type Error = Vec<T>;

    fn try_from(xs: Vec<T>) -> result::Result<Self, Self::Error> {
        NonEmpty::try_new(xs)
    }
}

impl<T> From<(T, Vec<T>)> for NonEmpty<T> {
    fn from((x, xs): (T, Vec<T>)) -> Self {
        NonEmpty::new(x, xs)
    }
}

impl<T> From<T> for NonEmpty<T> {
    fn from(x: T) -> Self {
        NonEmpty::single(x)
    }
}
