pub struct Board {
    pub grid: [[u8; 9]; 9],
    rows: [[bool; 9]; 9],
    cols: [[bool; 9]; 9], 
    subgrid: [[bool; 9]; 9],
}

impl Board {
    pub fn new() -> Self {
        Self {
            grid: [[0; 9]; 9],
            rows: [[false; 9]; 9],
            cols: [[false; 9]; 9],
            subgrid: [[false; 9]; 9],
        }
    }

    pub fn print(&self) {
        for row in &self.grid {
            for &num in row {
                if num == 0 {
                    print!(". ");
                } else {
                    print!("{num} ");
                }
            }
            println!();
        }
    }

    pub fn set(&mut self, row: usize, col: usize, val: u8) -> bool {
        if !(1..=9).contains(&val) {
            return false;
        }
        if !self.valid(row, col, val){
            return false;
        }

        let val_index = val as usize;
        let sg_index = (row / 3) * 3 + (col / 3);

        let cell = match self.grid.get_mut(row).and_then(|r| r.get_mut(col)) {
            Some(c) => c,
            None => return false,
        };
        *cell = val;

        let row_cell = match self.rows.get_mut(row).and_then(|r| r.get_mut(val_index)) {
            Some(c) => c,
            None => return false,
        };
        *row_cell = true;

        let col_cell = match self.cols.get_mut(col).and_then(|r| r.get_mut(val_index)) {
            Some(c) => c,
            None => return false,
        };
        *col_cell = true;

        let sg_cell = match self.subgrid.get_mut(sg_index).and_then(|r| r.get_mut(val_index)) {
            Some(c) => c,
            None => return false,
        };
        *sg_cell = true;

        true
    }

    pub fn unset(&mut self, row: usize, col: usize) -> bool {

        let sg_index = (row / 3) * 3 + (col / 3);

        let cell = match self.grid.get_mut(row).and_then(|r| r.get_mut(col)) {
            Some(c) => c,
            None => return false,
        };
        let val_index = *cell as usize;
        *cell = 0;

        let row_cell = match self.rows.get_mut(row).and_then(|r| r.get_mut(val_index)) {
            Some(c) => c,
            None => return false,
        };
        *row_cell = false;

        let col_cell = match self.cols.get_mut(col).and_then(|r| r.get_mut(val_index)) {
            Some(c) => c,
            None => return false,
        };
        *col_cell = false;

        let sg_cell = match self.subgrid.get_mut(sg_index).and_then(|r| r.get_mut(val_index)) {
            Some(c) => c,
            None => return false,
        };
        *sg_cell = false;

        true
    }

    pub fn get(&self, row: usize, col: usize) -> Option<u8> {
        self.grid
            .get(row)
            .and_then(|r| r.get(col))
            .copied()
    }

    pub fn valid(&self, row: usize, col: usize, value: u8) -> bool {
        if !(1..=9).contains(&value) {
            return false;
        }

        let sg_index = (row / 3) * 3 + (col / 3);

        if self.get(row, col) != Some(0)
        {
            return false;
        }

        let val_index = value as usize;

        if self.rows.get(row)
            .and_then(|r| r.get(val_index))
            .copied()
            .unwrap_or(true)
        {
            return false;
        }

        if self.cols.get(col)
            .and_then(|r| r.get(val_index))
            .copied()
            .unwrap_or(true)
        {
            return false;
        }

        if self.subgrid.get(sg_index)
            .and_then(|r| r.get(val_index))
            .copied()
            .unwrap_or(true)
        {
            return false;
        }

        true
    }
}

