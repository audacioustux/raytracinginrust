use indicatif::ParallelProgressIterator;
use rand::{rngs::SmallRng, Rng, SeedableRng};
use rayon::iter::{IndexedParallelIterator, IntoParallelIterator, ParallelIterator};
use raytracinginrust::{
    camera::Camera,
    hittable::{Hittable, List, Sphere},
    material::{Dielectric, Lambertian, Metal},
    ray::Ray,
    vec3::{Color, Position, Vec3f},
};

fn color<T: Hittable>(ray: Ray, world: &List<T>, depth: i32) -> Vec3f<Color> {
    if let Some(record) = world.hit(ray, 0.001, f32::MAX) {
        return if depth >= 50 {
            Vec3f::repeat(0.0)
        } else if let Some((attenuation, scattered)) = record.material.scatter(ray, record) {
            attenuation * color(scattered, world, depth + 1)
        } else {
            Vec3f::repeat(0.0)
        };
    }

    let direction = ray.direction().unit();
    let t = (direction.y() + 1.0) * 0.5;
    (1.0 - t) * Vec3f::repeat(1.0) + t * Vec3f::new(0.5, 0.7, 1.0)
}

fn random_scene() -> List<Sphere> {
    let mut rng = SmallRng::from_entropy();
    let mut list = List {
        list: vec![Sphere {
            center: (0.0, -1000.0, 0.0).into(),
            radius: 1000.0,
            material: Lambertian::boxed((0.5, 0.5, 0.5).into()),
        }],
    };
    for a in -11..11 {
        for b in -11..11 {
            let choose_mat: f32 = rng.gen();
            let center = Vec3f::<Position>::new(
                a as f32 + 0.9 + rng.gen::<f32>(),
                0.2,
                b as f32 + 0.9 + rng.gen::<f32>(),
            );
            if (center - Vec3f::new(4.0, 0.2, 0.0)).magnitude() > 0.9 {
                if choose_mat < 0.8 {
                    // diffuse
                    list.push(Sphere {
                        center,
                        radius: 0.2,
                        material: Lambertian::boxed(
                            (
                                rng.gen::<f32>() * rng.gen::<f32>(),
                                rng.gen::<f32>() * rng.gen::<f32>(),
                                rng.gen::<f32>() * rng.gen::<f32>(),
                            )
                                .into(),
                        ),
                    });
                } else if choose_mat < 0.95 {
                    // metal
                    list.push(Sphere {
                        center,
                        radius: 0.2,
                        material: Metal::boxed(
                            (
                                0.5 * (1.0 - rng.gen::<f32>()),
                                0.5 * (1.0 - rng.gen::<f32>()),
                                0.5 * (1.0 - rng.gen::<f32>()),
                            )
                                .into(),
                            0.5 * rng.gen::<f32>(),
                        ),
                    });
                } else {
                    // glass
                    list.push(Sphere {
                        center,
                        radius: 0.2,
                        material: Dielectric::boxed(1.5),
                    })
                }
            }
        }
    }
    list.push(Sphere {
        center: (0.0, 1.0, 0.0).into(),
        radius: 1.0,
        material: Dielectric::boxed(1.5),
    });
    list.push(Sphere {
        center: (-4.0, 1.0, 0.0).into(),
        radius: 1.0,
        material: Lambertian::boxed((0.4, 0.2, 0.1).into()),
    });
    list.push(Sphere {
        center: (4.0, 1.0, 0.0).into(),
        radius: 1.0,
        material: Metal::boxed((0.7, 0.6, 0.5).into(), 0.0),
    });

    list
}

fn print_result(nx: usize, ny: usize, ns: usize) {
    let lookfrom = (13.0, 2.0, 3.0).into();
    let lookat = (0.0, 0.0, 0.0).into();
    let camera = Camera::new(
        lookfrom,
        lookat,
        (0.0, 1.0, 0.0).into(),
        20.0,
        nx as f32 / ny as f32,
        0.1,
        10.0,
    );
    let world = random_scene();
    let image: Vec<lodepng::RGBA> = (0..ny)
        .into_par_iter()
        .rev()
        .progress_count(ny as u64)
        .flat_map(|j| {
            (0..nx)
                .into_par_iter()
                .map(|i| {
                    let col = (0..ns)
                        .into_par_iter()
                        .fold(
                            || Vec3f::<Color>::repeat(0.0),
                            |acc, _| {
                                let mut rng = SmallRng::from_entropy();
                                let u = (i as f32 + rng.gen::<f32>()) / nx as f32;
                                let v = (j as f32 + rng.gen::<f32>()) / ny as f32;
                                let ray = camera.ray(u, v);
                                acc + color(ray, &world, 0)
                            },
                        )
                        .sum::<Vec3f<Color>>()
                        / ns as f32;
                    let col = col.map(|x| x.sqrt() * 255.99);
                    lodepng::RGBA {
                        r: col.x() as u8,
                        g: col.y() as u8,
                        b: col.z() as u8,
                        a: 255,
                    }
                })
                .collect::<Vec<_>>()
        })
        .collect();
    lodepng::encode32_file("output.png", &image, nx, ny).unwrap();
}

fn main() {
    let instant = std::time::Instant::now();
    print_result(1920, 1080, 100);
    println!("{:?}", instant.elapsed())
}
