pub mod day_3_2_module
{
    use std::env;
    use std::fs;

    pub fn convert_bin_to_int(data: &str) -> i32
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

    fn get_most_common(items : &Vec<&str>) -> Vec<char>
    {
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

        let gamma : Vec<char> = result.iter()
                                    .map(|&x| x as f32 / count as f32)
                                    .map(|x| if x >= 0.5 {'1'} else {'0'})
                                    .collect();
        return gamma;
    }

    fn get_least_common(items : &Vec<&str>) -> Vec<char>
    {
        let most_common = get_most_common(items);
        let least_common : Vec<char> = most_common.iter().map(|&x| if x == '1' {'0'} else {'1'}).collect();
        least_common
    }

    pub fn run()
    {
        println!("day 3 exercise 2=---------------");
        let content = fs::read_to_string("input3.txt").expect("Something went wrong");

        //println!("text /n{}", content);

        let split = content.split("\n");
        let items: Vec<&str> = split.collect();
        
        // let items = vec![
        //     "00100",
        //     "11110",
        //     "10110",
        //     "10111",
        //     "10101",
        //     "01111",
        //     "00111",
        //     "11100",
        //     "10000",
        //     "11001",
        //     "00010",
        //     "01010"
        // ];
        

        let mut current_index = 0;
        let mut current_list = items.clone();
        loop
        {
            let list = current_list.clone();

            let gamma = get_most_common(&list);
            let most_common = gamma.get(current_index).unwrap();

            let filtered_list : Vec<&str> = list
                .into_iter()
                .filter(|item| 
                {
                    let a = item.chars().nth(current_index).unwrap();
                    a == *most_common
                }).collect();

            current_index += 1;
            current_list = filtered_list;
            if current_list.len() == 1
            {
                break;
            }
        }

        let last_item = current_list.get(0).unwrap();
        let oxigen_rating = convert_bin_to_int(last_item);
        println!("result A {:?}", oxigen_rating);
        current_index = 0;
        current_list = items;

        loop
        {
            let list = current_list.clone();

            let epsilon = get_least_common(&list);
            let least_common = epsilon.get(current_index).unwrap();

            let filtered_list : Vec<&str> = list
                .into_iter()
                .filter(|item| 
                {
                    let a = item.chars().nth(current_index).unwrap();
                    a == *least_common
                }).collect();

            current_index += 1;
            current_list = filtered_list;
            if current_list.len() == 1
            {
                break;
            }
        }

        let last_item = current_list.get(0).unwrap();
        let co2_rating = convert_bin_to_int(last_item);
        println!("result B {:?}", co2_rating);

        let result = oxigen_rating * co2_rating;

        println!("final result {}", result);














        // println!("gamma {:?}" ,gamma);
        // println!("epsilon {:?}" ,epsilon);
        // let gamma_string = gamma.iter().fold(String::new(), |acc, x| format!("{}{}", acc, x));
        // let epsilon_string = epsilon.iter().fold(String::new(), |acc, x| format!("{}{}", acc, x));
        // println!("gamma_string {}", gamma_string);
        // println!("epsilon_string {}", epsilon_string);

        // let parsed_gamma = convert_bin_to_int(gamma_string);
        // let parsed_epsilon = convert_bin_to_int(epsilon_string);
        // println!("gamma number {}", parsed_gamma);

        // println!("result {}", parsed_gamma * parsed_epsilon );

    }
}