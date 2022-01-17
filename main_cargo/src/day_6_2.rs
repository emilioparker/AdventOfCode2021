pub mod day_6_2_module
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

    fn calculate_new_fish(current : Fish) -> Vec<Fish>
    {
        // println!("test fish {:?}", current);

        let mut new_fish : Vec<Fish> = Vec :: new();

        let added = 6 - current.life;

        let new_fish_count = (current.steps + added) as f32 / 7.0;

        let rounded = new_fish_count.floor() as i32;

        // println!("added fish {}", rounded);
        
        let mut previous_life = current.life;
        for step in 0 .. rounded
        {
            new_fish.push(Fish{steps : current.steps - 7 * (step + 1) + added, life : 8});
            // previous_life = current.steps - previous_life - 1;
        }
        // println!("--- added fish {:?}", new_fish);
        new_fish
    }
    
    pub fn run()
    {
        println!("day 6 exercise 2 ------------");

        let content = fs::read_to_string("input6_test.txt").expect("Something went wrong");

        let split = content.split(",");
        let mut items: Vec<i32> = split
            .map(|x| x.parse :: <i32>().unwrap())
            .collect();

        println!(" initial state {:?} ", items);

        let mut total : i64 = 0;
        let steps = 18;



        // let new_fish = calculate_new_fish(Fish{life: 6, steps});


        let mut new_fish : Vec<Fish> = Vec :: new();
        // let mut all_fish : Vec<Fish> = Vec :: new();

        for f in items
        {
            new_fish.push(Fish{life: f, steps : steps});
        }

        let mut current_pending_fish = new_fish;
        loop
        {
            // println!("count {}", current_pending_fish.len());
            if current_pending_fish.len() == 0
            {
                break;
            }

            let mut new_list : Vec<Fish> = Vec:: new();
            for fish in current_pending_fish
            {

                // all_fish.push(fish.clone());
                let mut childs = calculate_new_fish(fish);
                new_list.append(&mut childs);
                // println!("childs {} ", childs.len());
                total += 1;
            }

            current_pending_fish = new_list;

            // println!("new list {:?}", current_pending_fish);
            // println!("{:?}", all_fish);
        }




        // println!("{:?}", total);
                
        println!("After days {:?} ", total);
    }
}