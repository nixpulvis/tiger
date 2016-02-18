pub mod ast;
pub mod tiger;

fn compile(source: &str) -> Box<ast::Expression> {
    match tiger::parse_Expression(source) {
        Ok(ast) => ast,
        Err(e) => {
            println!("{:?}", e);
            Box::new(ast::Expression::Nil)
        }
    }
}

fn main() {
    println!("{:?}", compile("nil"));
    println!("{:?}", compile("break"));
    println!("{:?}", compile("123"));
    println!("{:?}", compile(r#""hello""#));
    println!("{:?}", compile("foo"));
    println!("{:?}", compile("foo.bar"));
    println!("{:?}", compile("foo[(1;2)]"));
    println!("{:?}", compile("foo.bar[(1;2)]"));
    println!("{:?}", compile("if 1 then 2"));
    println!("{:?}", compile("()"));
    println!("{:?}", compile("(1)"));
    println!("{:?}", compile("(1; 2)"));
    println!("{:?}", compile("foo(1, 2)"));
    println!("{:?}", compile("posn { x=1, y=2 }"));
    println!("{:?}", compile("foo := nil"));
    println!("{:?}", compile("while 1 do if 2 then break"));
    println!("{:?}", compile("let type a = int in 2 end"));
    println!("{:?}", compile("let var a := 1 in a end"));
    println!("{:?}", compile("let var a : int := 1 in a end"));
    println!("{:?}", compile("let function foo(a:int) = () in a end"));
    println!("{:?}", compile("let function foo(a:int) : int = 1 in a end"));
    println!("{:?}", compile("foo #5# of 0"));
    println!("{:?}", compile("-5"));
}
