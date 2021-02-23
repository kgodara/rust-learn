// `super` and `self` keywords can be used in paths to remove ambiguity
// and prevent unnecessary path hardcoding.

fn function() {
    println!("called `function()`");
}

mod cool {
    pub fn function() {
        println!("called `cool::function()`");
    }
}

mod my {
    fn function() {
        println!("called `my::function()`");
    }

    mod cool {
        pub fn function() {
            println!("called `my::cool::function()`");
        }
    }

    pub fn indirect_call() {
        // Access all functions names `function` from this scope
        print!("called `my::indirect_call()`, that\n>");

        // The `self` keyword referes to the current module scope - `my`
        // Calling `self::function()` and `function()` would give same result
        self::function();
        function();

        // Can use `self` to access other modules in `my`:
        self::cool::function();

        // The `super` keyword refers to the parent scope outside the `my` module
        super::function();

        // This will bind to the `cool::function` in the *crate* scope.
        // In this case the crate scope is the outermost scope.
        {
            use crate::cool::function as root_function;
            root_function();
        }
    }
}

fn main() {
    my::indirect_call();
}