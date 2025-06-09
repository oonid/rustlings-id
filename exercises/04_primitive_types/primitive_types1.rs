// Boolean (`bool`)

fn main() {
    let is_morning = true;

    if is_morning {
        println!("Selamat pagi!");
    }

    // TODO: Sebelum pernyataan `if` di bawah ini, buat sebuah variabel baru
    // dengan nama `is_evening` yang bertipe boolean.
    // Nilai variabel ini harus merupakan negasi dari `is_morning` (yaitu kebalikannya).
    //
    // Petunjuk:
    // - Di Rust, kamu bisa membalik nilai boolean dengan tanda seru `!`.
    //   Contoh: `!true` akan menghasilkan `false`.
    // - Tanda seru `!` dapat digunakan juga pada variabel bertipe `bool`.
    // - Variabel baru harus dideklarasikan dengan `let`.
    //
    // Penjelasan:
    // - Tipe `bool` di Rust hanya memiliki dua kemungkinan nilai: `true` dan `false`.
    // - Operator logika `!` digunakan untuk negasi (membalik nilai boolean).
    // - Dalam kasus ini, karena `is_morning` bernilai `true`,
    //   maka `is_evening` akan bernilai `false`.

    // let ...

    if is_evening {
        println!("Selamat malam!");
    }
}
