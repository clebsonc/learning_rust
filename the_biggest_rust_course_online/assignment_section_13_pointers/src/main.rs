// allowing dead code since this is only for learning purposes. This disable the warning for dead
// code for the entire crate.
#![allow(dead_code)]

mod smart_pointers {
    use std::rc::Rc;
    use std::cell::RefCell;


    #[derive(Debug)]
    pub struct Node {
        pub val: RefCell<i32>,
    }

    pub fn stack_and_heap_allocation() -> u32 {
        // this is allocated on the stack
        let val1: u32 = 5;
        
        // this is allocated on the heap
        let val2 = Box::<u32>::new(9);

        val1 * *val2
    }

    pub fn stack_reference_couter() {
        let var1 = String::from("Hello World");
        let var2 = Rc::new(&var1);

        println!("Pointers of RC var2: {}", Rc::strong_count(&var2));

        //let var3 = Rc::new(var2.clone());
        let var3 = Rc::new(&var2);
        let var4 = Rc::new(&var3);
        println!("Pointers of RC var2: {}", Rc::strong_count(&var2));
        println!("Pointers of RC var3: {}", Rc::strong_count(&var3));
        println!("Values of RC variable: var1 = {}", var1);
        println!("Values of RC variable: var2 = {}", var2);
        println!("Values of RC variable: var3 = {}", var3);
        println!("Values of RC variable: var4 = {}", var4);
    }

    pub fn reference_counter_multiple_values() {
        let val = 10;
        let mut rc1 = Rc::new(Box::new(val));
        let rc2 = rc1.clone();
        let rc3 = rc1.clone();
        println!("Counters: {}", Rc::strong_count(&rc2));
        println!("{} {} {} {}", val, rc1, rc2, rc3);
        rc1 = Box::new(20).into();
        println!("{} {} {} {}", val, rc1, rc2, rc3);
    }

    #[cfg(test)]
    mod test{
        use super::*;

        #[test]
        fn multiply_two_numbers() {
            assert_eq!(stack_and_heap_allocation(), 45);
        }
    }
}


fn main() {
    use std::rc::Rc;
    use std::cell::RefCell;
    use crate::smart_pointers::*;

    // println!("Multiply result: {}", stack_and_heap_allocation());
    // stack_reference_couter();
    // reference_counter_multiple_values();
    let n1 = Rc::new(Node {val: RefCell::new(20)});
    {
        *n1.val.borrow_mut() = 30;
        let n2 = n1.clone();
        println!("Counters: {:#?}", Rc::strong_count(&n1));
        println!("{:#?} \n{:#?}", n1, n2);
    }
    // let n3 = n1.clone();
    println!("{:#?}", n1);
    println!("Counters: {:#?}", Rc::strong_count(&n1));

    // println!("Counters: {:#?}", Rc::strong_count(&n2));
    // println!("Counters: {:#?}", Rc::strong_count(&n1));
}
