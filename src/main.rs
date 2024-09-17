use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    // Создаем счетчик, обернутый в Arc и Mutex.
    let counter = Arc::new(Mutex::new(0));

    // Создаем вектор для хранения потоков.
    let mut handles = vec![];

    // Запускаем несколько потоков.
    for _ in 0..10 {
        // Клонируем Arc для использования в новом потоке.
        let counter_clone = Arc::clone(&counter);

        // Запускаем поток.
        let handle = thread::spawn(move || {
            // Получаем доступ к внутреннему значению счетчика через Mutex.
            let mut num = counter_clone.lock().unwrap();
            // Инкрементируем значение счетчика.
            *num += 1;
        });

        // Сохраняем дескриптор потока.
        handles.push(handle);
    }

    // Ожидаем завершения всех потоков.
    for handle in handles {
        handle.join().unwrap();
    }

    // Выводим итоговое значение счетчика.
    let final_count = *counter.lock().unwrap();
    println!("Итоговое значение счетчика: {}", final_count);
}
