pub mod day_11_2_module
{
    use std::env;
    use std::fs;
    use std::collections::HashMap;
    use array2d::Array2D;

    pub fn run()
    {
        println!("day 11 exercise 2 ------------");

        let content = fs::read_to_string("input11.txt").expect("Something went wrong");
        let lines : Vec<&str> = content.split("\n").collect();
        let first_line = lines.first().unwrap();
        let size = first_line.chars().count();

        let mut all_data : Vec<Vec<i32>> = Vec::new();

        for (index, line) in lines.iter().enumerate()
        {
            let items : Vec<i32> = line.chars().map(|c| c.to_string().parse :: <i32>().unwrap()).collect();
            all_data.push(items);
        }
        
        let mut map: Array2D<i32> = Array2D :: from_rows(&all_data);
        println!("{:?}", map);
        println!("size {} {}", map.row_len(), map.column_len());
        let mut step = 0;
        // for n in 0..100
        'simulation_loop : loop
        {
            // map = apply_to_map(map, |col, row, working_map|  working_map);
            map = apply_to_map(map, increment_by_1);
            'reaction_loop : loop
            {
                let bright_spots = check_data(&map, check_for_bright_spot);
                if bright_spots > 0
                {
                    // println!("after step {} has bright spots {}", n, has_bright_spot);
                    map = apply_to_map(map, apply_reaction);
                }
                else
                {
                    break 'reaction_loop;
                }
            }

            let flashes = check_data(&map, check_for_flash);
            map = apply_to_map(map, clean_bright_spots);
            step += 1;
            if flashes == map.row_len() as i32 * map.column_len() as i32
            {
                println!("finished on step {}", step);
                break 'simulation_loop;
            }
            // println!("------- after step {}", n);
            // print_map(&map);
        }

        print_map(&map);
    }


    fn check_for_bright_spot(col : usize, row : usize, map: &Array2D<i32>) -> bool
    {
        let mut map = map;
        let element = map.get(row, col);
        if let Some(data) = element
        {
            if data == &10
            {
                return true;
            }
        }
        return false;
    }

    fn check_for_flash(col : usize, row : usize, map: &Array2D<i32>) -> bool
    {
        let mut map = map;
        let element = map.get(row, col);
        if let Some(data) = element
        {
            if data == &11
            {
                return true;
            }
        }
        return false;
    }

    fn increment_by_1(col : usize, row : usize, map: Array2D<i32>) -> Array2D<i32>
    {
        let mut map = map;
        let element = map.get_mut(row, col);
        if let Some(data) = element
        {
            if data < &mut 9
            {
                *data += 1;
            }
            else
            {
                *data = 10;
            }
        }
        map
    }

    fn clean_bright_spots(col : usize, row : usize, map: Array2D<i32>) -> Array2D<i32>
    {
        let mut map = map;
        let element = map.get_mut(row, col);
        if let Some(data) = element
        {
            if data == &mut 11 
            {
                *data = 0;
            }
        }
        map
    }

    fn apply_reaction(col : usize, row : usize, map: Array2D<i32>) -> Array2D<i32>
    {
        let mut map = map;
        let element = map.get_mut(row, col);
        if let Some(data) = element
        {
            if data == &mut 10
            {
                *data = 11;
                for i in 0..=2
                {
                    for j in 0..=2
                    {
                        let r = i as i32 - 1 + row as i32;
                        let c = j as i32 - 1 + col as i32;
                        let neighbor = map.get_mut( r as usize,  c as usize);
                        if let Some(data) = neighbor
                        {
                            if data < &mut 10 
                            {
                                *data += 1;
                            }
                        }
                    }
                }
            }
        }
        map
    }

    fn apply_to_map<F: Fn(usize, usize, Array2D<i32>) -> Array2D<i32>>(map: Array2D<i32>, operation: F) -> Array2D<i32>
    {
        let row_len = map.row_len();
        let col_len = map.column_len();
        let mut updated_map : Array2D<i32> = map;
        for row_index in 0..row_len
        {
            for col_index in 0..col_len
            {
                updated_map = operation(col_index, row_index, updated_map);
            }
        }
        return updated_map
    }

    fn check_data<F: Fn(usize, usize, &Array2D<i32>) -> bool>(map: &Array2D<i32>, operation: F) -> i32
    {
        let row_len = map.row_len();
        let col_len = map.column_len();
        let mut count = 0;
        for row_index in 0..row_len
        {
            for col_index in 0..col_len
            {
                let result = operation(col_index, row_index, map);
                if result
                {
                    count += 1;
                }
            }
        }
        count
    }

    fn print_map(map: &Array2D<i32>)
    {
        let row_len = map.row_len();
        let col_len = map.column_len();
        for row_index in 0..row_len
        {
            for col_index in 0..col_len
            {
                let element = map.get(row_index, col_index);
                if let Some(data) = element
                {
                    print!("{}", data);
                }
            }
            println!("");
        }
    }
}