
impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack = Vec::with_capacity(s.len());
        for i  in s.chars(){
            match i {
                '(' | '{' | '[' =>stack.push(i),
                _ =>{
                    match stack.pop(){
                        Some('(') if i==')'=>(),
                        Some('[') if i==']'=>(),
                        Some('{') if i == '}'=>(),
                        _ => return false
                    }
                }
            }
        }
        return stack.is_empty()
        
    }
}
struct Solution;

fn main() {

    let s = "()".to_string();
    let result = Solution::is_valid(s);
    println!("{}", result);
  
    let s = "()[]{}()".to_string();
    let result = Solution::is_valid(s);
    println!("{}", result);
  
    let s = "([)]".to_string();
    let result = Solution::is_valid(s);
    println!("{}", result);
  }
  