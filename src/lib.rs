
#[derive(Debug, PartialEq)]
pub enum Type {
    Chamfer,
    Radius,
    Arc,
}

#[derive(Debug, PartialEq)]
pub struct Params {
    pub calc_type: Type,
    pub start_x: f64,
    pub start_z: f64,
    pub end_x: f64,
    pub end_z: f64,
    pub angle: f64,
    pub insert_radius: f64,
}

impl Params {
    pub fn new(args: &[String]) -> Result<Params, &'static str> {

        let calc_type = match args[1].to_lowercase().as_str() {
            "cha" => Type::Chamfer,
            "rad" => Type::Radius,
            "arc" => return Err("дуги не поддерживаются"),
            _ => return Err("указан некорректный тип расчета")
        };
    
        let mut op_start_x: Option<f64> = None;
        let mut op_start_z: Option<f64> = None;
        let mut op_end_x: Option<f64> = None;
        let mut op_end_z: Option<f64> = None;
        let mut op_angle: Option<f64> = None;
        let mut insert_radius: f64 = 0_f64;

        for arg in args.iter().skip(2) {
            if arg.to_lowercase().starts_with("sx") {
                op_start_x = match arg.trim_start_matches("sx").parse() {
                    Ok(x) => {
                        //println!("Начальная точка Х: {x}");
                        Some(x)
                    },
                    Err(_) => return Err("некорректно задан начальный X (агумент sx_)"),
                }
            } else if arg.to_lowercase().starts_with("sz") {
                op_start_z = match arg.trim_start_matches("sz").parse() {
                    Ok(x) => {
                        //println!("Начальная точка Z: {x}");
                        Some(x)
                    },
                    Err(_) => return Err("некорректно задан начальный Z (агумент sz_)")
                };
            } else if arg.to_lowercase().starts_with("ex") {
                op_end_x = match arg.trim_start_matches("ex").parse() {
                    Ok(x) => {
                        //println!("Конечная точка X: {x}");
                        Some(x)
                    },
                    Err(_) => return Err("некорректно задан конечный X (агумент ex_)")
                };
            } else if arg.to_lowercase().starts_with("ez") {
                op_end_z = match arg.trim_start_matches("ez").parse() {
                    Ok(x) => {
                        //println!("Конечная точка Z: {x}");
                        Some(x)
                    },
                    Err(_) => return Err("некорректно задан конечный Z (агумент ez_)")
                };
            } else if arg.to_lowercase().starts_with("a") {
                op_angle = match arg.trim_start_matches("a").parse() {
                    Ok(x) => {
                        //println!("Угол: {x}");
                        Some(x)
                    },
                    Err(_) => return Err("некорректно задан угол (агумент a_)")
                };
            } else if arg.to_lowercase().starts_with("tr") {
                insert_radius = match arg.trim_start_matches("tr").parse() {
                    Ok(x) => x,
                    Err(_) => return Err("некорректно задан радиус инструмента (агумент tr_)")
                };
            }
        }
        
        let start_x = match op_start_x {
            Some(x) => x,
            None => match (op_start_z, op_end_x, op_end_z, op_angle) {
                (Some(start_z), Some(end_x), Some(end_z), Some(angle)) => {
                    //print!("Расчет начальной точки Х");
                    end_x - (end_z - start_z).abs() * 2_f64 * angle.to_radians().tan()
                },
                _ => return Err("Не удалось расчитать начальную точку Х, т.к. указано недостаточно аргументов"),
            },
        };

        let start_z = match op_start_z {
            Some(x) => x,
            None => match (op_start_x, op_end_x, op_end_z, op_angle) {
                (Some(start_x), Some(end_x), Some(end_z), Some(angle)) => {
                    //println!("Расчет начальной точки Z");
                    end_z - (end_x - start_x).abs() / 2_f64 * angle.to_radians().tan()
                },
                _ => return Err("Не удалось расчитать начальную точку Z, т.к. указано недостаточно аргументов"),
            },
        };

        let end_x = match op_end_x {
            Some(x) => x,
            None => match (op_start_x, op_start_z, op_end_z, op_angle) {
                (Some(start_x), Some(start_z), Some(end_z), Some(angle)) => {
                    //println!("Расчет конечной точки X");
                    start_x + (end_z - start_z).abs() * 2_f64 * angle.to_radians().tan()
                },
                _ => return Err("Не удалось расчитать конечную точку Х, т.к. указано недостаточно аргументов"),
            },
        };

        let end_z = match op_end_z {
            Some(x) => x,
            None => match (op_start_x, op_start_z, op_end_x, op_angle) {
                (Some(start_x), Some(start_z), Some(end_x), Some(angle)) => {
                    //println!("Расчет конечной точки Z");
                    start_z + (end_x - start_x).abs() / 2_f64 * angle.to_radians().tan()
                },
                _ => return Err("Не удалось расчитать конечную точку Z, т.к. указано недостаточно аргументов"),
            },
        };

        let angle = match calc_type {
            Type::Chamfer => match op_angle {
                Some(x) => x,
                None => match (op_start_x, op_start_z, op_end_x, op_end_z) {
                    (Some(start_x), Some(start_z), Some(end_x), Some(end_z)) => {
                        //println!("Расчет угла");
                        ((start_x - end_x) / 2_f64).abs().atan2((start_z - end_z).abs()).to_degrees()
                    },
                    _ => return Err("Не удалось расчитать угол, т.к. указано недостаточно аргументов"),
                },
            }
            _ => 45_f64,
        };

        Ok(Params {calc_type, start_x, start_z, end_x, end_z, angle, insert_radius})
    }
}