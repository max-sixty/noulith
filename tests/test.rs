extern crate noulith;
// use std::rc::Rc;
use noulith::Obj;
use noulith::simple_eval;

#[test]
fn demos() {
    assert_eq!(simple_eval("fact := \\n: if (n == 0) 1 else n * fact(n - 1); fact 10"), Obj::from(3628800));
    assert_eq!(simple_eval("(for (x : 1 to 15) yield (o := ''; for (f, s : [[3, 'Fizz'], [5, 'Buzz']]) if (x % f == 0) o $= s; if (o == '') x else o)) join ';'"), Obj::from("1;2;Fizz;4;Buzz;Fizz;7;8;Fizz;Buzz;11;Fizz;13;14;FizzBuzz"));
}

#[test]
fn quick_operators() {
    assert_eq!(simple_eval("2 + 3"), Obj::from(5));
    assert_eq!(simple_eval("+(2, 3)"), Obj::from(5));
    assert_eq!(simple_eval("plus := +; 2 plus 3"), Obj::from(5));
    assert_eq!(simple_eval("plus := \\x, y: x + y; 2 plus 3"), Obj::from(5));
}

#[test]
fn modifications() {
    assert_eq!(simple_eval("x := 2; x += 3; x"), Obj::from(5));
    assert_eq!(simple_eval("x := 2; x max= 3; x"), Obj::from(3));
    assert_eq!(simple_eval("x := 2; x .= +3; x"), Obj::from(5));
}

#[test]
fn weird_assignments() {
    assert_eq!(simple_eval("every x, y := 2; x + y"), Obj::from(4));
    assert_eq!(simple_eval("x := 1 to 10; every x[2:4] = -1; sum x"), Obj::from(46));
}

#[test]
fn ranges() {
    assert_eq!(simple_eval("1 til 3 join ','"), Obj::from("1,2"));
    assert_eq!(simple_eval("1 to 3 join ','"), Obj::from("1,2,3"));
    assert_eq!(simple_eval("1 til 7 by 3 join ','"), Obj::from("1,4"));
    assert_eq!(simple_eval("1 to 7 by 3 join ','"), Obj::from("1,4,7"));
}

#[test]
fn evil_operators() {
    assert_eq!(simple_eval("2 + 5 * 3"), Obj::from(17));
    assert_eq!(simple_eval("+, * = *, +; 2 + 5 * 3"), Obj::from(13));
    assert_eq!(simple_eval("+['precedence'], *['precedence'] = *['precedence'], +['precedence']; 2 + 5 * 3"), Obj::from(21));
    assert_eq!(simple_eval("+, * = *, +; +['precedence'], *['precedence'] = *['precedence'], +['precedence']; 2 + 5 * 3"), Obj::from(16));
}

#[test]
fn math() {
    assert_eq!(simple_eval("7 + 4"), Obj::from(11));
    assert_eq!(simple_eval("7 - 4"), Obj::from(3));
    assert_eq!(simple_eval("7 * 4"), Obj::from(28));
    assert_eq!(simple_eval("7 / 4"), Obj::from(1.75));
    assert_eq!(simple_eval("7 % 4"), Obj::from(3));
    assert_eq!(simple_eval("7 // 4"), Obj::from(1));
    assert_eq!(simple_eval("7 %% 4"), Obj::from(3));
}

#[test]
fn bitwise() {
    assert_eq!(simple_eval("7 & 4"), Obj::from(4));
    assert_eq!(simple_eval("7 | 4"), Obj::from(7));
    assert_eq!(simple_eval("7 @ 4"), Obj::from(3));
    assert_eq!(simple_eval("7 << 4"), Obj::from(112));
    assert_eq!(simple_eval("7 >> 4"), Obj::from(0));
}

#[test]
fn len() {
    assert_eq!(simple_eval("len([2, 5, 3])"), Obj::from(3));
    assert_eq!(simple_eval("len(1 to 10)"), Obj::from(10));
}

#[test]
fn lists() {
    assert_eq!(simple_eval("[1, 2] ++ [3, 4] join ''"), Obj::from("1234"));
    assert_eq!(simple_eval("[1, 2, 3] +. 4 join ''"), Obj::from("1234"));
    assert_eq!(simple_eval("1 .+ [2, 3, 4] join ''"), Obj::from("1234"));
    assert_eq!(simple_eval("[1, 2] ** 3 join ''"), Obj::from("121212"));
}

