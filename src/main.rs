use std::io;

const C: f32 = 32.0;

fn c_to_f(celsius_temp: f32) -> f32 {
    (celsius_temp * (9.0 / 5.0)) + C
}

fn f_to_c(fahrenheit_temp: f32) -> f32 {
    (fahrenheit_temp - C) * (5.0 / 9.0)
}

fn convert(temperature: f32, choice: u8) -> Option<f32> {
    match choice {
        1 => Some(c_to_f(temperature)),
        2 => Some(f_to_c(temperature)),
        _ => None,
    }
}

fn main() {
    println!("Конвертация температуры. \n (1) C в F \n (2) F в C");

    let mut user_choice = String::new();
    io::stdin().read_line(&mut user_choice).unwrap();

    let number_choice: u8 = match user_choice.trim().parse::<u8>() {
        Ok(num) => num,
        Err(_) => {
            println!("Ошибка: Введите корректное число (1 или 2).");
            return;
        }
    };

    println!("Введите температуру:");

    let mut temperature_input = String::new();
    io::stdin().read_line(&mut temperature_input).unwrap();

    let temperature: f32 = match temperature_input.trim().parse::<f32>() {
        Ok(temp) => temp,
        Err(_) => {
            println!("Ошибка: Введите корректное значение температуры.");
            return;
        }
    };

    match convert(temperature, number_choice) {
        Some(result) => println!("Результат конвертации: {result}"),
        None => println!("Увы, неверный выбор. Пожалуйста, выберите 1 или 2."),
    };
}