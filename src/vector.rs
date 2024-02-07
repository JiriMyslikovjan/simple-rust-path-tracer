#[derive(Clone, Copy)]
pub struct Vector
{
    pub x : f64,
    pub y : f64,
    pub z : f64
}

impl Vector
{
    pub fn new() -> Self
    {
        Vector
        {
            x : 0.0,
            y : 0.0,
            z : 0.0
        }
    }

    pub fn set_vector(&mut self, x : f64, y : f64, z : f64) -> &mut Self
    {
        self.x = x;
        self.y = y;
        self.z = z;

        self
    }

    pub fn mul_by_vec(&self, vec2 : Vector) -> Vector
    {
        Vector
        {
            x : self.x * vec2.x,
            y : self.y * vec2.y,
            z : self.z * vec2.z
        }
    }

    pub fn normalize(&mut self) -> &mut Self
    {
        let vec_len : f64 = (self.x * self.x + self.y * self.y + self.z * self.z).sqrt();

        self.x = self.x * (1.0 / vec_len);
        self.y = self.y * (1.0 / vec_len);
        self.z = self.z * (1.0 / vec_len);

        self
    }

    pub fn dot_product(&self, vec2 : Vector) -> f64
    {
        self.x * vec2.x + self.y * vec2.y + self.z * vec2.z
    }

    pub fn cross_product(&self, vec2 : Vector) -> Vector
    {
        Vector
        {
            x : self.y * vec2.z - self.z * vec2.y,
            y : self.z * vec2.x - self.x * vec2.z,
            z : self.x * vec2.y - self.y * vec2.x
        }
    }
}

impl std::ops::Add for Vector
{
    type Output = Vector;

    fn add(self, vec2: Vector) -> Vector
    {
        Vector
        {
            x : self.x + vec2.x,
            y : self.y + vec2.y,
            z : self.z + vec2.z
        }
    }
}

impl std::ops::Sub for Vector
{
    type Output = Vector;

    fn sub(self, vec2: Vector) -> Vector
    {
        Vector
        {
            x : self.x - vec2.x,
            y : self.y - vec2.y,
            z : self.z - vec2.z
        }
    }
}

impl  std::ops::Mul<f64> for Vector
{
    type Output = Vector;

    fn mul(self, constant : f64) -> Vector
    {
        Vector
        {
            x : self.x * constant,
            y : self.y * constant,
            z : self.z * constant
        }
    }
}

pub fn orthonormal_sys(vec1 : Vector, mut vec2 : &mut Vector, vec3 : &mut Vector)
{
    if vec1.x.abs() > vec1.y.abs()
    {
        let inv_len = 1.0 / (vec1.x * vec1.x + vec1.z * vec1.z).sqrt();

        vec2 = vec2.set_vector(-vec1.z * inv_len, 0.0, vec1.x * inv_len);
    }
    else
    {
        let inv_len = 1.0 / (vec1.y * vec1.y + vec1.z * vec1.z).sqrt();

       vec2.set_vector(0.0, vec1.z * inv_len, -vec1.y * inv_len);
    }

   * vec3 = vec1.cross_product(*vec2);
}