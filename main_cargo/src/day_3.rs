pub mod day_3_module
{
    use std::env;
    use std::fs;

    pub fn convert_bin_to_int(data: String) -> i32
    {
        let mut dec: u32 = 0;
        for (i, bit) in data.chars().rev().enumerate()
        {
            let bit_num = bit.to_digit(10)
            .expect("Error while parsing bit");
            dec += bit_num * u32::pow(2, i as u32);
        }
        return dec as i32;
    }

    pub fn run()
    {
        println!("day 3 exercise-----------");
        let content = fs::read_to_string("input3.txt").expect("Something went wrong");

        //println!("text /n{}", content);

        let split = content.split("\n");
        let items: Vec<&str> = split.collect();

        let size = items[0].len();
        let mut result = vec![0;size];
        let mut count = 0;
        for item in items
        {
            count += 1;
            for (index, c) in item.char_indices()
            {
                if c == '0'
                {
                }
                else if c == '1'
                {
                    result[index] += 1;
                }
            } 
        }


        // let result2 : f32 = result.iter().map(|x| x / count).collect();
        let gamma : Vec<char> = result.iter()
                                    .map(|&x| x as f32 / count as f32)
                                    .map(|x| 
                                        {
                                            if x > 0.5
                                            {
                                                '1'
                                            }
                                            else
                                            {
                                                '0'
                                            }
                                        })
                                    .collect();
        
        let epsilon : Vec<char> = gamma.iter().map(|&x| if x == '1' {'0'} else {'1'}).collect();

        println!("gamma {:?}" ,gamma);
        println!("epsilon {:?}" ,epsilon);
        let gamma_string = gamma.iter().fold(String::new(), |acc, x| format!("{}{}", acc, x));
        let epsilon_string = epsilon.iter().fold(String::new(), |acc, x| format!("{}{}", acc, x));
        println!("gamma_string {}", gamma_string);
        println!("epsilon_string {}", epsilon_string);

        let parsed_gamma = convert_bin_to_int(gamma_string);
        let parsed_epsilon = convert_bin_to_int(epsilon_string);
        println!("gamma number {}", parsed_gamma);

        println!("result {}", parsed_gamma * parsed_epsilon );

    }
}