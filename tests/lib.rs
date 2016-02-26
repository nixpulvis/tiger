extern crate tiger;
extern crate tiger_syntax as syntax;

use tiger::env::Env;
use tiger::ty::Type;
use tiger::trans::Translate;

mod fixtures {
    use std::io::prelude::*;
    use std::path::Path;
    use std::fs::File;
    use syntax;
    use syntax::ast::Expression;

    pub fn parse<P: AsRef<Path>>(name: P) -> Box<Expression> {
        let path = Path::new("fixtures").join(name);
        let mut file = File::open(path)
            .expect("could not open fixture");
        let mut source = String::new();
        file.read_to_string(&mut source)
            .expect("error reading fixture");
        syntax::parse(&source)
            .expect("error parsing fixture")
    }
}

#[test]
fn test_translate() {
    let tenv = Env::default();
    let venv = Env::default();
    let ast = fixtures::parse("merge.tig");
    let trans = ast.translate(&tenv, &venv);
    assert_eq!(Type::Unit, trans.ty);
}
