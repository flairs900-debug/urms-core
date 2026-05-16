pub struct RuntimeLoop;

impl RuntimeLoop {
    pub fn run<T>(_graph: &mut T) {
        println!("Runtime loop started");

        println!("Interpreted -> system overload");

        println!("rewrite engine started");

        println!(
            "rewrite -> SemanticEngine -> ReflectiveSemanticEngine"
        );

        println!("rewrite engine finished");

        println!("Runtime loop finished");
    }
}