// fn main() {
//     let width1 = 30;
//     let height1 = 50;

//     println!(
//         "The area of the rectangle is {} square pixels.",
//         area(width1, height1)
//     );
// }

// fn area(width: u32, height: u32) -> u32 {
//     width * height
// }
// fn main() {
//     let rect1 = (30, 50);

//     println!(
//         "The area of the rectangle is {} square pixels.",
//         area(rect1)
//     );
// }
// fn area(dimensions: (u32, u32)) -> u32 {
//     dimensions.0 * dimensions.1
// }

// Refactoring with Structs: Adding More Meaning
// struct Rectangle {
//     width: u32,
//     height: u32,
// }

// fn main() {
//     let rect1 = Rectangle {
//         width: 30,
//         height: 50,
//     };

//     println!(
//         "The area of the rectangle is {} square pixels.",
//         area(&rect1)
//     );
// }

// fn area(rectangle: &Rectangle) -> u32 {
//     rectangle.width * rectangle.height
// }


// Displaying a struct with println!
// Its possible by doing this:
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    // To print rect1, we need to use a formatting specifier
    // which tells println! we want to use an output format called Debug.
    // The Debug trait enables us to print our struct in a way that is useful
    // for developers so we can see its value while we’re debugging our code.
    // To print rect1 using {:?}, which is similar to {}, we have to
    // annotate Rectangle with the annotation #[derive(Debug)].
    println!("rect1 is {:#?}", rect1);
}


// Here’s an example where we’re interested in the value that gets assigned to
// the width field, as well as the value of the whole struct in rect1:
// #[derive(Debug)]
// struct Rectangle {
//     width: u32,
//     height: u32,
// }

// fn main() {
//     let scale = 2;
//     let rect1 = Rectangle {
//         width: dbg!(30 * scale),
//         height: 50,
//     };

//     dbg!(&rect1);
// }
// We can put dbg! around the expression 30 * scale and, because dbg! returns 
// ownership of the expression’s value, the width field will get the same 
// value as if we didn’t have the dbg! call there. We don’t want dbg! to take 
// ownership of rect1, so we use a reference to rect1 in the next call. Here’s 
// what the output of this example looks like:

// $ cargo run
//    Compiling rectangles v0.1.0 (file:///projects/rectangles)
//     Finished dev [unoptimized + debuginfo] target(s) in 0.61s
//      Running `target/debug/rectangles`
// [src/main.rs:10] 30 * scale = 60
// [src/main.rs:14] &rect1 = Rectangle {
//     width: 60,
//     height: 50,
// }


// METHOD SYNTAX
// Reference: https://doc.rust-lang.org/book/ch05-03-method-syntax.html
// Methods are similar to functions: they’re declared with the fn keyword and
// their name, they can have parameters and a return value, and they contain
// some code that is run when they’re called from somewhere else. However,
// methods are different from functions in that they’re defined within the
// context of a struct (or an enum or a trait object, which we cover in
// Chapters 6 and 17, respectively), and their first parameter is always self,
// which represents the instance of the struct the method is being called on.
// #[derive(Debug)]
// struct Rectangle {
//     width: u32,
//     height: u32,
// }

// impl Rectangle {
//     fn area(&self) -> u32 {
//         self.width * self.height
//     }
// }

// fn main() {
//     let rect1 = Rectangle {
//         width: 30,
//         height: 50,
//     };

//     println!(
//         "The area of the rectangle is {} square pixels.",
//         rect1.area()
//     );
// }
//To define the function within the context of Rectangle, we start an impl (implementation) 
// block for Rectangle. Everything within this impl block will be associated with the 
// Rectangle type. Then we move the area function within the impl curly brackets and change 
// the first (and in this case, only) parameter to be self in the signature and everywhere 
// within the body. In main, where we called the area function and passed rect1 as an argument, 
// we can instead use method syntax to call the area method on our Rectangle instance. The 
// method syntax goes after an instance: we add a dot followed by the method name, parentheses, 
// and any arguments.