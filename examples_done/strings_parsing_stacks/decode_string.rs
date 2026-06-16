// Given an encoded string, return its decoded string.

// The encoding rule is: k[encoded_string], where the encoded_string inside the square brackets is being repeated exactly k times. Note that k is guaranteed to be a positive integer.

// You may assume that the input string is always valid; there are no extra white spaces, square brackets are well-formed, etc. Furthermore, you may assume that the original data does not contain any digits and that digits are only for those repeat numbers, k. For example, there will not be input like 3a or 2[4].

fn decode_string(s: String) -> String {
    let chars: Vec<char> = s.chars().collect();

    let mut count_stack: Vec<usize> = Vec::new();
    let mut string_stack: Vec<String> = Vec::new();

    let mut current = String::new();
    let mut k = 0;

    for ch in chars {
        match ch {
            // build number 
            '0'..='9' => {
                k = k*10 + (ch as usize - '0' as usize);
            }

            // push
            '[' => {
                count_stack.push(k);
                k = 0;
                string_stack.push(current.clone());
                current.clear();
            }

            // pop
            ']' => {
                let repeat = count_stack.pop().unwrap();
                let mut prev = string_stack.pop().unwrap();

                let expanded = current.repeat(repeat);
                prev.push_str(&expanded);

                current = prev;
            }

            // normal char
            _ => {
                current.push(ch);
            }
        }
    }

    current
}

fn main() {
    let input_1 = "3[a2[c]]".to_string();
    let input_2 = "2[abc]3[cd]ef".to_string();

    println!("decode 1: {}", decode_string(input_1));
    println!("decode 2: {}", decode_string(input_2));
}