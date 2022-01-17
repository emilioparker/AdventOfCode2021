pub mod day_6_2_faster_module
{
    use std::env;
    use std::fs;
    use std::collections::HashMap;

    #[derive(Debug, Clone)]
    struct Fish
    {
        steps : i32,
        life : i32
    }

    struct Record
    {
        data : HashMap<(i32, i32), i64>
    }

    fn calculate_new_fish_count(life : i32, steps : i32 , record : Record) -> (i64, Record)
    {
        // let mut new_fish : Vec<Fish> = Vec :: new();
        let added = 6 - life;
        let new_fish_count = (steps + added) as f32 / 7.0;
        let rounded = new_fish_count.floor() as i32;

        // println!("added fish {}", rounded);

        let mut result_record = record;
       
        let mut count : i64= 1;
        for step in 0 .. rounded
        {
            let child_steps = steps - 7 * (step + 1) + added;
            match result_record.data.get(&(8, child_steps))
            {
                Some(c) => 
                {
                    // println!("found entry {}", c);
                    count += *c;
                },
                None =>
                {
                    // let fish = Fish{steps, life : 8};
                    let (new_count, updated_record) =  calculate_new_fish_count(8, child_steps, result_record);
                    result_record = updated_record;
                    result_record.data.insert((life, child_steps), new_count);
                    count += new_count;
                    // println!("add entry {}", count);
                } 
            }

        }
        // println!("--- added fish {:?}", new_fish);
        (count, result_record)
    }
    
    pub fn run()
    {
        println!("day 6 exercise 2 ------------");

        let content = fs::read_to_string("input6.txt").expect("Something went wrong");

        let split = content.split(",");
        let mut items: Vec<i32> = split
            .map(|x| x.parse :: <i32>().unwrap())
            .collect();

        println!(" initial state {:?} ", items);

        let mut total : i64 = 0;
        let steps = 256;

        let mut new_fish : Vec<Fish> = Vec :: new();

        for f in items
        {
            new_fish.push(Fish{life: f, steps : steps});
        }

        let mut record : Record = Record{ data : HashMap::new()};

        for fish in new_fish
        {
            let (count, result_record) = calculate_new_fish_count(fish.life, fish.steps, record);
            record = result_record;
            total += count;
        }
                
        println!("After  a bunch of days {:?} ", total);
    }
}