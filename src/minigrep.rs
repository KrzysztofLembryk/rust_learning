use std::env;
use std::fs;

const QUERY_IDX: usize = 1;
const PATH_IDX: usize = 2;

pub fn grep_main()
{
    // 1) Reading command line arguments
    //      - We need to annotate args, since collect needs to know to which 
    //      collection we want
    //      - First arg is query, second is file path
    let args: Vec<String> = env::args().collect();
    if args.len() != 3
    {
        println!("Provide exactly two arguments");
        return;
    }

    let query = &args[QUERY_IDX];
    let file_path = &args[PATH_IDX];

}