pub mod day_5_module
{
    use std::env;
    use std::fs;
    use std::collections::HashMap;

    #[derive(Debug, Clone)]
    struct LineSegment
    {
        x_1 : i32,
        y_1 : i32,
        x_2 : i32,
        y_2 : i32
    }

    pub fn run()
    {
        println!("day 5 exercise ----------------");
        let content = fs::read_to_string("input5.txt").expect("Something went wrong");

        //println!("text /n{}", content);

        let split = content.split("\n");
        let items: Vec<&str> = split.collect();

        let segments : Vec<LineSegment> = items.iter()
            // .split(" -> ")
            .map(|n| 
            {
                let pair : Vec<&str> = n.split(" -> ").collect();
                // let 
                // println!("{:?}", pair);
                let a = pair.get(0);
                let b = pair.get(1);
                match (a, b)
                {
                    (Some(a), Some(b)) => 
                    {
                        let coord_a : Vec<&str> = a.split(",").collect();
                        let coord_b : Vec<&str> = b.split(",").collect();
                        let x = coord_a.get(0).unwrap().parse::<i32>().unwrap();
                        let y = coord_a.get(1).unwrap().parse::<i32>().unwrap();

                        let x_2 = coord_b.get(0).unwrap().parse::<i32>().unwrap();
                        let y_2 = coord_b.get(1).unwrap().parse::<i32>().unwrap();
                        LineSegment{x_1 : x, y_1 : y, x_2, y_2}

                    },
                    _ => LineSegment{x_1 : -1, x_2 : -1,  y_1 : -1, y_2 : -1}
                }

                // n.parse::<i32>().unwrap()
            })
            .collect();

            let mut min_x = 1000;
            let mut max_x = 0;
            let mut min_y = 1000;
            let mut max_y = 0;

            for line in &segments
            {
                if line.x_1 > max_x { max_x = line.x_1}
                if line.x_2 > max_x { max_x = line.x_2}
                if line.y_1 > max_y { max_y = line.y_1}
                if line.y_2 > max_y { max_y = line.y_2}

                if line.x_1 < min_x { min_x = line.x_1}
                if line.x_2 < min_x { min_x = line.x_2}
                if line.y_1 < min_y { min_y = line.y_1}
                if line.y_2 < min_y { min_y = line.y_2}
            }

            let mut record : HashMap<(i32, i32), i32> = HashMap::new();

            // println!("{:?}", (min_x, max_x, min_y, max_y));

            for x in min_x .. (max_x + 1)
            {
                for segment in &segments
                {
                    // let valid = segment.x_1 == segment.x_2 || segment.y_1 == segment.y_2;
                    // if !valid
                    // {
                    //     continue;
                    // }

                    // println!("testing segment {:?} for x {}", segment, x);
                    let x_values = vec![segment.x_1, segment.x_2];
                    let &max = x_values.iter().max().unwrap();
                    let &min = x_values.iter().min().unwrap();

                    let y_values = vec![segment.y_1, segment.y_2];
                    let &max_y = y_values.iter().max().unwrap();
                    let &min_y = y_values.iter().min().unwrap();

                    if x >= min && x <= max 
                    {
                        let m =
                            if segment.x_1 == segment.x_2 
                            {
                                None
                            }
                            else
                            {
                                Some(((segment.y_2 - segment.y_1) as f32) / ((segment.x_2 - segment.x_1) as f32))
                            };
                        // println!("-------------  m = {:?}", m);

                        match m
                        {
                            Some(valid_m) => 
                            {
                                
                                let b = segment.y_1 as f32 - valid_m * segment.x_1 as f32;
                                let y = valid_m as f32 * x as f32 + b;
                                let decimal_part = y - y.floor();
                                // println!("b {} y {} decimal {}", b, y, decimal_part);
                                if decimal_part == 0.0 && y >= min_y as f32 && y <= max_y as f32
                                {
                                    // println!("valid coord {:?} ", (x, y as i32));
                                    let count = record.entry((x, y as i32)).or_insert(0);
                                    *count += 1;
                                }
                            },
                            None =>
                            {
                                for y in min_y .. (max_y + 1)
                                {
                                    // println!("valid coord {:?}", (x, y));
                                    let count = record.entry((x, y)).or_insert(0);
                                    *count += 1;
                                }
                            }
                        }
                    }
                }

            }


            for data in &record
            {
                let (coord, count) = data;
                // println!("{:?} is {}", coord, count);
            }

            let count = record.into_iter().filter(|(_, c)| c > &1).count();

            println!(" result  {} ", count);
    }
}