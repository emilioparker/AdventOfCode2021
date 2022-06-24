pub mod day_12_module
{
    use std::env;
    use std::fs;
    use std::collections::HashMap;
    use array2d::Array2D;

    #[derive(Debug, Clone)]
    struct Node
    {
        id : String,
        links : Vec<String>
    }

    pub fn run()
    {
        println!("day 12 exercise ------------");

        let mut nodes_map : HashMap<&str, Node> = HashMap::new();
        let content = fs::read_to_string("input12.txt").expect("Something went wrong");
        let lines : Vec<&str> = content.split("\n").collect();
        for line in lines.iter()
        {
            println!("{}", line);
            let nodes : Vec<&str> = line.split("-").collect();
            for node_id in nodes.iter()
            {
                if !nodes_map.contains_key(node_id)
                {
                    let links : Vec<String> = Vec::new();
                    let node = Node{id : node_id.to_string(), links};
                    // node.id = *node_id;
                    nodes_map.insert(*node_id, node);
                }
            }
        }

        for line in lines.iter()
        {
            println!("{}", line);
            let nodes : Vec<&str> = line.split("-").collect();
            for node_id in nodes.iter()
            {
                for other_node_id in nodes.iter()
                {
                    if let Some(node) = nodes_map.get_mut(node_id)
                    {
                        if !node_id.eq(other_node_id)
                        {
                            if let Some(other_node) = nodes_map.get(other_node_id)
                            {
                                node.links.push(other_node.id.clone())
                            }
                        }
                    }
                }
            }
        }
    }
}