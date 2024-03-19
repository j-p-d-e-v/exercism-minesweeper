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

fn mine_surrounded_by_spaces() {
    #[rustfmt::skip]
    run_test(&[
        "111",
        "1*1",
        "111",
    ]);
}


fn space_surrounded_by_mines() {
    #[rustfmt::skip]
    run_test(&[
        "***",
        "*8*",
        "***",
    ]);
}
fn horizontal_line_mines_at_edges() {
    #[rustfmt::skip]
    run_test(&[
        "*1 1*",
    ]);
}
fn vertical_line() {
    #[rustfmt::skip]
    run_test(&[
        "1",
        "*",
        "2",
        "*",
        "1",
    ]);
}
fn vertical_line_mines_at_edges() {
    #[rustfmt::skip]
    run_test(&[
        "*",
        "1",
        " ",
        "1",
        "*",
    ]);
}

fn cross() {
    #[rustfmt::skip]
    run_test(&[
        " 2*2 ",
        "25*52",
        "*****",
        "25*52",
        " 2*2 ",
    ]);
}
fn large_board() {
    #[rustfmt::skip]
    run_test(&[
        "1*22*1",
        "12*322",
        " 123*2",
        "112*4*",
        "1*22*2",
        "111111",
    ]);
}

pub fn annotate(minefield: &[&str]) -> Vec<String> {

    let total_rows: usize = minefield.len();
    let mut result: Vec<String> = Vec::new();
    if total_rows > 0 {
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
                    line_field.push(cfield);
                }
                else{
                    let mut adjacents:Vec<usize> = Vec::from([cindex]);
                    if cindex > 0 {
                        adjacents.push(cindex - 1);
                    }
                    if cindex < fields.len() {
                        adjacents.push(cindex + 1);
                    }
                    adjacents.sort();
                    let mut total_mine_counter:usize = 0;
                    for pos in ["top","current","bottom"] {
                        let _pos_mine_locations: Vec<usize> = match pos {
                            "top" => {            
                                let mut top_row_mines: Vec<usize> = Vec::new();                        
                                if rindex > 0 {
                                    let top_row_index: usize = rindex - 1;
                                    if mine_locations.get(top_row_index) != None {
                                        top_row_mines = mine_locations.get(top_row_index).unwrap().to_vec();
                                    }
                                }
                                top_row_mines
                            },
                            "bottom" => {
                                let mut bottom_row_mines: Vec<usize> = Vec::new();
                                if rindex < total_rows {
                                    let bottom_row_index: usize = rindex + 1;
                                    if mine_locations.get(bottom_row_index) != None {
                                        bottom_row_mines = mine_locations.get(bottom_row_index).unwrap().to_vec();
                                    }
                                }
                                bottom_row_mines
                            },
                            _ => mine_locations.get(rindex).unwrap().to_vec()
                        };
                        if _pos_mine_locations.len() > 0 {
                            for adjacent in &adjacents {
                                if _pos_mine_locations.iter().any(|&i| &i == adjacent) {
                                    total_mine_counter += 1;
                                }
                            }
                        }
                    }
                    if total_mine_counter == 0 {
                        line_field.push_str(&" ");
                    }
                    else{
                        line_field.push_str(total_mine_counter.to_string().as_str());
                    }
                }
            }
            result.push(line_field);
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
    println!("mine_surrounded_by_spaces");
    mine_surrounded_by_spaces();
    println!("===============");
    println!("space_surrounded_by_mines");
    space_surrounded_by_mines();
    println!("===============");
    println!("===============");
    println!("vertical_line");
    vertical_line();
    println!("===============");
    println!("vertical_line_mines_at_edges");
    vertical_line_mines_at_edges();
    println!("===============");
    println!("cross");
    cross();
    println!("===============");
    println!("large_board");
    large_board();
    println!("===============");
    println!("horizontal_line_mines_at_edges");
    horizontal_line_mines_at_edges();
}