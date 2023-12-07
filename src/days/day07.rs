use std::{io, i32};
use crate::utils;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_examples() {
        let result = do_puzzle("day7_1_0.txt");
        match result{
            Ok(value) => {assert_eq!(value.0,6440);}
            Err(_error) =>{assert_eq!(1,2);}
        }
    }
    #[test]
    fn test_part2_examples() {
        let result = do_puzzle("day7_1_0.txt");
        match result{
            Ok(value) => {assert_eq!(value.1,5905);}
            Err(_error) =>{assert_eq!(1,2);}
        }
    }
}


pub fn solve() -> (i32,i32) {
    let result = do_puzzle("day7_1.txt"); 
    match result{
        Ok(value) => {return value}
        Err(error) =>{println!("Error occured:{}",error);}
    }
    
    (0,0)
}

fn do_puzzle(input: &str)-> Result<(i32,i32), io::Error>{
    let contents = utils::read_file(input)?;
    
    let mut hands: Vec<(&str,i32)> = contents.lines()
                                    .map(|line|
                                    (line.split_whitespace().nth(0).unwrap(),
                                     line.split_whitespace().nth(1).unwrap()
                                     .parse::<i32>().unwrap()))
                                     .collect();
                                
    hands.sort_by(|a, b|{
        let (x,y) = cmp_hands(*a, *b, 1);
        x.cmp(&y)
    });
    let mut result = 0;
    let mut i = 1;
    for hand in &hands{
        result += i*hand.1;
        i += 1;
    }
    hands.sort_by(|a, b|{
        let (x,y) = cmp_hands(*a, *b, 2);
        x.cmp(&y)
    });

    let mut result2 = 0;
    let mut i2 = 1;
    for hand in &hands{
        result2 += i2*hand.1;
        i2 += 1;
    }
    Ok((result,result2))
}

fn cmp_hands(a: (&str,i32) ,b: (&str,i32), part: usize) -> (i32,i32){
    let iter1 = a.0.chars();
    let iter2 = b.0.chars();
    let mut hand1: [i32; 15] = [0; 15];
    let mut hand2: [i32; 15] = [0; 15];

    for (char1, char2) in iter1.zip(iter2){
        if part == 1{
            hand1[get_card_num(char1)] += 1;
            hand2[get_card_num(char2)] += 1;
        }
        else if part == 2{
            hand1[get_card_num2(char1)] += 1;
            hand2[get_card_num2(char2)] += 1;
        }
    }
    let mut hand1_val = get_hand_value(hand1); 
    let mut hand2_val = get_hand_value(hand2); 
    
    let iter3 = a.0.chars();
    let iter4 = b.0.chars();
    if hand1_val == hand2_val{  //if the hand types are the same we go to high card decision
        for (char1, char2) in iter3.zip(iter4){
            let mut card1_val = 0;
            let mut card2_val = 0;
            if part == 1{
                card1_val = get_card_num(char1);
                card2_val = get_card_num(char2);
            } else if part == 2{
                card1_val = get_card_num2(char1);
                card2_val = get_card_num2(char2);
            }

            if card1_val > card2_val{
                hand1_val += 1;
                break;
            }
            if card2_val > card1_val{
                hand2_val += 1;
                break;
            }
        }
    }
    (hand1_val as i32,hand2_val as i32)
}

fn max_return(val: usize, new_val: usize) -> usize{
    if new_val > val{ return new_val}
    val
}

fn get_hand_value(hand: [i32; 15]) -> usize {
    let mut three_found = false;
    let mut two_found1 = false;
    let mut two_found2 = false;
    let mut max_return_val:usize = 0;
    let num_wild = hand[1];

    for i in 0..hand.len(){
        let card = hand[i];
        if i != 1{  //skip the jack
            (max_return_val,three_found,two_found1,two_found2) = match_card(card,three_found,two_found1,two_found2,max_return_val);
        }
    }
    
    match num_wild {
        0 =>{
            if three_found && two_found1{max_return_val =max_return(max_return_val,4)}   //full house
            if three_found{max_return_val =max_return(max_return_val,3)}    //three of a kind
            if two_found2{max_return_val =max_return(max_return_val,2)}      //two pair
            if two_found1{max_return_val =max_return(max_return_val,1)}     //one pair
            },
        1 =>{
            if max_return_val == 5{max_return_val =max_return(max_return_val,6)}    //if we had 4 of kind and a wild
            if three_found && two_found1{max_return_val =max_return(max_return_val,4)}   //full house
            if three_found{max_return_val =max_return(max_return_val,5)}    //4 of a kind
            if two_found2{max_return_val =max_return(max_return_val,4)}     //full house
            if two_found1{max_return_val =max_return(max_return_val,3)}     //3 of a kind
            max_return_val =max_return(max_return_val,1)                    //one pair
            },
        2 =>{
            if three_found {max_return_val =max_return(max_return_val,6)}   //if we had 3 found and
                                                                            //2 wild its now 5 of a kind
            if two_found1 {max_return_val =max_return(max_return_val,5)} //if we had 2 find is now4
            max_return_val =max_return(max_return_val,3)    //we are garuneted 3 of kind with 2wild
            },
        3 =>{
            if two_found1 {max_return_val =max_return(max_return_val,6)} //2 found + 3 wild
            max_return_val =max_return(max_return_val,5)    // 4 of a kind is garunteed
            },
        4 =>{
            max_return_val =max_return(max_return_val,6) //5 is garunteed
            },
        5 =>{
            max_return_val =max_return(max_return_val,6)//5 of a kind
            },
        _ => println!("This shouldn't happen"),

    }
    max_return_val
}

fn match_card(card: i32, a:bool,b:bool,c:bool,max_val:usize) -> (usize,bool,bool,bool){
    let mut three_found = a;
    let mut two_found1 = b;
    let mut two_found2 = c;
    let mut max_return_val:usize = max_val;
    match card {
        5 => max_return_val =max_return(max_return_val,6),    //5 of a kind
        4 => max_return_val =max_return(max_return_val,5),    //4 of a kind
        3 => three_found = true,    //found 3 of a kind
        2 => {
            if two_found1{two_found2 = true}    //if this is the second pair we found
            else {two_found1 = true}    //if this is the first pair we found
            },
        _ => {},
    }
    (max_return_val,three_found,two_found1,two_found2)
}

fn get_card_num(input: char) -> usize {
    match input {
        '2' => return 2,  
        '3' => return 3,  
        '4' => return 4,  
        '5' => return 5,  
        '6' => return 6,  
        '7' => return 7,  
        '8' => return 8,  
        '9' => return 9,  
        'T' => return 10,  
        'J' => return 11,  
        'Q' => return 12,  
        'K' => return 13,  
        'A' => return 14,  
         _  => return 2,  
    }
}

fn get_card_num2(input: char) -> usize {
    match input {
        '2' => return 2,  
        '3' => return 3,  
        '4' => return 4,  
        '5' => return 5,  
        '6' => return 6,  
        '7' => return 7,  
        '8' => return 8,  
        '9' => return 9,  
        'T' => return 10,  
        'J' => return 1,  
        'Q' => return 12,  
        'K' => return 13,  
        'A' => return 14,  
         _  => return 2,  
    }
}
