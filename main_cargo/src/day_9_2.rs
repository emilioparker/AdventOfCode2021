pub mod day_9_2_module
{
    use std::collections::HashSet;
    use std::env;
    use std::fs;
    use std::collections::HashMap;
    use std::fs::File;
    use std::io::Write;

    use array2d::Array2D;
    
    #[derive(Debug, Clone)]
    struct Basing
    {
        value : (i32, i32),
        to_be_explored : HashSet<(i32, i32)>,
        explored : HashSet<(i32, i32)>
    }

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

    fn explore_basing(basing : Basing, map : &Array2D<i32>) -> Basing
    {
        let border = basing.to_be_explored.clone();
        let mut updated_basing = basing;
        for border_point in border
        {
            let possible_elements = explore_basing_edge(border_point, map);
            updated_basing.explored.insert(border_point);
            updated_basing.to_be_explored.remove(&border_point);
            for possible_element in possible_elements
            {
                // let value = possible_element.0;
                let col = possible_element.1;
                let row = possible_element.2;
                if !updated_basing.explored.contains(&(col, row))
                {
                    updated_basing.to_be_explored.insert((col, row));
                }
            }
        }
        if updated_basing.to_be_explored.iter().count() > 0
        {
            return explore_basing(updated_basing, map);
        }
        else
        {
            return updated_basing; 
        }
    }

    fn explore_basing_edge(border_point : (i32, i32), map : &Array2D<i32>) -> Vec<(i32, i32, i32)>
    {
        let mut new_border_points: Vec<(i32, i32, i32)
        > = Vec:: new();
        let mut neighbors: Vec<(i32, i32)> = Vec :: new();
        let column = border_point.0;
        let row = border_point.1;
        neighbors.push((column, row - 1));
        neighbors.push((column, row + 1));
        neighbors.push((column - 1, row));
        neighbors.push((column + 1, row));

        
        let test_point = map.get(row as usize, column as usize);
        // println!("------- test {} n {}", test, neighbors.iter().count());
        for (column, row) in neighbors
        {
            let item = map.get(row as usize, column as usize);

            match (test_point, item)
            {
                (Some(t), Some(i)) =>
                {
                    // println!(" test {:?} at col:{} row:{}", (t, i), column, row);
                    if *t < *i && *i != 9
                    {
                        new_border_points.push((*i, column, row));
                    }
                }
                _ => {}
            }
        }
        new_border_points
    }
    
    pub fn run()
    {
        println!("day 9 exercise 2 ------------");
        let content = fs::read_to_string("input9.txt").expect("Something went wrong");
        let lines : Vec<&str> = content.split("\n").collect();
        let first_line = lines.first().unwrap();
        let size = first_line.chars().count();
        println!(" the size is {}", size);

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
        let mut basings: Vec<Basing> = Vec :: new(); 

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
                let mut to_be_explored : HashSet<(i32, i32)> = HashSet :: new();
                let mut explored : HashSet<(i32, i32)> = HashSet :: new();
                to_be_explored.insert((column, row));
                explored.insert((column, row));
                let basing = Basing {value : (column, row), to_be_explored, explored};
                basings.push(basing);
            }
        }
        // println!("{:?}", basings);
        let mut basing_sizes: Vec<i32> = Vec :: new();

        let mut updated_basings: Vec<Basing> = Vec :: new();
        for basing in basings
        {
            // println!("{:?}", basing);
            let explored_basing = explore_basing(basing, &map);
            // println!("result {:?}", explored_basing);
            basing_sizes.push(explored_basing.explored.iter().count() as i32);
            updated_basings.push(explored_basing);
            // break;
        }

        basing_sizes.sort();
        let reverted : Vec<i32> = basing_sizes.into_iter().rev().collect();
        // println!("result {:?}", reverted);
        let first_three_items: Vec<i32> = reverted.into_iter().take(3).collect();
        // println!("first three basings {:?}", first_three_items);
        let reduced = first_three_items.into_iter().reduce(|a, b| a * b);

        // println!("first three mul {:?}", reduced);

        let mut result_string = String :: new();
        
        result_string.push_str("<p style=\"font-family:Consolas; font-size:20px\">");
        let mut total = 0;

        for (index, item) in map.elements_row_major_iter().enumerate()
        {
            if index % size == 0
            {
                result_string.push_str("<br>");
            }

            let mut is_basing = false;
            let mut is_low_point = false;
            let row = (index / map.num_columns()) as i32;
            let column = (index % map.num_columns()) as i32;

            for basing in updated_basings.iter()
            {
                if basing.value == (column, row)
                {
                    is_low_point = true;
                }
                else if basing.explored.contains(&(column, row))
                {
                    is_basing = true;
                    break;
                }
            }
            if is_low_point
            {
                total += 1;
                result_string.push_str(&format!("<u>{}</u>", item.to_string()));
            }
            else if is_basing
            {
                total += 1;
                result_string.push_str(&format!("<b>{}</b>", item.to_string()));
            }
            else
            {
                result_string.push_str(&item.to_string());
            }
        }
        result_string.push_str("<p>");

        println!("result {}", total);
        let mut file = File::create("result_9.html").unwrap();
        file.write_all(result_string.as_bytes());

    }

    
}