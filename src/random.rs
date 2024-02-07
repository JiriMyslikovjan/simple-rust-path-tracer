use crate::rand::distributions::Distribution;

pub fn gen_num() -> f64
{
    let distribution = rand::distributions::Uniform::<f64>::from(0.0..1.0);
    let mut rng = rand::thread_rng();

    return distribution.sample(& mut rng)
}
