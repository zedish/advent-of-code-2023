use std::collections::HashSet;
use std::{io, collections::HashMap};
use crate::utils;
use uuid::Uuid;
use uuid::uuid;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_example1() {
        let result = do_puzzle("day22_1_1.txt");
        match result{
            Ok(value) => {assert_eq!(value.0,5);}
            Err(_error) =>{assert_eq!(1,2);}
        }
    }
}

#[derive(Clone)]
struct Block {
    x1: usize,
    y1: usize,
    z1: usize,
    x2: usize,
    y2: usize,
    z2: usize,
    // cur_z: usize,
    uuid: Uuid,
    suported_by: HashSet<Uuid>,
    suporting: HashSet<Uuid>,
}

pub fn solve() -> (i64,i64) {
    let result = do_puzzle("day22_1.txt"); 
    match result{
        Ok(value) => {return value}
        Err(error) =>{println!("Error occured:{}",error);}
    }
    
    (0,0)
}

fn do_puzzle(input: &str)-> Result<(i64,i64), io::Error>{
    let contents = utils::read_file(input)?;
    let ground = Uuid::new_v4();
    let mut max_x = 0;
    let mut max_y = 0;
    let mut max_z = 0;
    let mut blocks:HashMap<Uuid,Block> = contents.lines().map(|line|{
        let mut point1 = line.split("~").nth(0).unwrap().split(","); 
        let mut point2 = line.split("~").nth(1).unwrap().split(",");
        let tmp_block = Block {
            x1: (*point1.next().unwrap()).parse::<usize>().unwrap(),
            y1: (*point1.next().unwrap()).parse::<usize>().unwrap(),
            z1: (*point1.next().unwrap()).parse::<usize>().unwrap() - 1,
            x2: (*point2.next().unwrap()).parse::<usize>().unwrap(),
            y2: (*point2.next().unwrap()).parse::<usize>().unwrap(),
            z2: (*point2.next().unwrap()).parse::<usize>().unwrap() - 1,
            //subtract 1 from z to make indexing easier later
            // cur_z: 0,
            uuid: Uuid::new_v4(),
            suported_by: HashSet::new(),
            suporting: HashSet::new(),
        };
        max_x = max_x.max(tmp_block.x1.max(tmp_block.x2));
        max_y = max_y.max(tmp_block.y1.max(tmp_block.y2));
        max_z = max_z.max(tmp_block.z1.max(tmp_block.z2));
        (tmp_block.uuid,tmp_block)
    }).collect();

    println!("Max x:{}  Max y:{}  Max z:{}",max_x,max_y,max_z);

    // blocks.iter().for_each(|(_,block)|{print_block(&block)});
    // println!("=============================");
    // blocks.iter().for_each(|(_,block)|{print_block(&block)});
    // println!("=============================");


    let default_uuid = Uuid::new_v4();
    let mut world: Vec<Vec<Vec<Uuid>>> = vec![vec![vec![default_uuid;max_y+1];max_x+1];max_z+1];
    
    populate_world(&mut world, &blocks);
    gravity2(&mut world,&mut blocks,default_uuid,ground);
    
    blocks.iter_mut().for_each(|(_,block)|{
        find_supporting(world.clone(), block, default_uuid);
        // find_supported(world.to_vec(), block, default_uuid, ground);
    });

    println!("=============================");
    let mut tmp_blocks:Vec<Block> = blocks.iter().map(|(_,block)|{(*block).clone()}).collect();
    tmp_blocks.sort_by_key(|block|block.z1);
    let mut num = 0;
    let mut slice: Vec<Vec<char>> = vec![vec!['.';13];13];
    let slice_num = 1;
    tmp_blocks.iter_mut().for_each(|block|{
        num +=1;
        if block.z1.min(block.z2) == slice_num {
            print!("Sup:{} ",block.suporting.len());
            print_block(block);
            for x in block.x1.min(block.x2)..=block.x1.max(block.x2){
                for y in block.y1.min(block.y2)..=block.y1.max(block.y2){
                    if slice[x][y] == '.' {
                        slice[x][y] = '*';
                    }
                    else {
                        println!("error x1:{} y1:{} x2:{} y2:{}",block.x1,block.y1,block.x2,block.y2)
                    }
                }
            }
        }
    });
    slice.iter().for_each(|row|{
        row.iter().for_each(|val|{
            print!("{}",val);
        });
        println!("");
    });
    let mut slice: Vec<Vec<char>> = vec![vec!['.';10];10];
    tmp_blocks.iter_mut().for_each(|block|{
        num +=1;
        if block.z1.min(block.z2) == slice_num + 1 {
            print!("Sup:{} ",block.suporting.len());
            print_block(block);
            for x in block.x1.min(block.x2)..=block.x1.max(block.x2){
                for y in block.y1.min(block.y2)..=block.y1.max(block.y2){
                    if slice[x][y] == '.' {
                        slice[x][y] = '*';
                    }
                    else {
                        println!("error x1:{} y1:{} x2:{} y2:{}",block.x1,block.y1,block.x2,block.y2)
                    }
                }
            }
        }
    });
    slice.iter().for_each(|row|{
        row.iter().for_each(|val|{
            print!("{}",val);
        });
        println!("");
    });


    let mut top = 0;
    let mut anywhere = 0;
    //let mut removeable: HashSet<Uuid> = HashSet::new();
    let mut rev = 0;
    blocks.iter().for_each(|(_,block)|{
        //if block.suported_by.len() > 1 {
            //block.suported_by.iter().for_each(|val|{
            //    removeable.insert(*val);
            //    anywhere += 1;
           // });
        //}
        if block.suporting.len() == 0 {
            //removeable.insert(block.uuid);
            rev += 1;
            top += 1;
        }
        else {
            let mut bool = true;
            block.suporting.iter().for_each(|topblockuuid|{
                let topblock = blocks.get(topblockuuid).unwrap();
                
                if topblock.suported_by.len() == 1 {
                    bool = false;                
                }
                
            });
            if bool{
                //removeable.insert(block.uuid);
                anywhere += 1;
                rev += 1;
            }
        }
        if block.suported_by.len() == 0 {
            println!("shouldn't happen");
            print_block(block);
        }
    });


    println!("top:{}  anywhere:{}",top,anywhere); 
    println!("total blocks:{}",blocks.len()); 




    //let part1 = removeable.len() as i64;
    let part1 = rev as i64; 
    Ok((part1,0))
}

