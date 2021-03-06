
pub fn min<T: PartialOrd>(a: T, b: T) -> T
{
    if a < b { a } else { b }
}

pub fn max<T: PartialOrd>(a: T, b: T) -> T
{
    if a < b { b } else { a }
}
