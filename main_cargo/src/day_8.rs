pub mod day_8_module
{
    use std::env;
    use std::fs;
    use std::collections::HashMap;

    
    pub fn run()
    {
        println!("day 8 exercise ------------");
        let content = fs::read_to_string("input8.txt").expect("Something went wrong");
        let lines : Vec<&str> = content.split("\n").collect();

        let data_frames : Vec<(&str, &str)> = lines.iter()
            .map(|x| 
                {
                    let frame : Vec<&str> = x.split("|").collect();
                    (*frame.get(0).unwrap(), *frame.get(1).unwrap())
                })
            .collect();

        let mut record : HashMap<i32, i32> = HashMap::new();

        for data_frame in data_frames
        {
            let output = data_frame.1;
            let readings : Vec<&str> = output.split(" ")
                .filter(|x| x.trim().len() > 0)
                .collect();
            for reading in readings
            {
                let count = record.entry(reading.len() as i32).or_insert(0);
                *count += 1;
            }
        }

        println!("reading {:?}", record);

        let mut display_data : HashMap<i32, &str> = HashMap :: new();
        display_data.insert(2, "one");
        display_data.insert(4, "four");
        display_data.insert(3, "seven");
        display_data.insert(7, "eight");

        let mut total = 0;
        for (key, value) in record
        {
            let contained = display_data.contains_key(&key);
            if contained
            {
                total += value;
            }

        }
        println!("total {}", total);








        // let data = split_result.get(0).unwrap();
        // let result = split_result.get(1).unwrap().clone();
        // let mut items: Vec<&str> = result
        //     .split(" ")
        //     .filter(|x| x.len() > 0)
        //     .collect();
        // println!("output {:?}", data_frames);
    }
}