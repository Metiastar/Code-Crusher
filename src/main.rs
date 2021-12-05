use std::{io::Write};

use rand::{thread_rng, Rng};


//#[derive(Clone,Copy)]
struct Board{
    board: Vec<Vec<i32>>,
    rows: usize,
    cols: usize,
    pieces: i32,
}
impl Default for Board{
    fn default() -> Board {
        Board {
            board: Vec::new(),
            rows: 6,
            cols: 8,
            pieces: 6,
        }
    }
}
impl Board{
    fn create_board(&mut self){
        let mut rng = thread_rng();
        for _i in 0..self.rows{
            let mut row = Vec::new();
            for _j in 0..self.cols{
                let x: i32 = rng.gen_range(0..(self.pieces));
                row.push(x);
            }
            //print!("{}", x);
            self.board.push(row);
        }
        //println!();
    }
    fn set_board(&mut self, b:Vec<Vec<i32>>){
        self.board = b;
    }
    fn set_rows(&mut self, rows:usize){
        self.rows = rows;
    }
    fn set_cols(&mut self, cols:usize){
        self.cols = cols;
    }

    //Display the board to the player
    fn read_board(&self){
        
        for x in 0..(self.rows){
            println!("{:?}  ", self.board[x]);
        }
        println!();
    }

    /*function dialogue to initiate swap_pieces
    * has checks for row and column upper limits and non-adjacent answers, but not row or column negative answers
    *
    * TODO: make 5 line clears happen before 4 and 3 space clears
    */
    fn ask_swap(&mut self){
        println!("What pieces would you like to swap?");
    
        //First Row/Column
        print!("Row 1: ");
        std::io::stdout().flush().unwrap();
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).ok().expect("failed to read line");
        let mut r1= line.trim();

        let mut x1= r1.parse::<usize>().expect("Could not parse first Row input"); 
        while x1 >= self.rows{//x1 boundary check
            println!("Please input a valid row using computer counting - start from 0 - Previous Input: {}", x1);
            print!("Row 1: ");
            std::io::stdout().flush().unwrap();
            let mut line = String::new();
            std::io::stdin().read_line(&mut line).ok().expect("failed to read line");
            r1= line.trim();

            x1= r1.parse::<usize>().expect("Could not parse first Row input");
        }


        print!("Column 1: ");
        std::io::stdout().flush().unwrap();
        line = String::new();
        std::io::stdin().read_line(&mut line).ok().expect("failed to read line");
        let mut c1 = line.trim();
    
        let mut y1= c1.parse::<usize>().expect("Could not parse first Col input");
        while y1 >= self.cols{//y1 boundary check
            println!("Please input a valid column using computer counting - start from 0 - Previous input: {}", y1);
            print!("Column 1: ");
            std::io::stdout().flush().unwrap();
            line = String::new();
            std::io::stdin().read_line(&mut line).ok().expect("failed to read line");
            c1 = line.trim();
        
            y1= c1.parse::<usize>().expect("Could not parse first Col input");
        }


        //Second Row/Column
        print!("Row 2: ");
        std::io::stdout().flush().unwrap();
        line = String::new();
        std::io::stdin().read_line(&mut line).ok().expect("failed to read line");
        let mut r2= line.trim();

        let mut x2= r2.parse::<usize>().expect("Could not parse second Row input");

        let mut negx = 100;
        match (x1 as i32 - 1) > 0{
            true => {negx = (x1 as i32 - 1) as usize;},
            false => {()}
        }
        while x2 != negx && x2 != x1+1 && x2 != x1 || x2 >= self.rows{//x2 adjacent check
            if x2 != x1 || x2 >= self.rows{
                println!("Invalid Row: Please input a valid row using computer counting - start from 0 - Previous Input: {}", x2);
            }else{
                println!("Invalid Move: Row {} is neither adjacent to or the same as Row {}", x2, x1);
            }
            
            print!("Row 2: ");
            std::io::stdout().flush().unwrap();
            line = String::new();
            std::io::stdin().read_line(&mut line).ok().expect("failed to read line");
            r2 = line.trim();

            x2 = r2.parse::<usize>().expect("Could not parse second Row input");
        }
        

        print!("Column 2: ");
        std::io::stdout().flush().unwrap();
        line = String::new();
        std::io::stdin().read_line(&mut line).ok().expect("failed to read line");
        let mut c2 = line.trim();

        let mut y2= c2.parse::<usize>().expect("Could not parse second Col input");

