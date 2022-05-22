#[derive(Debug)] // This enables using the debugging format string "{:?}"
struct Carrot {
    percent_left: f32,
}

// 1. Define a trait named `Bite`
//
// Define a single required method, `fn bite(self: &mut Self)`.  We will call this method when we
// want to bite something.  Once this trait is defined, you should be able to run the program with
// `cargo run` without any errors.
//
trait Bite {
    // interface for the bite function
    fn bite(self: &mut Self);
}

impl Bite for Carrot {
    // concrete implementation of the bite function
    fn bite(self: &mut Self) {
        // Eat 20% of the remaining carrot. It may take awhile to eat it all...
        self.percent_left *= 0.8;
    }
}


// 2. Now create a struct named Grapes with a field that tracks how many grapes are left.  If you
// need a hint, look at how it was done for Carrot at the bottom of this file (you should probably
// use a different field, though).
//
#[derive(Debug)] // include this line right before your struct definition
struct Grapes {
    remaining: i32
}

// 3. Implement Bite for Grapes.  When you bite a Grapes, subtract 1 from how many grapes are left.
// If you need a hint, look at how it was done for Carrot at the bottom of this file.
//
impl Bite for Grapes {
    // this concrete implentation of bite MUST adhere (signature/parameters) to the interface defined above; I cannot do what I want here. 
    fn bite(self: &mut Self) {
        self.remaining -= 1;
    }
}

// This is what is known as a "Generic" Function
// I am accepting, as an argument, any object that IMPLEMENTS the Bite trait
// If the object does not implement Bite it will not have access to this funciton
fn bunny_nibbles<T: Bite>(item: &mut T) {
    item.bite();
    item.bite();
}


fn main() {
    // Once you finish #1 above, this part should work.
    let mut carrot = Carrot { percent_left: 100.0 };
    carrot.bite();
    println!("I take a bite: {:?}", carrot);

    // let mut grapes = Grapes { remaining: 101 };
    // grapes.bite();
    // println!("I now have {} grapes remaining!", grapes.remaining);
    // grapes.bite();
    // println!("I now have {} grapes remaining!", grapes.remaining);

    // 4. Uncomment and adjust the code below to match how you defined your
    // Grapes struct.
    
    let mut grapes = Grapes { remaining: 100 };
    grapes.bite();
    println!("Eat a grape: {:?}", grapes);

    // Challenge: Uncomment the code below. Create a generic `bunny_nibbles`
    // function that:
    // - takes a mutable reference to any type that implements Bite
    // - calls `.bite()` several times
    // Hint: Define the generic type between the function name and open paren:
    //       fn function_name<T: Bite>(...)
    //
    bunny_nibbles(&mut grapes);
    println!("Bunny nibbles for awhile: {:?}", grapes);
}

