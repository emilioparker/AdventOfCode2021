pub mod day_9_module
{
    use std::env;
    use std::fs;
    use std::collections::HashMap;

    use array2d::Array2D;

    fn check_neighbors(map : &Array2D<i32>, test : &i32, neighbors : Vec<(i32, i32)>) -> bool
    {
        // println!("------- test {} n {}", test, neighbors.iter().count());
        for (column, row) in neighbors
        {
            let item = map.get(row as usize, column as usize);
            // println!("checking {:?} at {:?}", item, (column, row));
            match item
            {
                Some(r) => 
                {
                    if r <= test 
                    {
                        return false;
                    };
                },
                None => {}
            }
        }
        true
    }
    
    pub fn run()
    {
        println!("day 9 exercise ------------");
        let content = fs::read_to_string("input9.txt").expect("Something went wrong");
        let lines : Vec<&str> = content.split("\n").collect();
        let first_line = lines.first().unwrap();
        let size = first_line.chars().count();
        // println!(" the size is {}", size);

        let mut all_data : Vec<Vec<i32>> = Vec::new();

        for (index, line) in lines.iter().enumerate()
        {
            let items : Vec<i32> = line.chars().map(|c| c.to_string().parse :: <i32>().unwrap()).collect();
            all_data.push(items);
        }
        // let map = Array2D :: fill_with(0, size, size);
        let map: Array2D<i32> = Array2D :: from_rows(&all_data);

        // println!(" map {:?}", map);
        let mut final_result = 0;

        for (index, reading) in map.elements_row_major_iter().enumerate()
        {
            let row = (index / map.num_columns()) as i32;
            let column = (index % map.num_columns()) as i32;

            // println!("test item at {:?} item {}", (column, row), reading);

            // check if it is a low point
            let mut adjacent_points: Vec<(i32, i32)> = Vec :: new();
            adjacent_points.push((column, row - 1));
            adjacent_points.push((column, row + 1));
            adjacent_points.push((column - 1, row));
            adjacent_points.push((column + 1, row));

            let result = check_neighbors(&map, reading, adjacent_points);
            if result
            {
                // println!("at {:?} item {}", (column, row), reading);
                final_result += reading + 1;
            }

        }
        println!("result {}", final_result);



    }
}