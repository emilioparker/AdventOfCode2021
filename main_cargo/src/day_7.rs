pub mod day_7_module
{
    use std::env;
    use std::fs;
    use std::collections::HashMap;

    
    pub fn run()
    {
        println!("day 7 exercise ------------");
        let content = fs::read_to_string("input7.txt").expect("Something went wrong");

        let split = content.split(",");
        let mut items: Vec<i32> = split
            .map(|x| x.parse :: <i32>().unwrap())
            .collect();

        let mut less_full_pos : Option<(i32, i32)> = None;

        for pos in &items
        {
            let aligned_pos = pos;
            let mut total_fuel = 0;
            for pos_2 in &items
            {
                let fuel_cost = (pos - pos_2).abs();
                total_fuel += fuel_cost;
            }

            // println!("testing fuel {}", total_fuel);

            match less_full_pos
            {
                Some((pos, fuel)) =>
                {
                    if fuel > total_fuel
                    {
                        less_full_pos = Some((*aligned_pos, total_fuel));
                    }
                },
                None => 
                {
                    less_full_pos = Some((*aligned_pos, total_fuel));
                }
            }
        }

        println!("result {:?}", less_full_pos);
    }
}