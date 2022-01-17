pub mod day_10_2_module
{
    use std::env;
    use std::fs;
    use std::collections::HashMap;
    use array2d::Array2D;

    
    pub fn run()
    {
        println!("day 10 exercise 2 ------------");
        let content = fs::read_to_string("input10.txt").expect("Something went wrong");
        let lines : Vec<&str> = content.split("\n").collect();

        let mut points = 0;
        let mut scores: Vec<i64> = Vec :: new();
        for line in lines
        {
            let chars = line.chars();
            let mut stack: Vec<char> = Vec :: new();
            let mut traversed = 0;
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
                traversed += 1;
            }

            if traversed == line.chars().count()
            {
                // this line has not error but might be incomplete
                let missing_part : String = stack.into_iter().collect();
                // println!(" this line is fine {} missing {}", line, missing_part);
                let missing_part_chars = missing_part.chars().rev();
                let mut score:i64 = 0;
                for c in missing_part_chars
                {

                    let score_by_5 = score * 5;
                    score = 
                        if c == ')' 
                        {
                            score_by_5 + 1
                        }
                        else if c == ']'
                        {
                            score_by_5 + 2
                        }
                        else if c == '}'
                        {
                            score_by_5 + 3
                        }
                        else if c == '>'
                        {
                            score_by_5 + 4
                        }
                        else 
                        {
                            0
                        };
                }
                // println!("score {}", score);
                scores.push(score);
            }
        }
        scores.sort();
        // println!("total points {:?}", scores);
        let mid_index = scores.len() / 2;
        let final_score = scores.get(mid_index).unwrap();
        println!("mid score {}", final_score);
    }
}