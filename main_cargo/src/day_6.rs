pub mod day_6_module
{
    use std::env;
    use std::fs;
    use std::collections::HashMap;
    
    pub fn run()
    {
        println!("day 6 exercise ------------");

        let content = fs::read_to_string("input6_test.txt").expect("Something went wrong");

        let split = content.split(",");
        let mut items: Vec<i32> = split
            .map(|x| x.parse :: <i32>().unwrap())
            .collect();

        println!(" initial state {:?} ", items);

        let steps = 80;

        for i in 0 .. steps
        {
            let mut new_items = 0;
            for f in 0 .. items.len()
            {
                if let Some(elem) = items.get_mut(f)
                {
                    if *elem == 0
                    {
                        *elem = 6;
                        new_items += 1;
                    }
                    else
                    {
                        *elem -= 1;
                    }
                }
            }

            for _ in 0 .. new_items 
            {
                items.push(8);
            }
        }
                
        println!("After  a bunch of days {:?} ", items.len());
    }
}