mod state;
mod tests;

use state::State;

pub fn brainfuck_exec(program: &str) -> String {
    let mut state = State::<30_000>::new(program);
    let mut out: Vec<u8> = Vec::new();

    loop {
        let maybe_instruction = state.get_current_instruction();
        if maybe_instruction.is_none() {
            break;
        }
        let instruction = maybe_instruction.unwrap();
        match instruction {
            '>' => {
                state.move_next_instruction();
                state.cell_index += 1;
            }
            '<' => {
                state.move_next_instruction();
                state.cell_index -= 1;
            }
            '+' => {
                state.move_next_instruction();
                state.cells[state.cell_index] += 1;
            }
            '-' => {
                state.move_next_instruction();
                state.cells[state.cell_index] -= 1;
            }
            '.' => {
                state.move_next_instruction();
                out.push(state.get_current_cell());
            }
            '[' => {
                if state.get_current_cell() == 0 {
                    // skip the loop body while keeping track of child loop bodies.
                    let mut child_loop_counter: i64 = 1;

                    while child_loop_counter > 0 {
                        state.move_next_instruction();
                        let instr = state.get_current_instruction();
                        child_loop_counter += match instr {
                            Some('[') => 1,
                            Some(']') => -1,
                            _ => 0,
                        }
                    }

                    // move past the closing bracket.
                    state.move_next_instruction();
                } else {
                    // push the index of the loop start onto the stack.
                    // we will return here when we hit the closing bracket.
                    state.loop_stack.push(state.instruction_index);
                    state.move_next_instruction();
                }
            }
            ']' => {
                let start_index = state.loop_stack.pop();
                state.instruction_index = start_index.unwrap();
                assert_eq!(Some('['), state.get_current_instruction());
            }
            _ => {
                state.move_next_instruction();
            }
        }
    }

    return out.iter().map(|b| *b as char).collect::<String>();
}

fn main() {
    let program = r#"
    ++       Cell c0 = 2
    > +++++  Cell c1 = 5
    
    [        Start your loops with your cell pointer on the loop counter (c1 in our case)
    < +      Add 1 to c0
    > -      Subtract 1 from c1
    ]        End your loops with the cell pointer on the loop counter
    
    At this point our program has added 5 to 2 leaving 7 in c0 and 0 in c1
    but we cannot output this value to the terminal since it is not ASCII encoded
    
    To display the ASCII character "7" we must add 48 to the value 7
    We use a loop to compute 48 = 6 * 8
    
    ++++ ++++  c1 = 8 and this will be our loop counter again
    [
    < +++ +++  Add 6 to c0
    > -        Subtract 1 from c1
    ]
    < .        Print out c0 which has the value 55 which translates to "7"!
    "#;

    let result = brainfuck_exec(program);

    assert_eq!("7", result);

    println!("{}", result);
}
