mod calc;
use std::{env, process, error::Error};
use scalc::{Params, Type};
use crate::calc::{chamfer_shift_x, chamfer_shift_z};

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let params = Params::new(&args).unwrap_or_else(|err| {
        println!("Возникла ошибка: {err}");
        process::exit(1);
    });

    match params.calc_type {
        Type::Chamfer => {
            println!("Расчет фаски: \nНачальный диаметр: {}\nНачальная глубина: {}\nКонечный диаметр: {}\nКонечная глубина: {}\nУгол: {}\nРадиус пластины: {}\n", 
            params.start_x,
            params.start_z,
            params.end_x,
            params.end_z,
            params.angle, 
            params.insert_radius);
            println!("Заход с учетом радиуса пластины: {}", params.start_x - (2_f64 * chamfer_shift_x(params.angle, params.insert_radius)));
            println!("Глубина с учетом радиуса пластины: {}", params.end_z + chamfer_shift_z(params.angle, params.insert_radius));
        },
        Type::Radius => {
            println!("Расчет фаски: \nНачальный диаметр: {}\nНачальная глубина: {}\nКонечный диаметр: {}\nКонечная глубина: {}\nРадиус пластины: {}\n", 
            params.start_x,
            params.start_z,
            params.end_x,
            params.end_z,
            params.insert_radius);
            println!("Заход с учетом радиуса пластины: {}", params.start_x - (2_f64 * params.insert_radius));
            println!("Глубина с учетом радиуса пластины: {}", params.end_z + params.insert_radius);
        },
        Type::Arc => unreachable!(),
    }

    //println!("x: {}", chamfer_shift_x(params.angle, params.insert_radius));
    //println!("z: {}", chamfer_shift_z(params.angle, params.insert_radius));
    Ok(())
}