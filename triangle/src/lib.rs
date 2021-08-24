use std::ops::Add;

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct Triangle<T> {
    side_one: T,
    side_two: T,
    side_three: T,
}

impl<T: Add<Output = T> + PartialEq + PartialOrd + Copy + Clone> Triangle<T> {
    pub fn build(sides: [T; 3]) -> Option<Triangle<T>> {        
        if sides[0] + sides[1] == sides[0] || sides[0] + sides[1] == sides[1] || sides[1] + sides[2] == sides[1] || sides[1] + sides[2] == sides[2] || sides[0] + sides[2] == sides[0] || sides[0] + sides[2] == sides[2] {
             return None;
        }

        if sides[0] + sides[1] < sides[2] || sides[1] + sides[2] < sides[0] || sides[0] + sides[2] < sides[1]{
             return None;
        }

        Some(Triangle{ side_one: sides[0],
                       side_two: sides[1],
                       side_three: sides[2]})
    }

    pub fn is_equilateral(&self) -> bool {
        if self.side_one == self.side_two && self.side_one == self.side_three {
	    return true;
        }       
        false
    }

    pub fn is_scalene(&self) -> bool {
        if self.side_one != self.side_two && self.side_one != self.side_three && self.side_two != self.side_three {
	    return true;
        }
       
        false
    }

    pub fn is_isosceles(&self) -> bool {
        if self.side_one == self.side_two || self.side_one == self.side_three || self.side_two == self.side_three {
             return true;
        }
        false
    }
}