        let mut negy = 100;
        match (y1 as i32 - 1) >= 0{
            true => {negy = (y1 as i32 - 1) as usize;},
            false => {()}
        }
        if x1 != x2{ //y2 vertical adjacent check
            while y2 != y1 || y2 >= self.cols{
                if y2 >= self.cols{
                    println!("Invalid Column: Please input a valid column using computer counting - start from 0 - Previous input: {}", y2);
                }else{
                    println!("Invalid Move: Column chosen must be Column {}, otherwise it is not adjacent", y1);
                }

                print!("Column 2: ");
                std::io::stdout().flush().unwrap();
                line = String::new();
                std::io::stdin().read_line(&mut line).ok().expect("failed to read line");
                c2 = line.trim();

                y2 = c2.parse::<usize>().expect("Could not parse second Col input");
            }
        }else{ //y2 horizontal adjacent check
            while (y2 != negy && y2 != y1+1) || (y2 >= self.cols){
                if y2 >= self.cols{
                    println!("Invalid Column: please input a valid column using computer counting - start from 0 - Previous input: {}", y2);
                }else{
                    println!("Invalid Move: Column {} is not adjacent to Column {}", y2, y1);
                }
                
                print!("Column 2: ");
                std::io::stdout().flush().unwrap();
                line = String::new();
                std::io::stdin().read_line(&mut line).ok().expect("failed to read line");
                let c2 = line.trim();

                y2 = c2.parse::<usize>().expect("Could not parse second Col input");
            }
        }
        println!();

