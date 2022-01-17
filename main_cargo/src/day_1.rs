pub mod day_1_module
{
    use std::env;
    use std::fs;

    pub fn run()
    {
        println!("day 1 exercise");
        let content = fs::read_to_string("input.txt").expect("Something went wrong");

        //println!("text /n{}", content);

        let split = content.split("\n");
        let items: Vec<&str> = split.collect();

        let mut numbers: Vec<i32> = Vec::new();

        for item in items
        {
            let number = item.parse::<i32>();
            match number
            {
                Ok(parsed_number) => numbers.push(parsed_number),
                _ => println!("Something wrong converting to a number {}", item)
            }
        }

        let mut previous : Option<i32> = None;
        let mut count : i32 = 0;

        for number in numbers
        {
            match previous
            {
                Some(previous_number) => 
                    {
                        let difference = number - previous_number;
                        if difference > 0
                        {
                            count = count + 1;
                        }
                        previous = Some(number);
                    }
                None => previous = Some(number)
            }
        }

        println!("Number of measurements that are larger than the previous measurement is {}", count);
    }
}