use crate::vector::Vector;

#[derive(Clone, Copy)]
pub struct Ray
{
    pub origin : Vector,
    pub dest : Vector
}

impl Ray
{
    pub fn new() -> Self
    {
        Ray
        {
            origin : Vector::new(),
            dest : Vector::new()
        }
    }

    pub fn set_origin(&mut self ,x : f64, y : f64, z : f64) -> &mut Self
    {
        self.origin.set_vector(x, y, z);

        self
    }

    pub fn set_dest(&mut self ,x : f64, y : f64, z : f64) -> &mut Self
    {
        // Normalize destination vector to avoid errors in calculations
        self.dest.set_vector(x, y, z).normalize();

        self
    }
}