        self.attempt_swap(x1, y1, x2, y2);
    }
    pub fn attempt_swap(&mut self, x1: usize, y1: usize, x2: usize, y2: usize){
        
        let first_piece: i32 = self.board[x1][y1];
        let second_piece: i32 = self.board[x2][y2];
        self.board[x1][y1] = second_piece;
        self.board[x2][y2] = first_piece;

        //println!("Checking: Row {}, Col {}, Value {}\nChecking: Row {}, Col {}, Value {} ", x1, y1, second_piece, x2, y2, first_piece);

        //checks & clears special matching of pieces swapped
        let (s1, i, j) = self.special_check_match(x1, y1); let (s2, x, y) = self.special_check_match(x2, y2);

        match s1{//first piece matches
            6 => self.special_clear_5(i, j, 6),
            7 => self.special_clear_5(i, j, 7),
            _other =>{()}
        }
        match s2{//second piece matches
            6 => self.special_clear_5(x, y, 6),
            7 => self.special_clear_5(x, y, 7),
            _other =>{()}
        }
        

        //checks & and clears horizontal matching of the pieces swapped
        let (h1, i1) =self.h_check_match(x1, y1);  let (h2, i2) =self.h_check_match(x2, y2);
        
        match h1{//first piece matches
            5 => self.h_clear_5(x1, i1),
            4 => self.h_clear_small(x1, i1,4),
            3 => self.h_clear_small(x1, i1,3),
            _other => (),
        }
        match h2{//second piece matches
            5 => self.h_clear_5(x2, i2),
            4 => self.h_clear_small(x2, i2, 4),
            3 => self.h_clear_small(x2, i2, 3),
            _other => (),
        }
        
        

        //checks & clears vertical matching
        let (v1, j1) =self.v_check_match(x1, y1);  let (v2, j2) =self.v_check_match(x2, y2);

        match v1{//first piece matches
            5 => self.v_clear_5(j1, y1),
            4 => self.v_clear_small(j1, y1, 4),
            3 => self.v_clear_small(j1, y1, 3),
            _other => (),
        }
        match v2{//second piece matches
            5 => self.v_clear_5(j2, y2),
            4 => self.v_clear_small(j2, y2, 4),
            3 => self.v_clear_small(j2, y2, 3),
            _other => (),
        }
        

        //no piece matches
        if h1 == -1 && h2 == -1 && v1 == -1 && v2 == -1 && s1 == -1 && s2 == -1{
            self.board[x1][y1] = first_piece;
            self.board[x2][y2] = second_piece;
            println!("Invalid Move: No matches found, reverting swap");
        }else{
            while self.check_all_match(){
                self.check_all_match();
            }
        }
        
    }
    
    /*Functions for checking swap legitimacy:
    * h-match returns matched starting column 
    * v-match returns matched starting row 
    * both return how many pieces are matched or (-1,100) if none matched
    */
    pub fn h_check_match(&mut self, row:usize, col:usize) -> (i32, usize) {
        let check = self.board[row][col];
        let mut m = (-1, 100);

        if check == -1{//mainly for intial checks
            return m
        }

        //check to make sure negative indexes are in bounds
        let neg1_bool= (col as i32 -1) >= 0; let neg2_bool= (col as i32 -2) >= 0;
        let neg3_bool= (col as i32 -3) >= 0; let neg4_bool= (col as i32 -4) >= 0;
        let mut neg1 = 0; let mut neg2 = 0; let mut neg3 = 0; let mut neg4 = 0;
        match neg1_bool{ //give the neg# variables actual values if respective neg#_bools are true
            true =>{
                neg1 = (col as i32 -1) as usize;
                match neg2_bool{
                    true =>{
                        neg2 = (col as i32 -2) as usize;
                        match neg3_bool{
                            true =>{
                                neg3 = (col as i32 -3) as usize;
                                match neg4_bool{
                                    true =>{
                                        neg4 = (col as i32 -4) as usize;
                                    },
                                    false => {()},
                                }
                            },
                            false =>{()},
                        }
                    },
                    false =>{()},
                }
            },
            false => {()},
        }

        //check to make sure positive indexes are in bounds
        let pos1_bool= col+1 < self.cols; let pos2_bool= col+2 < self.cols;
        let pos3_bool= col+3 < self.cols; let pos4_bool= col+4 < self.cols;

        //check 5
        if pos4_bool{
            match check == self.board[row][col+1] && check == self.board[row][col+2] && check == self.board[row][col+3] && check == self.board[row][col+4]{
                true => {m = (5, col); return m},
                false => {()},
            }
        }
        if neg1_bool && pos3_bool{
            match check == self.board[row][neg1] && check == self.board[row][col+1] && check == self.board[row][col+2] && check == self.board[row][col+3]{
                true => {m = (5, neg1); return m},
                false => {()},
            }
        }
        if neg2_bool && pos2_bool{
            match check == self.board[row][neg2] && check == self.board[row][neg1] && check == self.board[row][col+1] && check == self.board[row][col+2]{
                true => {m = (5, neg2); return m},
                false => {()},
            }
        }
        if neg3_bool && pos1_bool{
            match check == self.board[row][neg3] && check == self.board[row][neg2] && check == self.board[row][neg1] && check == self.board[row][col+1]{
                true => {m = (5, neg3); return m},
                false => {()},
            }
        }
        if neg4_bool{
            match check == self.board[row][neg4] && check == self.board[row][neg3] && check == self.board[row][neg2] && check == self.board[row][neg1]{
                true => {m = (5, neg4); return m},
                false => {()},
            }
        }

        //check 4
        if pos3_bool{
            match check == self.board[row][col+1] && check == self.board[row][col+2] && check == self.board[row][col+3]{
                true => {m = (4, col); return m},
                false => {()},
            }
        }
        if neg1_bool && pos2_bool{
            match check == self.board[row][neg1] && check == self.board[row][col+1] && check == self.board[row][col+2]{
                true => {m = (4, neg1); return m},
                false => {()},
            }
        }
        if neg2_bool && pos1_bool{
            match check == self.board[row][neg2] && check == self.board[row][neg1] && check == self.board[row][col+1]{
                true => {m = (4, neg2); return m},
                false => {()},
            }
        }
        if neg3_bool{
            match check == self.board[row][neg3] && check == self.board[row][neg2] && check == self.board[row][neg1]{
                true => {m = (4, neg3); return m},
                false => {()},
            }
        }

        //check 3
        if pos2_bool{
            match check == self.board[row][col+1] && check == self.board[row][col+2]{
                true => {m = (3, col); return m},
                false => {()},
            }
        }
        if neg1_bool && pos1_bool{
            match check == self.board[row][neg1] && check == self.board[row][col+1]{
                true => {m = (3, neg1); return m},
                false => {()},
            }
        }
        if neg2_bool{
            match check == self.board[row][neg2] && check == self.board[row][neg1]{
                true => {m = (3, neg2); return m},
                false => {()},
            }
        }
        return m

        /*When the index pertains to a long vector and not a vector within a vector
        
        let initial = (piece/self.cols)*self.rows;
        
        let mut m:(i32, usize) = (-1, 100);

        for i in 0..(self.cols-2){//can't get horizontal matches with just 2 spaces

            let index = initial + i;
            let value = self.board[index];

            //check if not already matched, then that we're not going out of bounds, then if everything matches
            if value != -1 && self.cols >= 5 && (index%self.cols)+4 <= self.cols && 
            value == self.board[index+1] && value==self.board[index+2] &&
            value==self.board[index+3] && value==self.board[index+4]{
                
                m = (5, index);
                return m
            }else if value != -1 && self.cols >= 4 && (index%self.cols)+3 <= self.cols && 
            value == self.board[index+1] && value==self.board[index+2] &&
            value==self.board[index+3]{

                m = (4, index);
                return m;
            }else if value != -1 && self.cols >= 3 && (index%self.cols)+2 <= self.cols && 
            value == self.board[index+1] && value==self.board[index+2]{
                
                m = (3, index);
                return m;
            }else{

                println!("No match found || index: {}, value: {}", index, value);
                m = (-1, 100);}

        }

        m*/
    }
    pub fn v_check_match(&mut self, row:usize, col:usize) -> (i32, usize){
        let check = self.board[row][col];
        let mut m = (-1, 100);

        if check == -1{
            return m
        }

        //check to make sure negative indexes are in bounds
        let neg1_bool= (row as i32 -1) >= 0; let neg2_bool= (row as i32 -2) >= 0;
        let neg3_bool= (row as i32 -3) >= 0; let neg4_bool= (row as i32 -4) >= 0;
        let mut neg1 = 0; let mut neg2 = 0; let mut neg3 = 0; let mut neg4 = 0;
        match neg1_bool{ //give the neg# variables actual values if respective neg#_bools are true
            true =>{
                neg1 = (row as i32 -1) as usize;
                match neg2_bool{
                    true =>{
                        neg2 = (row as i32 -2) as usize;
                        match neg3_bool{
                            true =>{
                                neg3 = (row as i32 -3) as usize;
                                match neg4_bool{
                                    true =>{
                                        neg4 = (row as i32 -4) as usize;
                                    },
                                    false => {()},
                                }
                            },
                            false =>{()},
                        }
                    },
                    false =>{()},
                }
            },
            false => {()},
        }

        //check to make sure positive indexes are in bounds
        let pos1_bool= row+1 < self.rows; let pos2_bool= row+2 < self.rows;
        let pos3_bool= row+3 < self.rows; let pos4_bool= row+4 < self.rows;

        //check 5
        if pos4_bool{
            match check == self.board[row+1][col] && check == self.board[row+2][col] && check == self.board[row+3][col] && check == self.board[row+4][col]{
                true => {m = (5, row); return m},
                false => {()},
            }
        }
        if neg1_bool && pos3_bool{
            match check == self.board[neg1][col] && check == self.board[row+1][col] && check == self.board[row+2][col] && check == self.board[row+3][col]{
                true => {m = (5, neg1); return m},
                false => {()},
            }
        }
        if neg2_bool && pos2_bool{
            match check == self.board[neg2][col] && check == self.board[neg1][col] && check == self.board[row+1][col] && check == self.board[row+2][col]{
                true => {m = (5, neg2); return m},
                false => {()},
            }
        }
        if neg3_bool && pos1_bool{
            match check == self.board[neg3][col] && check == self.board[neg2][col] && check == self.board[neg1][col] && check == self.board[row+1][col]{
                true => {m = (5, neg3); return m},
                false => {()},
            }
        }
        if neg4_bool{
            match check == self.board[neg4][col] && check == self.board[neg3][col] && check == self.board[neg2][col] && check == self.board[neg1][col]{
                true => {m = (5, neg3); return m},
                false => {()},
            }
        }

        //check 4
        if pos3_bool{
            match check == self.board[row+1][col] && check == self.board[row+2][col] && check == self.board[row+3][col]{
                true => {m = (4, row); return m},
                false => {()},
            }
        }
        if neg1_bool && pos2_bool{
            match check == self.board[neg1][col] && check == self.board[row+1][col] && check == self.board[row+2][col]{
                true => {m = (4, neg1); return m},
                false => {()},
            }
        }
        if neg2_bool && pos1_bool{
            match check == self.board[neg2][col] && check == self.board[neg1][col] && check == self.board[row+1][col]{
                true => {m = (4, neg2); return m},
                false => {()},
            }
        }
        if neg3_bool{
            match check == self.board[neg3][col] && check == self.board[neg2][col] && check == self.board[neg1][col]{
                true => {m = (4, neg3); return m},
                false => {()},
            }
        }


        //check 3
        if pos2_bool{
            match check == self.board[row+1][col] && check == self.board[row+2][col]{
                true => {m = (3, row); return m},
                false => {()},
            }
        }
        if neg1_bool && pos1_bool{
            match check == self.board[neg1][col] && check == self.board[row+1][col]{
                true => {m = (3, neg1); return m},
                false => {()},
            }
        }
        if neg2_bool{
            match check == self.board[neg2][col] && check == self.board[neg1][col]{
                true => {m = (3, neg2); return m},
                false => {()},
            }
        }
        return m

        /*When index pertains to a long vector instead of vector within vector
        let col = piece%self.cols;
        let mut m:(i32, usize) = (-1, 100);

        for i in 0..self.rows-2{//also can't match with just 2 pieces
            let index = col + (i*self.cols);
            let value = self.board[index];
            
            if value != -1 && self.rows >= 5 && col+4 <= self.rows && 
            value == self.board[index+self.cols] && value==self.board[index+(2*self.cols)] &&
            value==self.board[index+(3*self.cols)] && value==self.board[index+(4*self.cols)]{

                m = (5, index);
                return m;
            }else if value != -1 && self.rows >= 5 && col+4 <= self.rows && 
            value == self.board[index+self.cols] && value==self.board[index+(2*self.cols)] &&
            value==self.board[index+(3*self.cols)]{

                m = (4, index);
                return m;
            }else if value != -1 && self.rows >= 5 && col+4 <= self.rows && 
            value == self.board[index+self.cols] && value==self.board[index+(2*self.cols)]{

                m = (3, index);
                return m;
            }else{
                m =(-1, 100);
            }

        }
        m*/
    }

    /*Special match legitimacy check:
    * checks for upright T matches
    * checks for upright L matches
    * returns top-leftmost piece if matched, (-1, 100) if not
    */
    pub fn special_check_match(&mut self, row:usize, col:usize) -> (i32, usize, usize){//first usize is top row, second is leftmost column
        let check = self.board[row][col];
        let mut m = (-1, 100, 100);

        if check == -1{
            return m
        }

        let neg_c1_bool = (col as i32 - 1) >= 0; let neg_c2_bool = (col as i32 - 2) >= 0;//negative column bools
        let neg_r1_bool = (row as i32 - 1) >= 0; let neg_r2_bool = (row as i32 - 2) >= 0;//negative row bools
        let mut neg_c1 = 0; let mut neg_c2= 0; let mut neg_r1= 0; let mut neg_r2 = 0;
        match neg_c1_bool{
            true =>{
                neg_c1 = (col as i32 - 1) as usize;
                match neg_c2_bool{
                    true => {neg_c2 = (col as i32 - 2) as usize;}
                    false => {()}
                }
            }
            false => {()}
        }
        match neg_r1_bool{
            true =>{
                neg_r1 = (row as i32 - 1) as usize;
                match neg_r2_bool{
                    true => {neg_r2 = (row as i32 - 2) as usize;}
                    false => {()}
                }
            }
            false => {()}
        }

        let pos_c1_bool = (col as i32 + 1) < self.cols as i32; let pos_c2_bool = (col as i32 + 2) < self.cols as i32;//positive column bools
        let pos_r1_bool = (row as i32 + 1) < self.rows as i32; let pos_r2_bool = (row as i32 + 2) < self.rows as i32;//positive row bools

        //check T
        if pos_c2_bool && pos_r2_bool{//top-leftmost piece
            match check == self.board[row][col+1] && check == self.board[row][col+2] && check == self.board[row+1][col+1] && check == self.board[row+2][col+1]{
                true =>{ m = (6, row, col)}
                false => {()}
            }
        }
        if neg_c1_bool && pos_c1_bool && pos_r2_bool{//top-middle piece
            match check == self.board[row][neg_c1] && check == self.board[row][col+1] && check == self.board[row+1][col] && check == self.board[row+2][col]{
                true =>{ m = (6, row, neg_c1)}
                false => {()}
            }
        }
        if neg_c2_bool && pos_r2_bool{//top-rightmost piece
            match check == self.board[row][neg_c2] && check == self.board[row][neg_c1] && check == self.board[row+1][neg_c1] && check == self.board[row+2][neg_c1]{
                true =>{ m = (6, row, neg_c2);}
                false =>{()}
            }
        }
        if neg_c1_bool && pos_c1_bool && neg_r1_bool && pos_r1_bool{//middle piece
            match check == self.board[neg_r1][neg_c1] && check == self.board[neg_r1][col] && check == self.board[neg_r1][col+1] && check == self.board[row+1][col]{
                true =>{ m = (6, neg_r1, neg_c1)}
                false =>{()}
            }
        }
        if neg_c1_bool && pos_c1_bool && neg_r2_bool{//bottom piece
            match check == self.board[neg_r2][neg_c1] && check == self.board[neg_r2][col] && check == self.board[neg_r2][col+1] && check == self.board[neg_r1][col]{
                true =>{ m = (6, neg_r2, neg_c1)}
                false =>{()}
            }
        }

        //check L
        if pos_r2_bool && pos_c2_bool{//top piece
            match check == self.board[row+1][col] && check == self.board[row+2][col] && check == self.board[row+2][col+1] && check == self.board[row+2][col+2]{
                true =>{ m = (7, row, col)}
                false =>{()}
            }
        }
        if neg_r1_bool && pos_r1_bool && pos_c2_bool{//middle piece
            match check == self.board[neg_r1][col] && check == self.board[row+1][col] && check == self.board[row+1][col+1] && check == self.board[row+1][col+2]{
                true =>{ m = (7, neg_r1, col)}
                false =>{()}
            }
        }
        if neg_r2_bool && pos_c2_bool{//bottom-left
            match check == self.board[neg_r2][col] && check == self.board[neg_r1][col] && check == self.board[row][col+1] && check == self.board[row][col+2]{
                true =>{ m = (7, neg_r2, col)}
                false =>{()}
            }
        }
        if neg_r2_bool && neg_c1_bool && pos_c1_bool{//bottom-middle
            match check == self.board[neg_r2][neg_c1] && check == self.board[neg_r1][neg_c1] && check == self.board[row][neg_c1] && check == self.board[row][col+1]{
                true => { m = (7, neg_r2, neg_c1)}
                false => {()}
            }
        }
        if neg_r2_bool && neg_c2_bool{//bottom-right
            match check == self.board[neg_r2][neg_c2] && check == self.board[neg_r1][neg_c2] && check == self.board[row][neg_c2] && check == self.board[row][neg_c1]{
                true =>{ m = (7, neg_r2, neg_c2)}
                false =>{()}
            }
        }
        
        return m
    }


    /*Functions for clearing out board matches:
    * h-functions need the row & starting column
    * v-functions need the column & starting row
    * special clear: m-type is either 6 -> T match, or 7 -> L match
    *  -T & L match requires starting row, and leftmost column
    */
    pub fn special_clear_5(&mut self, row:usize, col:usize, m_type:i32){
        
        if m_type == 6{ // T match
            self.board[row][col] = -1; self.board[row][col+2] = -1; self.board[row+1][col+1] = -1; self.board[row+2][col+1] = -1;
            self.board[row][col+1] = self.pieces;
            println!("Special T match! Row {}, Column {}: special number {} added", row, col, self.pieces);
        }else{ // L match
            self.board[row][col] = -1; self.board[row+1][col] = -1; self.board[row+2][col+1] = -1; self.board[row+2][col+2] = -1;
            self.board[row+2][col] = self.pieces;
            println!("Special L match! Row {}, Column {}: special number {} added", row, col, self.pieces);
        }

        //fill in the empty spaces
        let mut need_swap = self.swap_and_fill();
        while need_swap{
            need_swap = self.swap_and_fill();
        }
        
        self.read_board();
        println!();
    }
    
    pub fn h_clear_5(&mut self, row:usize, col:usize){
        self.board[row][col] = -1; self.board[row][col+1] = -1; self.board[row][col+3] = -1; self.board[row][col+4] = -1;
        self.board[row][col+2] = self.pieces;
        println!("Matched 5! Row {}, Column {}: special number {} added", row, col, self.pieces);

        //fill in the empty spaces
        let mut need_swap = self.swap_and_fill();
        while need_swap{
            need_swap = self.swap_and_fill();
        }
        
        self.read_board();
        println!();
    }
    pub fn h_clear_small(&mut self, row:usize, col:usize, size:i32){
        if size == 4 {
            self.board[row][col] = -1; self.board[row][col+1] = -1; self.board[row][col+2] = -1; self.board[row][col+3] = -1;
            println!("Matched 4! Row {}, Column {}", row, col);
        }else{
            self.board[row][col] = -1; self.board[row][col+1] = -1; self.board[row][col+2] = -1;
            println!("Matched 3! Row {}, Column {}", row, col);
        }

        //fill in the empty spaces
        let mut need_swap = self.swap_and_fill();
        while need_swap{
            need_swap = self.swap_and_fill();
        }
        
        self.read_board();
        println!();
    }

    pub fn v_clear_5(&mut self, row:usize, col:usize){
        self.board[row][col] = -1; self.board[row+1][col] = -1; self.board[row+3][col] = -1; self.board[row+4][col] = -1;
        self.board[row+2][col] = self.pieces;
        println!("Matched 5!  Row {}, Column {}: special number {} added", row, col, self.pieces);

        //fill in the empty spaces
        let mut need_swap = self.swap_and_fill();
        while need_swap{
            need_swap = self.swap_and_fill();
        }

        self.read_board();
        println!();
    }
    pub fn v_clear_small(&mut self, row:usize, col:usize, size:i32){
        if size == 4 {
            self.board[row][col] = -1; self.board[row+1][col] = -1; self.board[row+2][col] = -1; self.board[row+3][col] = -1;
            println!("Matched 4! Row {}, Column {}", row, col);
        }else{
            self.board[row][col] = -1; self.board[row+1][col] = -1; self.board[row+2][col] = -1;
            println!("Matched 3! Row {}, Column {}", row, col);
        }

        //fill in the empty spaces
        let mut need_swap = self.swap_and_fill();
        while need_swap{
            need_swap = self.swap_and_fill();
        }

        self.read_board();
        println!();
    }
    
    //For clearing out initial boards and boards that gain more matches after new pieces are spawned
    fn check_all_match(&mut self) -> bool{

        for i in 0..self.rows{
            for j in 0..self.cols{
                
                let (s,x,y) = self.special_check_match(i, j);
                match s{
                    6 => {self.special_clear_5(x, y, 6); return true},
                    7 => {self.special_clear_5(x, y, 7); return true},
                    _other =>{()}
                }

                let (h, y1) = self.h_check_match(i, j);
                match h{
                    5 => {self.h_clear_5(i, y1); return true},
                    4 => {self.h_clear_small(i, y1,4); return true},
                    3 => {self.h_clear_small(i, y1,3); return true},
                    _other => (),
                }

                let (v, x1) = self.v_check_match(i, j);
                match v{
                    5 => {self.v_clear_5(x1, j); return true},
                    4 => {self.v_clear_small(x1, j, 4); return true},
                    3 => {self.v_clear_small(x1, j, 3); return true},
                    _other => (),
                }
            }
        }
        return false
    }

    //for filling in empty spaces, returns true if a place was swapped, returns false when all entries are viable pieces
    pub fn swap_and_fill(&mut self) -> bool{
        let mut rng = thread_rng();
        let mut swapped = false;

        for i in 0..self.rows{

            for j in 0..self.cols{

                let value = self.board[i][j];
                if value == -1 && i != 0{//swaps non Row 0, -1 pieces with non -1
                    
                    let mut stop_row_bool = (i as i32 - 1) >= 0;
                    let mut swap_row = (i as i32 - 1) as usize;
                    

                    //while we haven't found a non -1 piece to swap with
                    while !stop_row_bool{

                        if self.board[swap_row][j] != -1{   // found a non -1 to swap with
                            stop_row_bool = false;

                        }else{                              // need to go up another row to find a non -1 to swap with
                            stop_row_bool = (swap_row as i32 - 1) >= 0;
                            if stop_row_bool{
                                swap_row = (swap_row as i32 - 1) as usize;
                            }
                        }
                        
                    }

                    //swap the pieces
                    self.board[i][j] = self.board[swap_row][j];
                    self.board[swap_row][j] = -1;

                    swapped = true;
                }else if value == -1 && i == 0{//fills in Row 0, -1 pieces
                    self.board[i][j] = rng.gen_range(0..self.pieces);
                }
            }
        }

        return swapped;
    }
}

