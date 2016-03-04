extern crate tiger;
extern crate tiger_syntax as syntax;

use std::io::prelude::*;
use std::fs::File;

macro_rules! test_file {
    ($name:ident, $path:expr $(,$meta:meta)*) => {
        #[test]
        $(#[$meta])*
        fn $name() {
            let mut file = File::open($path).unwrap();
            let mut source = String::new();
            file.read_to_string(&mut source).expect("error reading fixture");
            let ast = syntax::parse(&source).unwrap();
            let translation = tiger::translate(&ast).unwrap();
        }
    };
}

// TODO: More granular checks than panic/no-panic.
test_file!(test_smoke_merge, "fixtures/merge.tig");
test_file!(test_smoke_queens, "fixtures/queens.tig");
test_file!(test_smoke_test1, "fixtures/test1.tig");
test_file!(test_smoke_test2, "fixtures/test2.tig");
test_file!(test_smoke_test3, "fixtures/test3.tig");
test_file!(test_smoke_test4, "fixtures/test4.tig");
test_file!(test_smoke_test5, "fixtures/test5.tig");
test_file!(test_smoke_test6, "fixtures/test6.tig");
test_file!(test_smoke_test7, "fixtures/test7.tig");
test_file!(test_smoke_test8, "fixtures/test8.tig");
test_file!(test_smoke_test9, "fixtures/test9.tig");
test_file!(test_smoke_test10, "fixtures/test10.tig", should_panic);
test_file!(test_smoke_test11, "fixtures/test11.tig", should_panic);
test_file!(test_smoke_test12, "fixtures/test12.tig");
test_file!(test_smoke_test13, "fixtures/test13.tig", should_panic);
test_file!(test_smoke_test14, "fixtures/test14.tig", should_panic);
test_file!(test_smoke_test15, "fixtures/test15.tig", should_panic);
test_file!(test_smoke_test16, "fixtures/test16.tig", should_panic);
test_file!(test_smoke_test17, "fixtures/test17.tig", should_panic);
test_file!(test_smoke_test18, "fixtures/test18.tig", should_panic);
test_file!(test_smoke_test19, "fixtures/test19.tig", should_panic);
test_file!(test_smoke_test20, "fixtures/test20.tig", should_panic);
test_file!(test_smoke_test21, "fixtures/test21.tig", should_panic);
test_file!(test_smoke_test22, "fixtures/test22.tig", should_panic);
test_file!(test_smoke_test23, "fixtures/test23.tig", should_panic);
test_file!(test_smoke_test24, "fixtures/test24.tig", should_panic);
test_file!(test_smoke_test25, "fixtures/test25.tig", should_panic);
test_file!(test_smoke_test26, "fixtures/test26.tig", should_panic);
test_file!(test_smoke_test27, "fixtures/test27.tig");
test_file!(test_smoke_test28, "fixtures/test28.tig", should_panic);
test_file!(test_smoke_test29, "fixtures/test29.tig", should_panic);
test_file!(test_smoke_test30, "fixtures/test30.tig");
test_file!(test_smoke_test31, "fixtures/test31.tig", should_panic);
test_file!(test_smoke_test32, "fixtures/test32.tig", should_panic);
test_file!(test_smoke_test33, "fixtures/test33.tig", should_panic);
test_file!(test_smoke_test34, "fixtures/test34.tig", should_panic);
test_file!(test_smoke_test35, "fixtures/test35.tig", should_panic);
test_file!(test_smoke_test36, "fixtures/test36.tig", should_panic);
test_file!(test_smoke_test37, "fixtures/test37.tig");
test_file!(test_smoke_test38, "fixtures/test38.tig", should_panic);
test_file!(test_smoke_test39, "fixtures/test39.tig", should_panic);
test_file!(test_smoke_test40, "fixtures/test40.tig", should_panic);
test_file!(test_smoke_test41, "fixtures/test41.tig");
test_file!(test_smoke_test42, "fixtures/test42.tig");
test_file!(test_smoke_test43, "fixtures/test43.tig", should_panic);
test_file!(test_smoke_test44, "fixtures/test44.tig");
test_file!(test_smoke_test45, "fixtures/test45.tig", should_panic);
test_file!(test_smoke_test46, "fixtures/test46.tig");
test_file!(test_smoke_test47, "fixtures/test47.tig");
test_file!(test_smoke_test48, "fixtures/test48.tig");
test_file!(test_smoke_test49, "fixtures/test49.tig", should_panic);
