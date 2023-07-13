use std::collections::VecDeque;

struct Solution {
}

impl Solution {
    pub fn simplify_path(path: String) -> String {
        let mut stack = VecDeque::new();
        for dir in path.split('/') {
            match dir {
                "."|"" => {},
                ".." => {
                    if !stack.is_empty() {
                        stack.pop_back();
                    }
                },
                _ => {
                    stack.push_back(dir);
                }
            }
        }

        let mut result = "/".to_owned();
        for dir in stack {
            result = result + &dir + "/";
        }

        if result.len() > 1 {
            result.pop();
        }

        return result;
    }
}

fn main() {
    let path = "/home/user/./subdir/".to_string();
    let result = Solution::simplify_path(path);
    println!("{}", result);
}
