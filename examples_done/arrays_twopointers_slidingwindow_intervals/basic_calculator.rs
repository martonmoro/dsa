fn calculate(s: &str) -> i32 {
    let mut stack: Vec<i32> = Vec::new();
    let mut num = 0;
    let mut op = '+';
    let chars: Vec<char> = s.chars().collect();

    for i in 0..chars.len() {
        let c = chars[i];

        if c.is_ascii_digit() {
            num = num * 10 + (c as i32 - '0' as i32);
        }

        if (!c.is_ascii_digit() && c != ' ') || i == chars.len() - 1 {
            match op {
                '+' => stack.push(num),
                '-' => stack.push(-num),
                '*' => {
                    let last = stack.pop().unwrap();
                    stack.push(last * num);
                }
                '/' => {
                    let last = stack.pop().unwrap();
                    stack.push(last / num);
                }
                _ => {}
            }

            op = c;
            num = 0;
        }
    }

    stack.iter().sum()
}

fn main() {
    let s1 = "3+2*2".to_string();
    let s2 = " 3/2 ".to_string();
    let s3 = " 3+5 / 2 ".to_string();

    println!("{} = {}", s1, calculate(&s1));
    println!("{} = {}", s2, calculate(&s2));
    println!("{} = {}", s3, calculate(&s3));
}