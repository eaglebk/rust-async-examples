// Пример 2: Параллельные HTTP-запросы
// -----------------------------------
// Используем tokio::join! для параллельного (конкурентного) запуска нескольких запросов.
// В результате общее время выполнения значительно сокращается.

use tokio::{join, time::Instant};

async fn request(url: &str, c: i32) {
    println!("Запрос {}: {}", c, url);
    let client = reqwest::Client::new();
    let _resp = client.get(url).send().await.unwrap();
    println!("Запрос {} завершен", c);
}

#[tokio::main]
async fn main() {
    println!("Старт...");
    let start = Instant::now();

    // Создаём future, но не ожидаем результата сразу.
    let f1 = request("https://httpbin.org/delay/1", 1);
    let f2 = request("https://httpbin.org/delay/1", 2);
    let f3 = request("https://httpbin.org/delay/1", 3);
    let f4 = request("https://httpbin.org/delay/1", 4);


    // Ожидаем завершения future, используя join!
    let (_r1, _r2, _r3, _r4) = join!(f1, f2, f3, f4);

    println!("Время выполнения (конкурентно): {:?}", start.elapsed());
}
