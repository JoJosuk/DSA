impl Solution {
    pub fn calculate(s: String) -> i32 {
        let mut stack: Vec<i32> = Vec::new();
        let mut number = 0;
        let mut sign = 1;
        let mut result = 0;

        for i in s.chars() {
            if i == '+' {
                result += sign * number;
                sign = 1;
                number = 0;
            } else if i == '-' {
                result += sign * number;
                sign = -1;
                number = 0;
            } else if i == '(' {
                stack.push(result);
                stack.push(sign);
                sign = 1;
                result = 0; // Reset result for the nested calculation
            } else if i == ')' {
                result += sign * number;
                result *= stack.pop().unwrap();
                result += stack.pop().unwrap();
                number = 0;
            } else if i.is_digit(10) {
                number = number * 10 + i.to_digit(10).unwrap() as i32;
            }
        }

        result += sign * number;
        result
    }
}
