pub mod day_1_2_module
{
    use std::fs;

    pub fn run()
    {
        println!("day 1 part 2 exercise");
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

        let mut count : i32 = 0;

        for (index, _) in numbers.iter().enumerate()
        {
            if index > 2
            {
                let first_sum : i32 = numbers[index -3 .. index].iter().sum();
                let second_sum : i32 = numbers[index -2 .. index + 1].iter().sum();
                if second_sum > first_sum
                {
                    count = count + 1;
                }
            }
        }

        println!("Number of measurements that are larger than the previous measurement is {}", count);
    }
}