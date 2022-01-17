pub mod day_2_module
{
    use std::fs;


    #[derive(Debug)]
    enum Command
    {
        Forward(i32),
        Down(i32),
        Up(i32)
    }

    pub fn run()
    {
        println!("day 2 --------- ");
        let content = fs::read_to_string("input2.txt").expect("Something went wrong");
        let items : Vec<&str> = content.split("\n").collect();

        let mut commands : Vec<Command> = Vec::new();

        for element in items
        {
            let splitted_line : Vec<&str> = element.split(" ").collect();
            let command = splitted_line[0];
            let number_string = splitted_line[1];
            
            let number = number_string.parse :: <i32>().unwrap();

            if command == "forward"
            {
                commands.push(Command::Forward(number));
            }
            else if command == "down"
            {
                commands.push(Command::Down(number));
            }
            else if command == "up"
            {
                commands.push(Command::Up(number));
            }
        }

        let mut depth : i32 = 0;
        let mut horizontal_position : i32 = 0;

        for command in commands
        {
            // println!("{:?}", command);
            match command
            {
                Command :: Forward(n) => horizontal_position += n,
                Command :: Up(n) => depth -= n,
                Command :: Down(n) => depth += n,
            }
        }

        let result = depth * horizontal_position;
        println!("result is {} " , result);
    }
}