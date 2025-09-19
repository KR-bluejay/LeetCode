struct Spreadsheet {
    sheet: Vec<Vec<i32>>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Spreadsheet {

    fn new(rows: i32) -> Self {
        let rows = rows as usize;
        let sheet: Vec<Vec<i32>> = vec![vec![0; 26]; rows + 1];

        Self {
            sheet
        }
    }

    fn get_cell_id(&self, cell: &str) -> (usize, usize) {
        let row_id: usize = cell[1 ..].parse::<usize>().unwrap() - 1;
        let cell_id = cell.chars().nth(0).unwrap() as usize - 'A' as usize;

        (row_id, cell_id)
    }
    fn set_cell(&mut self, cell: String, value: i32) {
        let mut express_num: i32 = 0;
        let mut is_right = false;
        let cells: Vec<&str> = cell.split('+').collect();

        let (row_id, col_id) = self.get_cell_id(cells[0]);
        let express_num = cells.get(1).unwrap_or(&"0").parse::<i32>().unwrap();


        self.sheet[row_id][col_id] = value + express_num;
    }
    
    fn reset_cell(&mut self, cell: String) {
        let (row_id, col_id) = self.get_cell_id(&cell);

        self.sheet[row_id][col_id] = 0;
    }

    
    fn get_value(&self, formula: String) -> i32 {
        let mut value: i32 = 0;
        let formula: String = formula[1..].to_string();
        let formulas: Vec<&str> = formula.split('+').collect();
        

        for formula_item in formulas {
            match formula_item.parse::<i32>() {
                Ok(num) => {
                    value += num;
                }
                Err(e) => {
                    let (row_id, cell_id) = self.get_cell_id(&formula_item);
                    value += self.sheet[row_id][cell_id];
                }
            }
        }

        value
    }
}

/**
 * Your Spreadsheet object will be instantiated and called as such:
 * let obj = Spreadsheet::new(rows);
 * obj.set_cell(cell, value);
 * obj.reset_cell(cell);
 * let ret_3: i32 = obj.get_value(formula);
 */