use std::{
    convert::{self, TryFrom},
    fmt, ops, ptr, result,
};

#[derive(Debug)]
pub struct Grid<T> {
    buffer: Vec<T>,
    width: usize,
    height: usize,
}

impl<T> Grid<T> {
    fn max_across(&self) -> usize {
        self.width - 1
    }
    fn max_down(&self) -> usize {
        self.height - 1
    }
    pub fn width(&self) -> usize {
        self.width
    }
    pub fn height(&self) -> usize {
        self.height
    }
}

impl<T: fmt::Debug> fmt::Display for Grid<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> result::Result<(), fmt::Error> {
        for down in 0..self.max_across() {
            for across in 0..self.max_down() {
                let location = Location {
                    grid: &self,
                    across,
                    down,
                };
                println!("{:?}", location);
                write!(f, "{:?}", self[location])?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl<T> ops::Index<Index<'_, T>> for Grid<T> {
    type Output = T;
    fn index(&self, index: Index<T>) -> &Self::Output {
        assert!(ptr::eq(self, index.grid), "Index is not for this Grid!");
        &self.buffer[index.value]
    }
}

impl<T> ops::Index<Location<'_, T>> for Grid<T> {
    type Output = T;
    fn index(&self, location: Location<T>) -> &Self::Output {
        match Index::try_from(location) {
            Err(_) => panic!("Invalid Location for Grid!"),
            Ok(i) => &self.buffer[i.value],
        }
    }
}

#[derive(Debug)]
pub struct Location<'g, T> {
    grid: &'g Grid<T>,
    pub across: usize,
    pub down: usize,
}

#[derive(Debug)]
struct Index<'g, T> {
    grid: &'g Grid<T>,
    value: usize,
}

impl<'g, T> convert::From<Index<'g, T>> for Location<'g, T> {
    fn from(index: Index<'g, T>) -> Self {
        Self {
            grid: index.grid,
            across: index.value % index.grid.width,
            down: index.value % index.grid.height,
        }
    }
}

#[derive(Debug)]
struct InvalidLocation<'g, T> {
    location: Location<'g, T>,
    grid: &'g Grid<T>,
}

impl<'g, T> convert::TryFrom<Location<'g, T>> for Index<'g, T> {
    type Error = InvalidLocation<'g, T>;
    fn try_from(location: Location<'g, T>) -> result::Result<Self, Self::Error> {
        let grid = location.grid;
        if location.across > grid.max_across() || location.down > grid.max_down() {
            return Err(InvalidLocation { grid, location });
        } else {
            return Ok(Self {
                grid,
                value: location.down * grid.max_across() + location.across,
            });
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn g1() -> Grid<usize> {
        Grid {
            buffer: vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11],
            width: 3,
            height: 4,
        }
    }
    fn g2() -> Grid<usize> {
        Grid {
            buffer: vec![0, 1, 2, 3, 4, 5, 6, 7, 8],
            width: 3,
            height: 3,
        }
    }
    #[test]
    fn can_index_wit_index() {
        let g = g1();
        let i = Index { grid: &g, value: 0 };
        assert_eq!(0, g[i]);
    }
    #[test]
    #[should_panic = "Index is not for this Grid!"]
    fn cannot_index_other_grid() {
        let (g1, g2) = (g1(), g2());

        // Create an index into g1
        let i = Index {
            grid: &g1,
            value: 0,
        };

        // Try and use it for g2
        assert_eq!(0, g2[i]);
    }
    #[test]
    fn max_location_max_height() {
        let g1 = g1();
        assert_eq!(g1.max_across(), 2);
        assert_eq!(g1.max_down(), 3)
    }
    #[test]
    fn test_display_grid() {
        let g1 = g1();
        println!("{}", g1);
    }
}
