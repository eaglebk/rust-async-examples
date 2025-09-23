// Пример 1: Последовательные HTTP-запросы
// ---------------------------------------
// Несмотря на использование async/await, код остаётся синхронным по сути:
// каждый новый запрос ждёт завершения предыдущего.

use tokio::time::Instant;

#[tokio::main]
async fn main() {
    println!("Старт...");
    let urls = vec![
        "https://httpbin.org/delay/1",
        "https://httpbin.org/delay/1",
        "https://httpbin.org/delay/1",
        "https://httpbin.org/delay/1",
    ];

    let client = reqwest::Client::new();
    let start = Instant::now();

    for (i, url) in urls.iter().enumerate()   {
 
        let c = i+1;
        println!("Запрос {}: {}", c, url);

        // .await приостанавливает текущую задачу,
        // остальные async-задачи при этом могут выполняться.
        let _resp = client.get(*url).send().await.unwrap();
        println!("Запрос {} завершен", c);
    }

    println!("Время выполнения (последовательно): {:?}", start.elapsed());
}
