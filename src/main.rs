extern crate tiger_syntax as syntax;
extern crate tiger;

use tiger::ty;

const SOURCE: &'static str = r###"
(nil;
 123;
 -123;
 foo();
 bar(i);
 concat(chr(3), chr(2));
 "hello";
 myFirstTestRecord {};
 mySecondTestRecord { name="derp" };
 myThirdTestRecord { name="derp", age=1 };
 someVar := 1;
 someRecord.name := "derp";
 someField[1] := "lvalue";
 someList[0].someField[1] := "complex lvalue";
 if "b" then "c";
 while 1 do bar();
 for i := 0 to 10 do foo();
 while 1 do break)
"###;

fn main() {
    // let ast = syntax::compile(SOURCE);
    // let ast = syntax::compile("if 1 then 2 else nil");
    // println!("{:?}", ast);
    // let ast = syntax::compile("if 1 then 2");
    // println!("{:?}", ast);
    // let ast = syntax::compile("1 + 2 * 3");
    // println!("{:?}", ast);
    // let ast = syntax::compile("a #1# of 4");
    // println!("{:?}", ast);
    let ast = syntax::compile("1 | 2");
    println!("{:?}", ast);
    // let trans = ty::translate(&*ast);
    // println!("{:?}", trans);
}
