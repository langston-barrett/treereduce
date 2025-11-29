#![feature(test)]

extern crate test;

#[cfg(test)]
mod tests {
    use std::fmt::Debug;
    use std::fs::read_to_string;
    use std::io;
    use test::Bencher;
    use treereduce::{treereduce, Check, Edits, NodeTypes, Original};

    #[derive(Clone, Debug)]
    pub struct TrueCheck {}

    impl Check for TrueCheck {
        type State = ();

        fn start(&self, _stdin: &[u8]) -> io::Result<Self::State> {
            Ok(())
        }

        fn cancel(&self, _state: Self::State) -> io::Result<()> {
            Ok(())
        }

        fn try_wait(&self, _state: &mut Self::State) -> io::Result<Option<bool>> {
            Ok(Some(true))
        }

        fn wait(&self, _state: Self::State) -> io::Result<bool> {
            Ok(true)
        }
    }

    #[derive(Clone, Debug)]
    pub struct FalseCheck {}

    impl Check for FalseCheck {
        type State = ();

        fn start(&self, _stdin: &[u8]) -> io::Result<Self::State> {
            Ok(())
        }

        fn cancel(&self, _state: Self::State) -> io::Result<()> {
            Ok(())
        }

        fn try_wait(&self, _state: &mut Self::State) -> io::Result<Option<bool>> {
            Ok(Some(false))
        }

        fn wait(&self, _state: Self::State) -> io::Result<bool> {
            Ok(false)
        }
    }

    fn parse(language: tree_sitter::Language, code: &str) -> tree_sitter::Tree {
        let mut parser = tree_sitter::Parser::new();
        parser
            .set_language(language)
            .expect("Failed to set tree-sitter parser language");
        parser.parse(code, None).expect("Failed to parse code")
    }

    fn test_c_file(path: &str) -> String {
        read_to_string(String::from("benches/c/") + path).unwrap()
    }

    fn test_java_file(path: &str) -> String {
        read_to_string(String::from("benches/java/") + path).unwrap()
    }

    fn reduce<T: Check + Debug + Send + Sync + 'static>(
        node_types: NodeTypes,
        orig: Original,
        chk: T,
    ) -> Edits {
        treereduce(1, node_types, orig, chk, 1).unwrap()
    }

    fn bench_reduce_c<T: Clone + Check + Debug + Send + Sync + 'static>(
        b: &mut Bencher,
        src: &str,
        chk: T,
    ) {
        let node_types = NodeTypes::new(tree_sitter_c::NODE_TYPES).unwrap();
        let orig = Original {
            tree: parse(tree_sitter_c::language(), &src),
            text: src.as_bytes().to_vec(),
        };
        b.iter(|| reduce(node_types.clone(), orig.clone(), chk.clone()));
    }

    fn bench_reduce_java<T: Clone + Check + Debug + Send + Sync + 'static>(
        b: &mut Bencher,
        src: &str,
        chk: T,
    ) {
        let node_types = NodeTypes::new(tree_sitter_java::NODE_TYPES).unwrap();
        let orig = Original {
            tree: parse(tree_sitter_java::language(), &src),
            text: src.as_bytes().to_vec(),
        };
        b.iter(|| reduce(node_types.clone(), orig.clone(), chk.clone()));
    }

    #[bench]
    fn bench_node_types(b: &mut Bencher) {
        b.iter(|| NodeTypes::new(tree_sitter_c::NODE_TYPES).unwrap());
    }

    #[bench]
    fn bench_reduce_c_true_empty(b: &mut Bencher) {
        bench_reduce_c(b, "", TrueCheck {});
    }

    #[bench]
    fn bench_reduce_c_false_empty(b: &mut Bencher) {
        bench_reduce_c(b, "", FalseCheck {});
    }

    #[bench]
    fn bench_reduce_c_true_hello(b: &mut Bencher) {
        bench_reduce_c(b, &test_c_file("hello-world.c"), TrueCheck {});
    }

    #[bench]
    fn bench_reduce_c_false_hello(b: &mut Bencher) {
        bench_reduce_c(b, &test_c_file("hello-world.c"), FalseCheck {});
    }

    #[bench]
    fn bench_reduce_java_true_hello(b: &mut Bencher) {
        bench_reduce_java(b, &test_java_file("HelloWorld.java"), TrueCheck {});
    }

    #[bench]
    fn bench_reduce_java_false_hello(b: &mut Bencher) {
        bench_reduce_java(b, &test_java_file("HelloWorld.java"), FalseCheck {});
    }

    // TODO(lb):
    // #[bench]
    // fn bench_reduce_c_clang_hello(b: &mut Bencher) {
    //     bench_reduce_c(b, &test_file("hello-world.c"), &CmdCheck{});
    // }
}
