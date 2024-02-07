use crate::camera::Camera;
use crate::object::*;  
use crate::ray::Ray;

pub struct Scene
{
    pub objects : Vec<Object>,
    pub camera : Camera
}

pub struct Intersection<'a>
{
    pub object : &'a Object,
    pub distance : f64,
    pub found : bool
}

impl<'a> Intersection<'a>
{
    pub fn new(obj : &'a Object, dist : f64, _found : bool) -> Self
    {
        Intersection
        {
            object: obj,
            distance: dist,
            found: _found
        }
    }
}

impl Scene
{
    pub fn add(&mut self, obj : Object)
    {
        self.objects.push(obj);
    }

    //
    pub fn intersect(&self, ray : Ray) -> Intersection
    {
        let mut closest_intersect = Intersection::new(&self.objects[0], f64::INFINITY, false);
        for object in self.objects.iter()
        {
            let distance = object.body.intersect(ray);

            if distance > f64::EPSILON && distance < closest_intersect.distance
            {
                closest_intersect.distance = distance;
                closest_intersect.object = object;
                closest_intersect.found = true;
            }
        }

        return closest_intersect;
    }
}