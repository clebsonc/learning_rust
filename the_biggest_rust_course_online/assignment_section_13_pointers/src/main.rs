mod smart_pointers {
    use std::rc::Rc;

    pub fn stack_and_heap_allocation() -> u32 {
        // this is allocated on the stack
        let val1: u32 = 5;
        
        // this is allocated on the heap
        let val2 = Box::<u32>::new(9);

        val1 * *val2
    }

    pub fn stack_reference_couter() {
        let var1 = String::from("Hello World");
        let var2 = Rc::new(var1);

        println!("Pointers of RC var2: {}", Rc::strong_count(&var2));

        let var3 = Rc::new(var2.clone());
        println!("Pointers of RC var2: {}", Rc::strong_count(&var2));
        println!("Pointers of RC var3: {}", Rc::strong_count(&var3));
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
    use crate::smart_pointers::stack_reference_couter;

    stack_reference_couter();
}