fn gravity2(world: &mut Vec<Vec<Vec<Uuid>>>, blocks: &mut HashMap<Uuid,Block>, defualt_uuid: Uuid, ground_uuid: Uuid){
    let mut sorted_block:Vec<Block> = blocks.iter().map(|(_,block)|{(*block).clone()}).collect();
    sorted_block.sort_by_key(|block|block.z1.min(block.z2));
    sorted_block.iter().for_each(|block_uuid|{
        let tmp_block = blocks.get_mut(&block_uuid.uuid).unwrap();
        println!("=============");
        print_block(tmp_block);
        let lowest_z = find_closest_bloc(world.to_vec(), tmp_block, defualt_uuid);
        let z_delta = lowest_z.abs_diff(tmp_block.z1.min(tmp_block.z2));
        for z in tmp_block.z1.min(tmp_block.z2)..=tmp_block.z1.max(tmp_block.z2){
            for x in tmp_block.x1.min(tmp_block.x2)..=tmp_block.x1.max(tmp_block.x2){
                for y in tmp_block.y1.min(tmp_block.y2)..=tmp_block.y1.max(tmp_block.y2){
                    world[z][x][y] = defualt_uuid;//reset block location
                    world[z - z_delta][x][y] = tmp_block.uuid;//reset block location
                }
            }
        }
        let z_diff = tmp_block.z1.abs_diff(tmp_block.z2);
        tmp_block.z1 = lowest_z;
        tmp_block.z2 = tmp_block.z1 + z_diff;
        find_supported(world.to_vec(), tmp_block, defualt_uuid, ground_uuid);
        print_block(tmp_block);
        println!("=============");
    });
}

fn find_supporting(world: Vec<Vec<Vec<Uuid>>>,block:&mut Block,defualt_uuid: Uuid){
    let max_z = block.z1.max(block.z2) + 1;
    if max_z >= world.len(){
        return
    }
    for x in block.x1.min(block.x2)..=block.x1.max(block.x2){
        for y in block.y1.min(block.y2)..=block.y1.max(block.y2){
            let cell_uuid = world[max_z][x][y];
            if cell_uuid != defualt_uuid && cell_uuid != block.uuid{
                block.suporting.insert(cell_uuid);
            }
        }
    }
}

