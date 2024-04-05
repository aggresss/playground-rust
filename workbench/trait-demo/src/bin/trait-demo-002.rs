// Default Impls

trait Greet {
    fn greet(&self, name: &str) -> String;
    fn greet_loudly(&self, name: &str) -> String {
        self.greet(name) + "!"
    }
}

struct Hello;
struct Hola;

impl Greet for Hello {
    fn greet(&self, name: &str) -> String {
        format!("Hello {}", name)
    }
}

impl Greet for Hola {
    fn greet(&self, name: &str) -> String {
        format!("Hola {}", name)
    }

    fn greet_loudly(&self, name: &str) -> String {
        let mut greeting = self.greet(name);
        greeting.insert_str(0, "i");
        greeting + "!"
    }
}

fn main() {
    println!("{}", Hello.greet("John"));
    println!("{}", Hello.greet_loudly("John"));
    println!("{}", Hola.greet("John"));
    println!("{}", Hola.greet_loudly("John"));
}
