fn factorial(x: i64)
{
    match x
    {
        0 => 1,
        _ => x * factorial(x - 1)
    }
}

fn double_factorial(x: i64, y: i64)
{
    factorial(x) * factorial(y)
}
