use std::{fs::OpenOptions, io::Read, path::Path};

use rhai::Engine;

use super::{duplicate_triangle, light_scale, rotate_prop_static, texture_scale};

fn rotate_prop_static_single(map: &mut map::Map) {
    rotate_prop_static::rotate_prop_static(map, None);
}

fn rotate_prop_static(map: &mut map::Map, new: &str) {
    rotate_prop_static::rotate_prop_static(map, Some(new));
}

fn light_scale(map: &mut map::Map, r: f64, g: f64, b: f64, brightness: f64) {
    light_scale::light_scale(map, (r, g, b, brightness))
}

fn light_scale_int(map: &mut map::Map, r: i64, g: i64, b: i64, brightness: i64) {
    light_scale::light_scale(map, (r as f64, g as f64, b as f64, brightness as f64))
}

fn light_scale_brightness(map: &mut map::Map, brightness: f64) {
    light_scale(map, 1., 1., 1., brightness);
}

fn light_scale_brightness_int(map: &mut map::Map, brightness: i64) {
    light_scale(map, 1., 1., 1., brightness as f64);
}

fn texture_scale(map: &mut map::Map, scalar: f64) {
    texture_scale::texture_scale(map, scalar);
}

fn texture_scale_int(map: &mut map::Map, scalar: i64) {
    texture_scale(map, scalar as f64);
}

// TODO propagate results
pub fn custom_script(rhai_file: &Path) {
    // Rhai engine part
    let mut engine = Engine::new();

    engine
        // light_scale
        .register_fn("light_scale", light_scale)
        .register_fn("light_scale", light_scale_int)
        .register_fn("light_scale", light_scale_brightness)
        .register_fn("light_scale", light_scale_brightness_int)
        // rotate_prop_static
        .register_fn("rotate_prop_static", rotate_prop_static)
        .register_fn("rotate_prop_static", rotate_prop_static_single)
        // texture_scale
        .register_fn("texture_scale", texture_scale)
        .register_fn("texture_scale", texture_scale_int)
        // duplicate_triangle
        .register_fn("duplicate_triangle", duplicate_triangle::duplicate_triangle)
        .register_fn(
            "mass_duplicate_triangle",
            duplicate_triangle::mass_duplicate_triangle,
        );

    // For write functions. Need to ignore Result.
    engine
        .register_type_with_name::<map::Map>("Map")
        .register_fn("new_map", |file_name: String| {
            map::Map::from_file(file_name).unwrap()
        })
        .register_fn("write", |map, out: String| {
            let _ = map::Map::write(map, out);
        })
        .register_fn("light_scale", light_scale::light_scale)
        .register_fn("sexture_scale", texture_scale::texture_scale)
        .register_fn("rotate_prop_static", rotate_prop_static::rotate_prop_static);

    engine
        .register_type_with_name::<qc::Qc>("Qc")
        .register_fn("new_qc", |file_name: String| {
            qc::Qc::from_file(file_name).unwrap()
        })
        .register_fn("write", |qc, out: String| {
            let _ = qc::Qc::write(qc, out);
        });

    engine
        .register_type_with_name::<smd::Smd>("Smd")
        .register_fn("new_smd", |file_name: String| {
            smd::Smd::from_file(file_name).unwrap()
        })
        .register_fn("write", |smd, out: String| {
            let _ = smd::Smd::write(smd, out);
        })
        .register_fn("duplicate_triangle", duplicate_triangle::duplicate_triangle);

    let file = OpenOptions::new().read(true).open(rhai_file);

    if let Err(err) = file {
        println!("Cannot open file. {}", err);
        return;
    }

    let mut script = String::new();

    if let Err(err) = file.unwrap().read_to_string(&mut script) {
        println!("Cannot read file. {}", err);
        return;
    }

    if let Err(err) = engine.run(&script) {
        println!("Problem with running the script. {}", err);
    };
}
