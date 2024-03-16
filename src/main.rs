fn remove_annotations(board: &[&str]) -> Vec<String> {
    board.iter().map(|r| remove_annotations_in_row(r)).collect()
}
fn remove_annotations_in_row(row: &str) -> String {
    row.chars()
        .map(|ch| match ch {
            '*' => '*',
            _ => ' ',
        })
        .collect()
}
fn run_test(test_case: &[&str]) {
    let cleaned = remove_annotations(test_case);
    let cleaned_strs = cleaned.iter().map(|r| &r[..]).collect::<Vec<_>>();
    let expected = test_case.iter().map(|&r| r.to_string()).collect::<Vec<_>>();
    let output = annotate(&cleaned_strs);
    println!("Input: {:?}",cleaned_strs);
    println!("Expected: {:?}",expected);
    println!("Output: {:?}",output);
    assert_eq!(expected, output);
}


fn no_rows() {
    #[rustfmt::skip]
    run_test(&[
    ]);
}


fn no_columns() {
    #[rustfmt::skip]
    run_test(&[
        "",
    ]);
}


fn no_mines() {
    #[rustfmt::skip]
    run_test(&[
        "   ",
        "   ",
        "   ",
    ]);
}


fn board_with_only_mines() {
    #[rustfmt::skip]
    run_test(&[
        "***",
        "***",
        "***",
    ]);
}

pub fn annotate(minefield: &[&str]) -> Vec<String> {

    let total_rows: usize = minefield.len();
    let mut result: Vec<String> = Vec::new();
    if total_rows > 0 {    
        if total_rows == 1 {
            if minefield[0] == "" {
                result.push(String::from(""))
            }
        }
        else{
            //1. Get the mines location
            let mut mine_locations:Vec<Vec<usize>> = Vec::new(); 
            for (rindex,fields) in minefield.iter().enumerate() {
                if mine_locations.get(rindex) == None {
                    let mines:Vec<usize> = Vec::new();
                    mine_locations.push(mines);
                }
                for (cindex,cfield) in fields.chars().enumerate() {
                    if cfield == '*' {
                        mine_locations[rindex].push(cindex);                     
                    }
                }
            }
            for (rindex,fields) in minefield.iter().enumerate() {
                let mut line_field: String = String::new();
                for (cindex,cfield) in fields.chars().enumerate() {
                    if cfield == '*' {
                        mine_locations[rindex].push(cindex);
                        line_field.push(cfield);
                    }
                    else{
                        let top_adjacent: Vec<usize> = match rindex {
                            i if i > 0 && total_rows > i => {
                                let top_rindex = rindex - 1;
                                mine_locations.get(top_rindex).unwrap().to_vec()
                            },
                            _ => vec![]
                        };
                        let bottom_adjacent: Vec<usize> = match rindex {
                            i if total_rows > i => {
                                let bottom_rindex = rindex + 1;
                                let bottom_mine_locations = mine_locations.get(bottom_rindex);
                                if bottom_mine_locations != None {
                                    bottom_mine_locations.unwrap().to_vec()
                                }
                                else{
                                    vec![]
                                }                                
                            },
                            _ => vec![]
                        };
                        println!("TOP ADJECENT: {:?}",top_adjacent);
                        println!("BOTTOM ADJECENT: {:?}",bottom_adjacent);
                    }
                }
                result.push(line_field);
            }
            println!("mine_locations: {:?}",mine_locations);   
        }
    }
    result
}


fn main(){
    println!("no_rows");
    no_rows();
    println!("===============");
    println!("no_columns");
    no_columns();
    println!("===============");
    println!("board_with_only_mines");
    board_with_only_mines();
    println!("===============");
    println!("no_mines");
    no_mines();
    println!("===============");
}