use std::ops::{Add, Sub};
use std::fmt;

///Builder for points that saves me from having to define the bounds for every point.
pub struct PointBuilder(usize, usize);

impl PointBuilder {
    ///Creates a new PointBuilder with bounds x and y for bounds.
    pub fn new(x: usize, y: usize) -> Self {
        PointBuilder(x, y)
    }

    ///Builds a new point at the coordinates x and y with bounds set when PointBuilder is defined.
    pub fn build(&self, x: usize, y: usize) -> Point {
        Point(x, y, self.0, self.1)
    }
}

///Coordinate struct where the first two fields are x and y, and the second two fields are x_bound and y_bound.
#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash)]
pub struct Point(usize, usize, usize, usize);

impl Point {
    ///Creates a new PointBuilder with bounds x and y.
    pub fn builder(x: usize, y: usize) -> PointBuilder {
        PointBuilder::new(x, y)
    }

    ///Performs an addition operation and returns the result as an option. Automatically performs a bounds check based on the bounds given,   
    ///and returns None if x or y is outside the bounds. The bounds of the second point don't matter, and can safely be ignored.
    pub fn checked_add(self, other: Self) -> Option<Self> {
        self.0.checked_add(other.0).zip(self.1.checked_add(other.1)).map(|result|{
            let (x, y) = result;
            Self(x, y, self.2, self.3)
        }).filter(|&result| {
            let Point(x, y, _, _) = result;
            x < self.2 && y < self.3
        })
    }

    ///Performs a subtraction operation and returns the result as an option. Returns `None` if either x or y would go below zero.
    pub fn checked_sub(self, other: Self) -> Option<Self> {
        self.0.checked_sub(other.0).zip(self.1.checked_sub(other.1)).map(|result|{
            let (x, y) = result;
            Self(x, y, self.2, self.3)
        })
    }

    ///Returns the x and y coordinates as a tuple.
    pub fn get(self) -> (usize, usize) {
        (self.0, self.1)
    }

    ///Returns a vector of the points in each cardinal direction. Panics if no direction is in bounds.
    pub fn check_neighbors(self) -> Vec<Point> {
        let mut rtn = Vec::with_capacity(4);
        if let Some(result) = self.north() {rtn.push(result);}
        if let Some(result) = self.south() {rtn.push(result);}
        if let Some(result) = self.east() {rtn.push(result);}
        if let Some(result) = self.west() {rtn.push(result);}
        if rtn.is_empty() {panic!("Point {{{}, {}}} has no neigbors in bounds: {{{}, {}}}.", self.0, self.1, self.2, self.3)}
        rtn
    }

    ///Returns the point to the north of `self`. Returns `None` if its out of bounds.
    pub fn north(self) -> Option<Self> {
        self.checked_add(Point(0, 1, 0, 0))
    }

    ///Returns the point to the south of `self`. Returns `None` if its out of bounds.
    pub fn south(self) -> Option<Self> {
        self.checked_sub(Point(0, 1, 0, 0))
    }

    ///Returns the point to the east of `self`. Returns `None` if its out of bounds.
    pub fn east(self) -> Option<Self> {
        self.checked_add(Point(1, 0, 0, 0))
    }

    ///Returns the point to the west of `self`. Returns `None` if its out of bounds.
    pub fn west(self) -> Option<Self> {
        self.checked_sub(Point(1, 0, 0, 0))
    }
}

impl Add for Point {
    type Output = Self;
   
    fn add(self, other: Self) -> Self {
        Self(self.0 + other.0, self.1 + other.1, self.2, self.3)
    }
}

impl Sub for Point {
    type Output = Self;
   
    fn sub(self, other: Self) -> Self {
        Self(self.0 - other.0, self.1 - other.1, self.2, self.3)
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}, {}", self.0, self.1)
    }
}

impl Default for Point {
    fn default() -> Self {
        Point(0, 0, 0, 0)
    }
}