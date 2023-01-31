pub struct FloodFill {
    q: Vec<(usize, usize)>,
    mat: Vec<Vec<usize>>,
    grid_size: usize,
    wall: usize,
    no_wall: usize,
    weight: usize
}

impl FloodFill {
    pub fn new(mat: &Vec<Vec<usize>>, wall: usize, no_wall: usize) -> Self {
        FloodFill {
            q: Vec::new(),
            mat: mat.clone(),
            grid_size: mat.first().expect("Mat empty").len(),
            wall,
            no_wall,
            weight: 0
        }
    }

    fn update_q(&mut self, (x, y): (i32, i32)) {
        if x < 0 || y < 0 {
            return;
        }

        let x = x as usize;
        let y = y as usize;
        if x >= self.grid_size || y >= self.grid_size {
            return;
        }

        // Check if the position is fill-able
        if self.mat[x][y] != self.no_wall || self.q.contains(&(x, y)) {
            return;
        }

        self.q.push((x, y))
    }

    fn process(&mut self) {
        loop {
            if self.q.is_empty() {
                break;
            }

            let (x, y) = self.q.remove(0);

            self.weight += 1;
            self.mat[x][y] = self.weight;

            // Four way flood fill
            self.update_q((x as i32 + 1, y as i32));
            self.update_q((x as i32 - 1, y as i32));
            self.update_q((x as i32, y as i32 + 1));
            self.update_q((x as i32, y as i32 - 1));
        }
    }

    pub fn solve(&mut self, (x, y): (usize, usize)) -> Vec<Vec<usize>> {
        self.q.push((x, y));
        self.process();

        self.mat.clone()
    }
}

