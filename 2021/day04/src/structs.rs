#[derive(Debug)]
pub struct BoardElement {
    pub number: usize,
    pub is_marked: bool,
}

impl BoardElement {
    pub fn new(number: usize) -> Self {
        BoardElement {
            number,
            is_marked: false,
        }
    }
}

impl Clone for BoardElement {
    fn clone(&self) -> Self {
        Self {
            number: self.number.clone(),
            is_marked: self.is_marked.clone(),
        }
    }
}

#[derive(Debug)]
pub struct Board {
    pub elements: Vec<BoardElement>,
    pub has_won: bool,
}

impl Board {
    pub fn new(numbers: Vec<BoardElement>) -> Self {
        Board {
            elements: numbers,
            has_won: false,
        }
    }

    pub fn check_number(&mut self, number: usize) {
        for element in self.elements.iter_mut() {
            if element.number == number {
                element.is_marked = true;
                return;
            }
        }
    }

    pub fn check_for_win(&self) -> bool {
        'column: for i in 0..5 {
            for j in 0..5 {
                let index = i + 5 * j;
                if !self.elements[index].is_marked {
                    continue 'column;
                }
            }

            return true;
        }

        'row: for i in 0..5 {
            for j in 0..5 {
                let index = 5 * i + j;
                if !self.elements[index].is_marked {
                    continue 'row;
                }
            }

            return true;
        }

        return false;
    }

    pub fn calculate_score(&self) -> usize {
        let mut score = 0;
        for element in self.elements.iter() {
            if !element.is_marked {
                score += element.number;
            }
        }

        score
    }
}
