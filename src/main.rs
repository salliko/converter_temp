use std::io;


fn main() {
    println!("Утилита конвертиции температуры из значения по Фаренгейту к Цельсию.");
    println!("====================================================================");

    println!("Введите значение по Фаренгейту:");

    let mut farengate = String::new();
    
    io::stdin()
        .read_line(&mut farengate)
        .expect("Ошибка чтения строки");

    let farengate: f32 = farengate.trim().parse().expect("Пожалуйста введите числовое значение!");
    
    let cels: f32 = convert_to_cels(farengate);

    println!("Тепмература по Цельсию равна: {cels}");
}

fn convert_to_cels(inner: f32) -> f32 {
    5.0 / 9.0 * (inner - 32.0)
}
