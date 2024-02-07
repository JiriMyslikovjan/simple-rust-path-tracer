use crate::material::Material;
use crate::ray::Ray;
use crate::vector::Vector;

const EPSILON : f64 = 0.000001;

pub struct Object
{
    pub body: Box<dyn Body + Send + Sync>
}

pub trait Body
{
    fn intersect(&self, ray : Ray) -> f64;
    fn normal(&self, vec : Vector) -> Vector;
    fn get_material(&self) -> Material;
    fn clone_self(&self) -> Box<dyn Body + Send + Sync>;
}

#[derive(Clone)]
pub struct Sphere
{
    pub centre : Vector,
    pub radius : f64,
    pub material : Material,
}

impl Sphere
{
    pub fn new(cent : Vector, rad : f64, mat : Material) -> Self
    {
        Sphere
        {
            centre : cent,
            radius : rad,
            material : mat
        }
    }

}

impl Body for Sphere
{
    fn intersect(&self, ray: Ray) -> f64
    {
        let sphere_centre = self.centre;
        let sphere_radius = self.radius;
        let ray_origin = ray.origin;
        let ray_dest = ray.dest;

        let b = ((ray_origin - sphere_centre) * 2.0).dot_product(ray_dest);
        let c = (ray_origin - sphere_centre).dot_product(ray_origin - sphere_centre) - (sphere_radius * sphere_radius);

        let mut discr = b * b - 4.0 * c;

        if discr < 0.0
        { return 0.0;}

        discr = discr.sqrt();

        let result1 = -b + discr;
        let result2 = -b - discr;

        if result2 > EPSILON
        { return result2 / 2.0; }

        else if  result1 > EPSILON
        { return result1 / 2.0; }

        return 0.0
    }

    fn normal(&self, vec : Vector) -> Vector
    {
        let sphere_centre = self.centre;

        return *(vec - sphere_centre).normalize();
    }

    fn get_material(&self) -> Material {
        return self.material;
    }

    fn clone_self(&self) -> Box<dyn Body + Send +Sync> { return Box::new(self.clone()) }
}

#[derive(Clone)]
pub struct Plane
{
    normal : Vector,
    d : f64,
    material : Material
}

impl Plane
{
    pub fn new(norm : Vector, d_ : f64, mat : Material) -> Self
    {
        Plane
        {
            normal : norm,
            d : d_,
            material : mat
        }
    }

}

impl Body for Plane {
    fn intersect(&self, ray: Ray) -> f64
    {
        let plane_normal = self.normal;
        let plane_d = self.d;
        let d0 = plane_normal.dot_product(ray.dest);

        if d0 != 0.0
        {
            let t = -1.0 * ((plane_normal.dot_product(ray.origin) + plane_d) / d0);

            if t > EPSILON
            { return t; }

            return 0.0;
        }

        return 0.0
    }

    fn normal(&self, _vec: Vector) -> Vector
    { return self.normal; }

    fn get_material(&self) -> Material {
        return self.material;
    }

    fn clone_self(&self) -> Box<dyn Body + Send + Sync> { return Box::new(self.clone()) }
}

impl Clone for Box<dyn Body>
{
    fn clone(&self) -> Self { return self.clone_self() }
}