fn test_swap(){
    let mut board = Board{ ..Default::default()};

    print!("Test which swap? 1:horizontal, 2:vertical, 3:special, 4:all || answer:  ");
    std::io::stdout().flush().unwrap();
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).ok().expect("failed to read line");
    let answer = line.trim();
    let a = answer.parse::<i32>().expect("Could not parse answer number");

    if a == 1 || a == 4{
        //testing horizontal swaps
        println!("Testing: Swap/match 3 horizontal");
        let debug_h3 = vec![
            vec![0,0,1,0],
        ];
        board.set_board(debug_h3); board.set_rows(1); board.set_cols(4);
        board.read_board();
        board.ask_swap();
        println!();

        println!("Testing: Swap/match 4 horizontal");
        let debug_h4 = vec![
            vec![0,0,1,0],
            vec![1,1,0,1],
        ];
        board.set_board(debug_h4); board.set_rows(2); board.set_cols(4);
        board.read_board();
        board.ask_swap();
        println!();

        println!("Testing: Swap/match 5 horizontal");
        let debug_h5 = vec![
            vec![0,0,1,0,0],
            vec![1,1,0,1,1],
        ];
        board.set_board(debug_h5); board.set_rows(2); board.set_cols(5);
        board.read_board();
        board.ask_swap();
        println!();
    }
    
    if a == 2 || a == 4{
        //testing vertical swaps
        println!("Testing: Swap/match 3 vertical");
        let debug_v3 = vec![
            vec![0],
            vec![0],
            vec![1],
            vec![0],
        ];
        board.set_board(debug_v3); board.set_rows(4); board.set_cols(1);
        board.read_board();
        board.ask_swap();
        println!();

        println!("Testing: Swap/match 4 vertical");
        let debug_v4 = vec![
            vec![0,1],
            vec![0,1],
            vec![1,0],
            vec![0,1],
        ];
        board.set_board(debug_v4); board.set_rows(4); board.set_cols(2);
        board.read_board();
        board.ask_swap();
        println!();

        println!("Testing: Swap/match 3 vertical");
        let debug_v5 = vec![
            vec![0,1],
            vec![0,1],
            vec![1,0],
            vec![0,1],
            vec![0,1],
        ];
        board.set_board(debug_v5); board.set_rows(5); board.set_cols(2);
        board.read_board();
        board.ask_swap();
        println!();

    }

    if a == 3 || a == 5{
        //testing special matching
        println!("Testing: special T match");
        let debug_st = vec![
            vec![3,0,3],
            vec![0,1,0],
            vec![2,0,2],
            vec![2,0,2],
        ];
        board.set_board(debug_st); board.set_rows(4); board.set_cols(3);
        board.read_board();
        board.ask_swap();
        println!();

        println!("Testing: special L match");
        let debug_sl = vec![
            vec![2,0,2,3],
            vec![2,0,2,3],
            vec![0,1,0,0],
        ];
        board.set_board(debug_sl); board.set_rows(3); board.set_cols(4);
        board.read_board();
        board.ask_swap();
        println!();
    }

}


