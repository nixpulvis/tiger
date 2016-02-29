// use std::iter::Iterator;
use syntax::ast;
use ty::{Type, UnresolvedType};
use env::{Env, Value};
use trans::{Translate, Translation};

impl<'a> Translate for &'a [Box<ast::Declaration>] {
    type Prime = (Env<Type>, Env<Value>);

    fn translate(&self,
                 tenv: &Env<Type>,
                 venv: &Env<Value>) -> Self::Prime
    {
        let tenv: Env<UnresolvedType<'a>> = tenv.clone().into();
        let venv = venv.clone();
        let (tenv, venv) = self.into_iter().fold((tenv, venv), |(t, e), d| {
            Declaration(d).insert(&t, &e)
        });
        let (tenv, venv) = self.into_iter().fold((tenv, venv), |(t, e), d| {
            Declaration(d).fill(&t, &e)
        });
        let tenv: Env<Type> = tenv.clone().into();
        (tenv, venv)
    }
}

struct Declaration<'a>(&'a ast::Declaration);

impl<'a> Declaration<'a> {
    fn insert(&self,
                 tenv: &Env<UnresolvedType<'a>>,
                 venv: &Env<Value>) -> (Env<UnresolvedType<'a>>, Env<Value>)
    {
        match *self.0 {
            ast::Declaration::Function { ref ident, ref parameters, ref result, ref body } => {
                let tenv = tenv.clone();
                let mut venv = venv.clone();
                let args = parameters.iter().map(|&(_, ref t)| {
                    match tenv.get(t) {
                        Some(ty) => ty.clone(),
                        None => UnresolvedType::Name(t.clone(), None),
                    }
                }).collect();
                let ret = if let &Some(ref result) = result {
                    match tenv.get(result) {
                        Some(ty) => ty.clone(),
                        None => UnresolvedType::Name(result.clone(), None),
                    }
                } else {
                    Box::new(Type::Unit).into()
                };
                let fb = Value::Function { args: args, ret: ret };
                venv.insert(ident.clone(), fb);
                (tenv, venv)
            }

            ast::Declaration::Variable { ref ident, ref tdent, ref init } => {
                (tenv.clone(), venv.clone())
            }

            ast::Declaration::Type { ref tdent, ref ty } => {
                let mut tenv = tenv.clone();
                let venv = venv.clone();
                let tb = UnresolvedType::Name(tdent.clone(), None);
                tenv.insert(tdent.clone(), tb);
                (tenv, venv)
            }
        }
    }

    fn fill(&self,
                 tenv: &Env<UnresolvedType<'a>>,
                 venv: &Env<Value>) -> (Env<UnresolvedType<'a>>, Env<Value>)
    {
        match *self.0 {
            ast::Declaration::Function { ref ident, ref parameters, ref result, ref body } => {
                let tenv = tenv.clone();
                let mut venv = venv.clone();
                let args = parameters.iter().map(|&(_, ref t)| {
                    match tenv.get(t) {
                        Some(ty) => ty.clone(),
                        None => panic!("undefined type: `{}`.", t),
                    }
                }).collect();
                let ret = if let &Some(ref result) = result {
                    match tenv.get(result) {
                        Some(ty) => ty.clone(),
                        None => panic!("undefined type: `{}`.", result),
                    }
                } else {
                    Type::Unit
                };
                let fb = Value::Function { args: args, ret: ret };
                venv.insert(ident.clone(), fb);
                (tenv, venv)
            }

            ast::Declaration::Variable { ref ident, ref tdent, ref init } => {
                let tenv = tenv.clone();
                let mut venv = venv.clone();
                let init = init.translate(&tenv, &venv);
                if let &Some(ref tdent) = tdent {
                    match tenv.get(tdent) {
                        Some(ty) => {
                            ty.unify(&init.ty);
                        },
                        None => {
                            panic!("undefined type: `{}`.", tdent);
                        },
                    }
                }
                let vb = Value::Variable { ty: init.ty };
                venv.insert(ident.clone(), vb);
                (tenv, venv)
            }

            ast::Declaration::Type { ref tdent, ref ty } => {
                let mut tenv = tenv.clone();
                let venv = venv.clone();
                let ty = ty.translate(&tenv, &venv);
                println!("translating type dec {:?}", self.0);
                let tb = UnresolvedType::Name(tdent.clone(), Some(Box::new(ty)));
                tenv.insert(tdent.clone(), tb);
                println!("TENV: {:?}", tenv);
                (tenv, venv)
            }
        }
    }
}

pub struct MutualDeclarations<'a>(&'a [Box<ast::Declaration>]);

impl<'a> MutualDeclarations<'a> {
    pub fn new(declarations: &'a [Box<ast::Declaration>]) -> Self {
        MutualDeclarations(declarations)
    }
}

impl<'a> IntoIterator for MutualDeclarations<'a> {
    type Item = &'a [Box<ast::Declaration>];
    type IntoIter = MutualDeclarationsIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        MutualDeclarationsIter {
            current:  self.0
        }
    }
}

pub struct MutualDeclarationsIter<'a> {
    current: &'a [Box<ast::Declaration>],
}

impl<'a> Iterator for MutualDeclarationsIter<'a> {
    type Item = &'a [Box<ast::Declaration>];

    fn next(&mut self) -> Option<Self::Item> {
        if self.current.is_empty() {
            return None;
        }

        let mut index = 0;
        for declaration in &*self.current {
            match (&*self.current[0], &**declaration) {
                (&ast::Declaration::Function { .. },
                 &ast::Declaration::Function { .. })
              | (&ast::Declaration::Variable { .. },
                 &ast::Declaration::Variable { .. })
              | (&ast::Declaration::Type { .. },
                 &ast::Declaration::Type { .. }) =>
                {
                    index += 1;
                },
                _ => {}
            };
        }

        let ret = Some(&self.current[0..index]);
        self.current = &self.current[index..];
        ret
    }
}
