extern crate tiger_syntax as syntax;
extern crate tiger;

use tiger::env::Env;
use tiger::trans::Translate;

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
    let mut tenv = Env::default();
    let mut venv = Env::default();

    // let ast = syntax::parse(SOURCE).unwrap();
    // println!("{:#?}", ast);

    let ast = syntax::parse(r###"
let
    type a = b
    type b = int
    var foo : a := 1

in
    foo
end
    "###).unwrap();
    println!("{:#?}", ast);

    // Translate the AST.
    let trans = ast.translate(&mut tenv, &mut venv);
    println!("{:?}", trans);
}
