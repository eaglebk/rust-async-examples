// Пример 4: Собственная реализация Future
// ---------------------------------------
// Демонстрация того, как работает Poll::Pending и Poll::Ready
// а также планирование повторного опроса.

use std::{
    pin::Pin,
    task::{Context, Poll},
};
use futures::Future;

struct MyFuture {
    counter: u8,
}

impl Future for MyFuture {
    type Output = &'static str; // Тип результата для Future 

    // функция опроса Future
    // принимает self: Pin<&mut Self> - ссылку на себя
    // cx: &mut Context<'_> - контекст опроса Future
    // Возвращает Poll<Self::Output> - результат опроса
    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if self.counter < 3 {
            self.counter += 1;
            println!("Ещё не готово, counter = {}, возвращаем Pending", self.counter);
            cx.waker().wake_by_ref(); // Планируем повторный опрос
            Poll::Pending
        } else {
            println!("Готово!");
            Poll::Ready("Результат получен")
        }
    }
}

#[tokio::main]
async fn main() {
    let my_future = MyFuture { counter: 0 };
    let result = my_future.await;
    println!("Результат: {}", result);
}
