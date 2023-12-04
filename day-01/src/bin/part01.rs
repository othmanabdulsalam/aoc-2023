use std::fs::{self};

#[derive(Clone, Copy)]
enum CheckDigitType
{
    LeftMost,
    RightMost
}
#[derive(Clone,Copy)]
struct DigitStruct
{
    digit: u32,
    position: usize
}


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

    // begin parsing file from left to right, finding right most then left mose file 
    let mut total : u32 = 0;
    let lines = text.lines();
    
    let mut current_check_num = DigitStruct {
        digit : 0,
        position : 0
    };


    let mut current_most_num = DigitStruct {
        digit : 0,
        position : 0
    };

    // start loop
    for line in lines
    {
        let mut check_type = CheckDigitType::LeftMost;

        for (pos, value) in line.char_indices()
        {
            //check the char is a number 
            if value.is_digit(10)
            {
                current_check_num.digit = value.to_digit(10).unwrap();
                current_check_num.position = pos;

                if check_digit(current_check_num, current_most_num, check_type)
                {
                    //update current most to that value 
                    current_most_num = current_check_num.clone();
                    // first time it will be left most we are searching for, need to account for
                    // that
                    match check_type{
                        CheckDigitType::LeftMost => {
                            check_type = CheckDigitType::RightMost;
                            total += current_most_num.digit * 10;
                        }
                        _ => ()
                    }
                }
            }
        }
        total += current_most_num.digit;
    }

    println!("{}", total ); 
    return;
}





/// This function checks if the digit is the left or right most that is being searched for,
/// depending on what check type is passed in
fn check_digit( current_check_digit: DigitStruct, 
                current_most_digit:DigitStruct, check_type: CheckDigitType ) -> bool 
{
    match check_type {
        CheckDigitType::LeftMost => return true,
        CheckDigitType::RightMost => {
            ()
        }
    };



    if current_check_digit.position > current_most_digit.position
    {
        return true;
    }
    else {
        return false;
    }

}
