pub mod day_7_2_module
{
    use std::env;
    use std::fs;
    use std::collections::HashMap;

    
    pub fn run()
    {
        println!("day 7 exercise 2 ------------");
        let content = fs::read_to_string("input7.txt").expect("Something went wrong");

        let split = content.split(",");
        let mut items: Vec<i32> = split
            .map(|x| x.parse :: <i32>().unwrap())
            .collect();

        let mut less_full_pos : Option<(i32, i32)> = None;

        let min = &items.iter().min().unwrap();
        let max = &items.iter().max().unwrap();

        for pos in **min .. (**max + 1)
        {
            let aligned_pos = pos;
            // println!(" ------- testing {}", aligned_pos);

            let mut total_fuel = 0;

            for pos_2 in &items
            {
                let movement = (pos - pos_2).abs();
                let fuel_cost = (movement * (movement + 1)) as f32 * 0.5;
                // println!("from {} to {} cost {}", pos_2, pos, total_fuel);
                total_fuel += fuel_cost.floor() as i32;
            }


            match less_full_pos
            {
                Some((pos, fuel)) =>
                {
                    if fuel > total_fuel
                    {
                        less_full_pos = Some((aligned_pos, total_fuel));
                    }
                },
                None => 
                {
                    less_full_pos = Some((aligned_pos, total_fuel));
                }
            }
        }

        println!("result {:?}", less_full_pos);
    }
}