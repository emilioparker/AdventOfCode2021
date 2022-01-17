pub mod day_4_2_module
{
    use std::env;
    use std::fs;

    #[derive(Clone, Debug)]
    struct Board
    {
        board : Vec<Vec<i32>>
    }

    fn sum_board(numbers : &[i32], board : &Board) -> i32
    {
        let mut total_sum = 0;
        // println!("----Test board {:?}", board.board);
        for row in &board.board
        {
            // println!(" row len {}", row.len());
            let count = row.len();
            let sum : i32 = row.iter().filter(|x| 
                {
                    // println!("testing if {} is contained in {:?}", x, numbers);
                    !numbers.contains(x)
                }).sum();
            // println!("valid row {}", valid);
            total_sum += sum;
            // return false;
        }

        total_sum
    }

    fn test_board(numbers : &[i32], board : &Board) -> bool
    {
        // println!("----Test board {:?}", board.board);
        for row in &board.board
        {
            // println!(" row len {}", row.len());
            let count = row.len();
            let valid : usize = row.iter().filter(|x| 
                {
                    // println!("testing if {} is contained in {:?}", x, numbers);
                    numbers.contains(x)
                }).count();
            // println!("valid row {}", valid);
            if valid == count
            {
                return true;
            }
            // return false;
        }
        // let gamma_string = gamma.iter().fold(String::new(), |acc, x| format!("{}{}", acc, x));

        // println!(" test columns");
        for i in 0 .. 5
        {
            let column : Vec<&i32>  = board.board.iter().fold(Vec::new(), |acc, x| 
            {
                let mut c = acc.clone();
                c.push(x.get(i).unwrap());
                c
            });

            let count = column.len();
            let valid : usize = column
                .iter()
                .filter(|x| numbers.contains(x))
                .count();

            // println!(" valid column {} ", valid);

            if valid == count
            {
                return true;
            }
        }

        false
    }

    pub fn run()
    {
        println!("day 4 exercise 2-----------");
        let content = fs::read_to_string("input4.txt").expect("Something went wrong");

        //println!("text /n{}", content);

        let split = content.split("\n");
        let items: Vec<&str> = split.collect();

        // let size = items[0].len();
        // let mut result = vec![0;size];
        // let mut count = 0;

        let chosen_number : Vec<i32> = items
            .first()
            .unwrap()
            .split(",")
            .map(|n| n.parse::<i32>().unwrap())
            .collect();
        
        // println!("{:?}", chosen_number);

        // let mut y = 0;
        // let mut current_board_index = 0;

        let mut boards : Vec<Board> = Vec :: new();
        let mut current_board : Board = Board {board: Vec :: new()};
        // boards.push(current_board.clone());

        // println!("---- generating boards");

        for item in items.iter().skip(2)
        {
            // println!("line ------- {}", item);
            if item.len() == 0
            {

                // println!("creating board");
                // this means we add what we have accumulated.
                // y = 0;
                // println!("{:?}", current_board);
                boards.push(current_board.clone());
                current_board = Board {board: Vec :: new()};

            }
            else
            {
                let row : Vec<i32> = item
                    .split(" ")
                    .filter(|x| x.len() > 0)
                    .map(|x| x.parse::<i32>().unwrap())
                    // .map(|x| 
                    //     {
                    //         match x
                    //         {
                    //             Ok(i) => Some(i),
                    //             Err(e) => None
                    //         }
                    //     })
                    .collect();

                // println!("{:?}", row);
                current_board.board.push(row);
            }
        }

        boards.push(current_board.clone());

        println!("----- calculting winners {}" ,boards.len());

        // println!("{:?}", boards);

        // println!(" for input {:?}", &chosen_number[0 .. 12]);
        // for (index, board) in boards .iter().enumerate()
        // {
        //     let result = test_board(&chosen_number[0 .. 12], board);
        //     println!(" result for {:?} is {}", board, result );
        //     // break;
        // }

        // let mut winners_count = 0;

        let mut last_board : Option<&Board> = None;

        'outer : for (index, _number) in chosen_number.iter().enumerate()
        {
            let current_numbers = &chosen_number[0 .. index];
            // println!(" for input {:?}", &chosen_number[0 .. index]);
            let losers : Vec<&Board> = boards
                .iter()
                .filter(|board|
                {
                    let result = test_board(current_numbers, board);
                    !result
                }).collect();
            
            if losers.len() == 1
            {
                last_board = Some(losers.get(0).unwrap());
            }

            if losers.len() == 0
            {
                match last_board
                {
                    Some(board) =>
                    {
                        let sum = sum_board(current_numbers, board);
                        let last_number = chosen_number.get(index - 1).unwrap();
                        println!("final result {} {} > {}", last_number, sum , (sum * last_number));
                    }
                    None => ()
                }

                break 'outer;
            }
        }
    }
}