/// Y-conbinator
/// ```
/// Y := λf.(
///             λx.(f(x)(x))
///             λx.(f(x)(x))
///         )
/// ```
///
/// Then pass
/// ```
/// Y f
/// = (λf.(λx.f(x x))(λx.f(x x))) f
/// = (λx.f(x x))(λx.f(x x))        // pass the later `(λx.f(x x))` to the former as `x` (*)
/// = f(
///     (λx.f(x x))(λx.f(x x))      // this line is same as (*) = (Y f)
///   )
/// = f (Y f)
/// ```
fn main() {
    struct Y<'y, T> {
        fix: &'y dyn Fn(&Y<T>, T) -> T,
    }

    let fibonacci = {
        let yc = Y {
            fix: &|y, n| {
                if n <= 1 {
                    n
                } else {
                    (y.fix)(y, n - 1) + (y.fix)(y, n - 2)
                }
            },
        };
        move |i| (yc.fix)(&yc, i)
    };

    let factorial = {
        let yc = Y {
            fix: &|y, n| if n == 0 { 1 } else { n * (y.fix)(y, n - 1) },
        };
        move |i| (yc.fix)(&yc, i)
    };
    assert_eq!(fibonacci(7), 13);
    assert_eq!(factorial(7), 5040);
}
