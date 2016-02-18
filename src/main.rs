pub mod syntax;

fn main() {
    println!("{:?}", syntax::compile("nil"));
    println!("{:?}", syntax::compile("break"));
    println!("{:?}", syntax::compile("123"));
    println!("{:?}", syntax::compile(r#""hello""#));
    println!("{:?}", syntax::compile("foo"));
    println!("{:?}", syntax::compile("foo.bar"));
    println!("{:?}", syntax::compile("foo[(1;2)]"));
    println!("{:?}", syntax::compile("foo.bar[(1;2)]"));
    println!("{:?}", syntax::compile("if 1 then 2"));
    println!("{:?}", syntax::compile("()"));
    println!("{:?}", syntax::compile("(1)"));
    println!("{:?}", syntax::compile("(1; 2)"));
    println!("{:?}", syntax::compile("foo(1, 2)"));
    println!("{:?}", syntax::compile("posn { x=1, y=2 }"));
    println!("{:?}", syntax::compile("foo := nil"));
    println!("{:?}", syntax::compile("while 1 do if 2 then break"));
    println!("{:?}", syntax::compile("let type a = int in 2 end"));
    println!("{:?}", syntax::compile("let var a := 1 in a end"));
    println!("{:?}", syntax::compile("let var a : int := 1 in a end"));
    println!("{:?}", syntax::compile("let function foo(a:int) = () in a end"));
    println!("{:?}", syntax::compile("let function foo(a:int) : int = 1 in a end"));
    println!("{:?}", syntax::compile("foo #5# of 0"));
    println!("{:?}", syntax::compile("-5"));
}