#[test]
fn indexing() {
    assert_eq!(simple_eval("(1 to 10)[2]"), Obj::from(3));
    assert_eq!(simple_eval("(1 to 10)[-2]"), Obj::from(9));
    assert_eq!(simple_eval("(1 to 10)[2:4] join ''"), Obj::from("34"));
    assert_eq!(simple_eval("x := 1 to 10; x[2] = 4; x[2]"), Obj::from(4));
    assert_eq!(simple_eval("x := [[0] ** 10] ** 10; x[1][2] = 3; x[2][2] = 4; x[1][2] $ x[2][2]"), Obj::from("34"));
}

#[test]
fn dicts() {
    assert_eq!(simple_eval("len({1, 2} || {3, 4})"), Obj::from(4));
    assert_eq!(simple_eval("len({1, 2} || {3, 2})"), Obj::from(3));
    assert_eq!(simple_eval("len({1, 2} && {3, 4})"), Obj::from(0));
    assert_eq!(simple_eval("len({1, 2} && {3, 2})"), Obj::from(1));
    assert_eq!(simple_eval("len({1, 2} |. 3)"), Obj::from(3));
    assert_eq!(simple_eval("len({1, 2} |. 2)"), Obj::from(2));
    assert_eq!(simple_eval("{1: 2, 3: 4}[1]"), Obj::from(2));
    assert_eq!(simple_eval("{:5, 1: 2, 3: 4}[1]"), Obj::from(2));
    assert_eq!(simple_eval("{:5, 1: 2, 3: 4}[2]"), Obj::from(5));
}

#[test]
fn fast_append_pop() {
    assert_eq!(simple_eval("x := []; for (i : 1 to 10000) x append= i; y := 0; for (i : 1 to 10000) y += pop x; y"), Obj::from(50005000));
}

#[test]
fn short_circuit() {
    assert_eq!(simple_eval("3 or x"), Obj::from(3));
    assert_eq!(simple_eval("0 and x"), Obj::from(0));
    assert_eq!(simple_eval("0 or 4"), Obj::from(4));
    assert_eq!(simple_eval("3 and 4"), Obj::from(4));
}

#[test]
fn comparisons() {
    assert_eq!(simple_eval("1 == 1 == 1"), Obj::from(1));
    assert_eq!(simple_eval("0 == 0 == 1"), Obj::from(0));
    assert_eq!(simple_eval("1 < 2 == 2"), Obj::from(1));
    assert_eq!(simple_eval("1 < (2 == 2)"), Obj::from(0));
    assert_eq!(simple_eval("(1 < 2) == 2"), Obj::from(0));
    assert_eq!(simple_eval("1 < 2 < 3"), Obj::from(1));
    assert_eq!(simple_eval("3 > 2 > 1"), Obj::from(1));
    assert_eq!(simple_eval("(-1) < 2 < 2"), Obj::from(0));
    assert_eq!(simple_eval("((-1) < 2) < 2"), Obj::from(1));
    assert_eq!(simple_eval("(-1) < (2 < 2)"), Obj::from(1));
}
#[test]
fn minmax() {
    assert_eq!(simple_eval("3 min 4"), Obj::from(3));
    assert_eq!(simple_eval("3 max 4"), Obj::from(4));
    assert_eq!(simple_eval("min(3 to 5)"), Obj::from(3));
    assert_eq!(simple_eval("max(3 to 5)"), Obj::from(5));
}

#[test]
fn opassigns() {
    assert_eq!(simple_eval("x := 3; x += 4; x"), Obj::from(7));
    assert_eq!(simple_eval("x := 3; x min= 2; x"), Obj::from(2));
}

#[test]
fn for_loops() {
    assert_eq!(simple_eval("x := 0; for (y : 1 to 5) x += y + 1; x"), Obj::from(20));
    assert_eq!(simple_eval("sum (for (y : 1 to 5) yield y + 1)"), Obj::from(20));
    assert_eq!(simple_eval("x := 0; for (i, y :: 1 to 5) x += i * y; x"), Obj::from(40));
    assert_eq!(simple_eval("sum (for (i, x :: 1 to 5) yield i * x)"), Obj::from(40));
    assert_eq!(simple_eval("x := 0; for (y := 5) x += y + 1; x"), Obj::from(6));
}
