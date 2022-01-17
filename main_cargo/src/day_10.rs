pub mod day_10_module
{
    use std::env;
    use std::fs;
    use std::collections::HashMap;
    use array2d::Array2D;

    
    pub fn run()
    {
        println!("day 10 exercise ------------");
        let content = fs::read_to_string("input10.txt").expect("Something went wrong");
        let lines : Vec<&str> = content.split("\n").collect();

        let mut points = 0;
        for line in lines
        {
            let chars = line.chars();
            let mut stack: Vec<char> = Vec :: new();
            'line : for current_char in chars
            {
                if current_char == '('
                {
                    stack.push(')');
                }
                else if current_char == '['
                {
                    stack.push(']');
                }
                else if current_char == '{'
                {
                    stack.push('}');
                }
                else if current_char == '<'
                {
                    stack.push('>');
                }
                else if current_char == ')'
                {
                    match stack.pop()
                    {
                        Some(c) => if c != ')' { points += 3; break 'line;},
                        _ => {}
                    }
                }
                else if current_char == ']'
                {
                    match stack.pop()
                    {
                        Some(c) => if c != ']' { points += 57; break 'line;},
                        _ => {}
                    }
                }
                else if current_char == '}'
                {
                    match stack.pop()
                    {
                        Some(c) => if c != '}' { points += 1197; break 'line;},
                        _ => {}
                    }
                }
                else if current_char == '>'
                {
                    match stack.pop()
                    {
                        Some(c) => if c != '>' { points += 25137; break 'line;},
                        _ => {}
                    }
                }
            }
        }
        println!("totaal points {}", points);
    }
}