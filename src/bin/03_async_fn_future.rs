// Пример 3: async fn под капотом
// -------------------------------
// async-функция возвращает не результат напрямую,
// а Future, которое будет выполнено позже.

async fn say_hello() -> String {
    "Hello из async-функции!".to_string()
}

#[tokio::main]
async fn main() {
    let fut = say_hello(); // fut: impl Future<Output = String>
    let result = fut.await;
    println!("Результат: {}", result);
}