fn find_supported(world: Vec<Vec<Vec<Uuid>>>,block:&mut Block,defualt_uuid: Uuid, ground_uuid: Uuid){
    if block.z1.min(block.z2) == 0{
        block.suported_by.insert(ground_uuid);
        return 
    }

    let mut found = false;
    let min_z = block.z1.min(block.z2) -1;
    for x in block.x1.min(block.x2)..=block.x1.max(block.x2){
        for y in block.y1.min(block.y2)..=block.y1.max(block.y2){
            let cell_uuid = world[min_z][x][y];
            if cell_uuid != defualt_uuid && cell_uuid != block.uuid{
                block.suported_by.insert(cell_uuid);
                found = true;
            }
        }
    }
    if !found {
        println!("this shouldn't happen");
    }

}

fn find_closest_bloc(world: Vec<Vec<Vec<Uuid>>>,block:&mut Block,defualt_uuid: Uuid) -> usize{
    let mut lowest_z = 0;
    let low_z = block.z1.min(block.z2);
    for x in block.x1.min(block.x2)..=block.x1.max(block.x2){
        for y in block.y1.min(block.y2)..=block.y1.max(block.y2){
            for z_dec in (0..low_z).rev(){
                let block_uuid = world[z_dec][x][y];
                if block_uuid != block.uuid && block_uuid != defualt_uuid{
                    lowest_z = lowest_z.max(z_dec+1);
                    break;
                }
            }
        }
    }

    lowest_z 
}

fn populate_world(world: &mut Vec<Vec<Vec<Uuid>>>, blocks:&HashMap<Uuid,Block>){
    blocks.iter().for_each(|(_,block)|{
        for z in block.z1.min(block.z2)..=block.z1.max(block.z2){
            for x in block.x1.min(block.x2)..=block.x1.max(block.x2){
                for y in block.y1.min(block.y2)..=block.y1.max(block.y2){
                    print!("{}",block.uuid);
                    world[z][x][y] = block.uuid;
                }
            }
        }
    });
} 


fn print_block(input: &Block) {
    print!("x:{} y:{} z:{}\t",input.x1,input.y1,input.z1);
    print!("x:{} y:{} z:{}\t",input.x2,input.y2,input.z2);
    println!("");
}

// fn gravity(world: &mut Vec<Vec<Vec<Uuid>>>, blocks: &mut HashMap<Uuid,Block>, defualt_uuid: Uuid, ground_uuid: Uuid){
//     world.clone().iter().enumerate().for_each(|(z_index, slice)|{
//         slice.iter().enumerate().for_each(|(x_index, row)|{
//             row.iter().enumerate().for_each(|(y_index,block_uuid)|{
//                 if *block_uuid != defualt_uuid {
//                     let tmp_block = blocks.get_mut(block_uuid).unwrap();
//                     if tmp_block.suported_by.len() == 0 {
//                         if tmp_block.z1.min(tmp_block.z2) == 0 {
//                             tmp_block.suported_by.insert(ground_uuid);
//                         } else { 
//                             let lowest_z = find_closest_bloc(world.to_vec(), tmp_block, defualt_uuid);
//                             for z in tmp_block.z1.min(tmp_block.z2)..=tmp_block.z1.max(tmp_block.z2){
//                                 for x in tmp_block.x1.min(tmp_block.x2)..=tmp_block.x1.max(tmp_block.x2){
//                                     for y in tmp_block.y1.min(tmp_block.y2)..=tmp_block.y1.max(tmp_block.y2){
//                                         world[z][x][y] = defualt_uuid;//reset block location
//                                         world[lowest_z + (z-tmp_block.z1.min(tmp_block.z2))][x][y] = tmp_block.uuid;//reset block location
//                                     }
//                                 }
//                             }
//                             tmp_block.cur_z = lowest_z;
//                             find_supported(world.to_vec(), tmp_block, defualt_uuid, ground_uuid);
//                         }
//                     }
//                 }
//             })
//         })
//     });
// }

