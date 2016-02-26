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

    // let ast = syntax::compile(SOURCE);
    // println!("{:#?}", ast);

    let ast = syntax::compile(r###"
let
    type a = string
    type b = array of a
    var buf : b := a #512# of ""
in
    buf[0]
end
    "###);
    println!("{:#?}", ast);

    // Translate the AST.
    let trans = ast.translate(&mut tenv, &mut venv);
    println!("{:?}", trans);
}
