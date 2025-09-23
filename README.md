<p align="center">
<img src="assets/logo.svg" alt="Async Rust — Примеры к подкасту" width="600" />
</p>

# 🦀 Async Rust — примеры к подкасту «Rust в деталях»

Этот репозиторий — приложение к подкасту **«Rust в деталях»**.
Здесь собраны минимальные, но наглядные примеры из выпусков про асинхронное программирование.
 

## 🎙 Подкаст: «Rust в деталях»
* 📘 **Выпуск 1 — BufReader | Как ускорить чтение файлов и потоков** → **[слушать](https://www.youtube.com/watch?v=gATf436M6ow)**
* 📘 **Выпуск 2 — Основы async Rust** → слушать

---
### 📂 Что внутри **src/bin**
* `00_cpu_usage` — мини-монитор ресурсов: процессы, потоки, загрузка CPU.
* `01_sequential_requests` — HTTP-запросы один за другим (последовательно).
* `02_concurrent_requests` — те же запросы, но конкурентно (tokio::join!).
* `03_async_fn_future` — демонстрация того, что async fn возвращает Future.
---

### 🚀 Запуск примеров

```bash
cargo run --bin 00_cpu_usage
cargo run --bin 01_sequential_requests
cargo run --bin 02_concurrent_requests
cargo run --bin 03_async_fn_future
```

Перед запуском можно собрать проект:

```bash
cargo build
```

> Примеры 1–2 выполняют HTTP-запросы к публичному тестовому API (httpbin.org).
> Через какое-то время этот ресурс может стать недоступен - такогда просто замените адрес на любой другой доступный сервис

---

## Полезные ссылки

* Async Book: [https://rust-lang.github.io/async-book/](https://rust-lang.github.io/async-book/)
* Tokio: [https://tokio.rs/](https://tokio.rs/)
* Reqwest: [https://docs.rs/reqwest/latest/reqwest/](https://docs.rs/reqwest/latest/reqwest/)
* Мой телеграм-канал «Маяк Программиста»: [https://t.me/prog\_lighthouse](https://t.me/prog_lighthouse)
* Блог неунывающего программиста — Eagle blog: [https://eagle2001.space/](https://eagle2001.space/)

---

MIT © EaGle