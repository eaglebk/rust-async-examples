
// Утилита для проверки загрузки CPU и системы
// -----------------------------------------
// Выводит базовые метрики системы: ОС, количество процессов, потоков и загрузку CPU.


use std::process::Command;
use std::thread;
use sysinfo::{ProcessRefreshKind, ProcessesToUpdate, System};

/// Обновляет информацию о системе и возвращает ссылку на System
fn refresh_system() -> System {
    let mut system = System::new();
    system.refresh_processes_specifics(
        ProcessesToUpdate::All,
        true,
        ProcessRefreshKind::everything(),
    );
    system
}

fn get_thread_count_platform(system: &System) -> usize {
    // Получение количества потоков на основе tasks() крейта sysinfo (работает только на Linux)
    #[cfg(target_os = "linux")]
    {
        let thread_count: usize = system
            .processes()
            .values()
            .map(|p| p.tasks().map(|t| t.len()).unwrap_or(0))
            .sum();
        thread_count
    }
    #[cfg(target_os = "macos")]
    {
        let mut total_threads = 0;
        for (pid, _) in system.processes() {
            let pid_num = pid.as_u32();
            // Выполненяем: ps M <pid> | wc -l
            // Спасибо решению: https://superuser.com/a/753707
            let output = Command::new("sh")
                .arg("-c")
                .arg(&format!("ps M {} | wc -l", pid_num))
                .output();

            if let Ok(output) = output {
                if output.status.success() {
                    let stdout = String::from_utf8_lossy(&output.stdout);
                    if let Ok(count) = stdout.trim().parse::<i32>() {
                        // Вычитаем 1, потому что ps M включает заголовок
                        total_threads += (count - 1).max(0) as usize;
                    }
                }
            }
        }
        total_threads
    }
    #[cfg(target_os = "windows")]
    {
        let mut total_threads = 0;
        for (pid, _) in system.processes() {
            let pid_num = pid.as_u32();
            // Выполненяем: wmic где processid=<pid> >> ThreadCount
            let output = Command::new("wmic")
                .args(&[
                    "process",
                    "where",
                    &format!("processid={}", pid_num),
                    "get",
                    "ThreadCount",
                ])
                .output();

            if let Ok(output) = output {
                if output.status.success() {
                    let stdout = String::from_utf8_lossy(&output.stdout);

                    // Обрабатываем вывод wmic
                    // Вывод будет иметь вид: header\nvalue
                    let lines: Vec<&str> = stdout.lines().collect();
                    if lines.len() >= 2 {
                        // Получаем количество потоков из второй строки
                        if let Ok(count) = lines[1].trim().parse::<usize>() {
                            total_threads += count;
                        }
                    }
                }
            }
        }
        total_threads
    }
    #[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "windows")))]
    {
        // Для остальных ОС всегда возвращаем 0
        0
    }
}

fn get_cpu_usage() -> f32 {
    use sysinfo::CpuRefreshKind;
    let mut system = System::new();

    // Ждём немного для сбора данных о CPU
    system.refresh_cpu_specifics(CpuRefreshKind::everything());
    thread::sleep(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL);

    system.global_cpu_usage()
}

fn main() {
    println!("=== Системный монитор ===");

    let os = match System::name() {
        Some(name) if name == "Darwin" => "macOS".to_string(),
        Some(name) => name.to_string(),
        None => "Unknown OS".to_string(),
    };
    println!("ОС: {} {}", os, System::os_version().unwrap_or_default());

    let system = refresh_system();
    let processes = system.processes().len();
    let threads = get_thread_count_platform(&system);
    let cpu_usage = get_cpu_usage();

    println!();
    println!("--- Текущие метрики ---");
    println!("Процессов: {}", processes);
    println!("Потоков: {}", threads);
    println!("Использование CPU: {:.1}%", cpu_usage);
}
