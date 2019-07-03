/*

Example of dynamically nested objects in Rust using LinkedLists

*/

struct LinkedList {
    val: i32,
    // Nullable<Pointer<Object>>
    next: Option<Box<LinkedList>>,
}

impl LinkedList {
    fn new(v: i32, n: Option<Box<LinkedList>>) -> Self {
        return Self{val: v, next: n};
    }
}

fn print_list(mut list: &LinkedList) {  
    print!("{}", list.val);
    loop {
        match &list.next {
            Some(x) => {
                print!(", {}", x.val);
                list = x;
            },
            _ => {
                print!("\nEnd of list.\n");
                break;
            },
        }
    }
}

fn main() {
    let mut head = LinkedList::new(16, None);
    head.next = Some(Box::new(LinkedList::new(8, None)));

    // Should print: 16, 8
    print_list(&head);

    let some_bool = false;
    let some_other_bool = true;

    // Variables can be assigned using expressions
    let x: i32 = 
        if some_bool {
            12
        } else if some_other_bool {
            5
        }
        else {
            -1
        }
    ; // x = 5

    // or using {} wraps statements into an expression. Note that no value can be returned from sub-expressions.
    let y: i32 = {
        let mut return_value: i32 = 12;
        if some_bool {
            return_value = return_value - 12;
            // return 92; <-- Throws an error because the if statement is wrapped in another expression (the {} brackets)
        } else if some_other_bool {
            return_value = return_value + 5;
        }
        return_value
    }; // y = 17

    println!("\nx is: {}\ny is: {}", x, y);

    // Example of extreme expression nesting
    let really_confusing_list = generate_nested_object(true, true, {
        println!("Printing from a function parameter");
        let _random_statement_here: Option<i32> = None;

        if y == 100 {
            102
        } else {
            20
        }
    });


    println!("\nPrinting really confusing list:");

    print_list(&really_confusing_list);
}

// Demonstrates the ability to mix conditional expressions and statements in declarations
// Also demonstrates the power to nest items in Rust
fn generate_nested_object(condition1: bool, condition2: bool, some_value: i32) -> LinkedList {
    // Mutable objects can be mutated within conditional declarations like below
    let mut random_int: i32 = some_value - 2;
    println!("random_int is {}", random_int);

    // Declaring a linked list to be returned. I could also use `let` here, then return later.
    LinkedList { val: 1, next: Some(Box::new(LinkedList {
        val: 2, next: // List 2
            if condition1 { // Expression
                random_int = random_int + 5; // Can do logic within an expression
                Some(Box::new(LinkedList { // Nested expression containing statements
                    val: 3, next: { // List 3
                        let mut another_one: Option<Box<LinkedList>> = None;
                        if condition2 {
                            random_int = random_int - 100;
                            another_one = Some(Box::new(LinkedList {val: 4, next: None})) // List 4
                        }
                        println!("\nPrinting random_int from a nested object declaration: {}", random_int);
                        
                        another_one // List 4 is returned from the expression here (`next` of List 3)
                    }
                }))
            } else {
                println!("This statment runs within an expression within an expression");
                None
            }
            // println!("This statement must be wrapped in an expression to compile");
        })) // End long LinkedList
    } // End returned LinkedList
}
