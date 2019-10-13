use std::io::BufRead;

#[derive(Debug)]
pub struct Grid {
    vec: Vec<Vec<i32>>,
}

impl Grid {
    pub fn from<R: BufRead>(reader: R) -> Self {
        let mut grid: Vec<Vec<i32>> = Vec::with_capacity(40);
        for line in reader.lines() {
            let line = line.unwrap();
            grid.push(
                line.chars()
                    .chain(std::iter::repeat(' '))
                    .take(80)
                    .map(|c| c as i32)
                    .collect(),
            );
        }
        Grid { vec: grid }
    }
}

impl std::ops::Index<crate::vm::Coord> for Grid {
    type Output = i32;
    fn index(&self, i: crate::vm::Coord) -> &Self::Output {
        &self.vec[i.y][i.x]
    }
}

impl std::ops::IndexMut<crate::vm::Coord> for Grid {
    fn index_mut(&mut self, i: crate::vm::Coord) -> &mut Self::Output {
        &mut self.vec[i.y][i.x]
    }
}
