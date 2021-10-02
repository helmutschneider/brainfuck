pub struct State<const N: usize> {
    pub cells: [u8; N],
    pub cell_index: usize,
    pub instruction_index: usize,
    pub loop_stack: Vec<usize>,
    program: Vec<char>,
}

impl<const N: usize> State<N> {
    pub fn new(program: &str) -> Self {
        return Self {
            cells: [0; N],
            cell_index: 0,
            instruction_index: 0,
            loop_stack: Vec::new(),
            program: program.chars().collect::<Vec<char>>(),
        };
    }

    pub fn get_current_instruction(&self) -> Option<char> {
        return self.program.get(self.instruction_index).map(|chr| *chr);
    }

    pub fn get_current_cell(&self) -> u8 {
        return self.cells[self.cell_index];
    }

    pub fn move_next_instruction(&mut self) {
        self.instruction_index += 1;
    }
}
