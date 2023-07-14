struct MinStack {
    s:Vec<i32>,
    min_stack: Vec<i32>
}



impl MinStack {

    fn new() -> Self {
        MinStack{
            s: vec![],
            min_stack:vec![]
        }
        
    }
    
    fn push(&mut self, val: i32) {
        self.s.push(val);
        let minval= self.min_stack.last().unwrap_or(&val);
        self.min_stack.push(val.min(*minval));
        
    }
    
    fn pop(&mut self) {
        self.min_stack.pop();
        self.s.pop();
        
    }
    
    fn top(&self) -> i32 {
        *self.s.last().unwrap()
    }
    
    fn get_min(&self) -> i32 {
        *self.min_stack.last().unwrap()
    }
}

/**
 * Your MinStack object will be instantiated and called as such:
 * let obj = MinStack::new();
 * obj.push(val);
 * obj.pop();
 * let ret_3: i32 = obj.top();
 * let ret_4: i32 = obj.get_min();
 */
fn main() {
    // Create a new MinStack
    let mut stack = MinStack::new();

    // Push values into the stack
    stack.push(5);
    stack.push(2);
    stack.push(8);

    // Get the top value from the stack
    let top_value = stack.top();
    println!("Top value: {}", top_value);  // Output: 8

    // Get the minimum value from the stack
    let min_value = stack.get_min();
    println!("Minimum value: {}", min_value);  // Output: 2

    // Pop a value from the stack
    stack.pop();

    // Get the updated top value
    let updated_top_value = stack.top();
    println!("Updated top value: {}", updated_top_value);  // Output: 2
}
