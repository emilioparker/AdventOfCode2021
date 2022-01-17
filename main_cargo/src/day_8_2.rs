pub mod day_8_2_module
{
    use std::env;
    use std::fs;
    use std::collections::HashMap;

    #[derive(Debug)]
    struct Reading<'a>
    {
        data : Vec<&'a str>,
        output : Vec<&'a str>
    }

    pub fn contains(container : &str, test : &str) -> bool
    {
        let chars : Vec<char> = container.chars().collect();
        let one_chars : Vec<char> = test.chars().collect();
        let mut is_contained = true;
        for c in one_chars
        {
            if !chars.contains(&c)
            {
                is_contained = false;
                break;
            }
        }
        is_contained
    }
    
    pub fn run()
    {
        println!("day 8 exercise 2 ------------");
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


        let converted_data : Vec<Reading> = data_frames
            .iter()
            .map(|x| 
                {
                    let output = x.0;
                    let readings : Vec<&str> = output.split(" ")
                        .filter(|x| x.trim().len() > 0)
                        .collect();
                    let output_2 = x.1;
                    let readings_2 : Vec<&str> = output_2.split(" ")
                        .filter(|x| x.trim().len() > 0)
                        .collect();
                    Reading{data : readings, output : readings_2}
                })
            .collect();

        let mut total = 0;

        for reading in &converted_data
        {
            let mut all_data : Vec<&str> = Vec :: new();
            all_data.extend(reading.data.clone());
            all_data.extend(reading.output.clone());

            // find the one

            // println!("data {:?}", all_data);

            let all_ones : Vec<&&str> = all_data.iter().filter(|x| x.len() == 2).collect();
            let one = all_ones.get(0).unwrap();

            let all_fours : Vec<&&str> = all_data.iter().filter(|x| x.len() == 4).collect();
            let four = all_fours.get(0).unwrap();

            let all_sevens : Vec<&&str> = all_data.iter().filter(|x| x.len() == 3).collect();
            let seven = all_sevens.get(0).unwrap();

            let all_eights : Vec<&&str> = all_data.iter().filter(|x| x.len() == 7).collect();
            let eight = all_eights.get(0).unwrap();



            let five_digits : Vec<&&str> = all_data.iter().filter(|x| x.len() == 5).collect();

            // find the 3
            let threes : Vec<&&&str> = five_digits.iter().filter(|x| 
                {
                    contains(x, one)
                })
                .collect();
            let three = threes.get(0).unwrap();


            let six_digits : Vec<&&str> = all_data.iter().filter(|x| x.len() == 6).collect();

            // find the 3
            let nines : Vec<&&&str> = six_digits.iter().filter(|x| 
                {
                    contains(x, three)
                })
                .collect();
            let nine = nines.get(0).unwrap();

            let six_digits_minus_nine : Vec<&&&str> = six_digits
            .iter()
            .filter(|x| x != nine)
            .collect();

            // find the 3
            let ceros : Vec<&&&&str> = six_digits_minus_nine.iter().filter(|x| 
                {
                    contains(x, one)
                })
                .collect();

            // println!("six digits minus nice is less {} {}", six_digits_minus_nine.len(), six_digits.len());
            let cero = ceros.get(0).unwrap();

            let six_digits_minus_nine_minus_cero : Vec<&&&&str> = six_digits_minus_nine
            .iter()
            .filter(|x| x != cero)
            .collect();
            let six = six_digits_minus_nine_minus_cero.get(0).unwrap();


            let fives : Vec<&&&str> = five_digits.iter().filter(|x| 
                {
                    contains(six, x)
                })
                .collect();
            let five = fives.get(0).unwrap();


            let twos : Vec<&&&str> = five_digits
            .iter()
            .filter(|x| x != five && x != three)
            .collect();
            let two = twos.get(0).unwrap();

            let mut display_data : HashMap<&str, i32> = HashMap :: new();
            // find the one
            // println!("cero {:?} one {:?} two {:?} three {:?} four {:?} five {:?} six {:?} seven {:?} eight {:?} nine {:?}", cero, one, two, three, four, five, six, seven, eight, nine);
            display_data.insert(one, 1);
            display_data.insert(two, 2);
            display_data.insert(three, 3);
            display_data.insert(four, 4);
            display_data.insert(five, 5);
            display_data.insert(six, 6);
            display_data.insert(seven, 7);
            display_data.insert(eight, 8);
            display_data.insert(nine, 9);

            // println!("conversion {:?}", display_data);

            let output : Vec<i32> = reading.output
                .iter()
                .map(|x| 
                    {
                        let mut number = 0;
                        for (code, n) in &display_data
                        {
                            // println!(" data {} {}", x, code);
                            if contains(code, x) && code.len() == x.len()
                            {
                                // println!("found");
                                number = *n;
                            }
                        }
                        number
                    })
                .collect();
            
            let mut number = 0;
            for (index, n) in output.iter().rev().enumerate()
            {
                let base : i32 = 10;
                number += n * base.pow(index as u32);
            }
            // println!("number {}", number);

            total += number;

            // break;
        }

        // for reading in &converted_data
        // {
        //     let output = reading.output
        //         .iter()
        //         .map(|x|
        //         {
        //             var converted 
        //         })
        // }


        // println!("reading {:?}", converted_data);
        println!("total {}", total);
    }
}