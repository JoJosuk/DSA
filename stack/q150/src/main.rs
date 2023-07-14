impl Solution {
    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
     let mut stack= Vec::new();
     for s in tokens.iter(){
         if let Ok(x) =s.parse::<i32>(){
             stack.push(x);
         }
         else{
             let b = stack.pop().unwrap();
             let a = stack.pop().unwrap();
             stack.push(match s.as_str(){
                 "+"=>a+b,
                 "-"=>a-b,
                 "/"=>a/b,
                 "*"=>a*b,
                 _=>panic!("no leetcode pani taralle")
             });
         }

     } 
     stack.pop().unwrap()  
    }
}
struct Solution;
fn main() {
    let tokens = vec![
        String::from("2"),
        String::from("1"),
        String::from("+"),
        String::from("3"),
        String::from("*"),
    ];
    
    let result = Solution::eval_rpn(tokens);
    println!("Result: {}", result);
}