fn main(){
    let mut answer = String::new();
    println!("Debug or Run?: ");
    std::io::stdin().read_line(&mut answer).expect("Could not read mode input");
    if answer.trim().to_lowercase().eq("debug") { //io reader includes a newline character, so trim is needed
        
        let mut function_test = String::new();
        println!("Debug what function/process?: swap, clear");
        std::io::stdin().read_line(&mut function_test).expect("Could not read function input");

        if function_test.trim().to_lowercase().eq("swap"){
            test_swap();

        }else if function_test.trim().to_lowercase().eq("clear"){
            let debug_c = vec![
                vec![1,0,0,0,3],
                vec![1,2,3,4,5],
                vec![1,1,2,3,5],
                vec![4,2,3,4,5],
                vec![0,0,0,0,0],
            ];

            let mut board = Board { rows:5, cols:5, pieces:6, board:debug_c };
            board.read_board();
            while board.check_all_match(){
                board.check_all_match();
            }
            board.ask_swap();
        }

    }else if answer.trim().to_lowercase().eq("run"){
        let mut board = Board { ..Default::default() };
        board.create_board(); board.read_board();

        while board.check_all_match(){
            board.check_all_match();
        }

        board.ask_swap();

        print!("Keep Playing? 'y'/1: yes, anything else: no/exit || answer: ");
        std::io::stdout().flush().unwrap();
        let mut play = String::new();
        std::io::stdin().read_line(&mut play).ok().expect("failed to read line");
        let mut answer= play.trim();

        while answer.to_lowercase().eq("yes") || answer.to_lowercase().eq("y") || answer.eq("1"){
            board.ask_swap();

            print!("Keep Playing? 'y'/1: yes, anything else: no/exit || answer: ");
            std::io::stdout().flush().unwrap();
            play = String::new();
            std::io::stdin().read_line(&mut play).ok().expect("failed to read line");
            answer= play.trim();
        }

        

        println!("Thank you for playing!");
    }else{
        println!("Not an available mode: {}", &answer);
    }
}
