// Задание из книги
// Конвертация температур между значениями по Фаренгейту к Цельсия.
// C = (F — 32)/1,8  - Формула ;)
use std::io;

fn main() {
    loop {
        println!("Enter the number: ");
        let mut input_user: String = String::new(); // Хранится ввод

        io::stdin()
            .read_line(&mut input_user)// Получаем ввод от пользователя и записываем в input_user
            .expect("Failed to read line");

        let input_user: f32 = match input_user.trim().parse() { // Конвертируем из строки в число с плавающей точкой
            Ok(num) => num, // Проверка, является ли строка числом
            Err(_) => continue, // Если ввели что-то кроме числа, начинаем заново.
        };
        println!("{}", (input_user - 32.0)/1.8); // Выводим результат
        break; // Завершаем работу приложения
    }
}
