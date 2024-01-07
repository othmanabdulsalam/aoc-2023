use std::fs;

fn main() {
    // Read the input from the file 
    let file = fs::read_to_string("./src/bin/input.txt");

    let text : String; 

    match file {
        Ok(res) => text = res,
        Err(error) => { 
             panic!("{:?}", error);
        }
    }


    let output = text 
        .lines()
        .map(|line| {
            parse_line(line)
        })
        .sum::<u32>();
        
    println!("{}", output.to_string() ); 
    return;
}


fn parse_line( line: &str ) -> u32 
{
    let mut it =
        line.chars().filter_map(|character| {
            character.to_digit(10)
        });
    let first =
        it.next().expect("should be a number");

    match it.last() {
        Some(num) => format!("{first}{num}"),
        None => format!("{first}{first}"),
    }
    .parse::<u32>()
        .expect("should be a valid number")
}
