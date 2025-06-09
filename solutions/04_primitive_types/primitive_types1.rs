fn main() {
    // Variabel boolean `is_morning` dideklarasikan dan diberi nilai `true`.
    // Artinya, saat program dijalankan, kita menganggap waktu sekarang adalah pagi.
    let is_morning = true;

    // Jika `is_morning` bernilai true, maka akan mencetak pesan "Selamat pagi!" ke layar.
    if is_morning {
        println!("Selamat pagi!");
    }

    // Di bawah ini kita membuat variabel baru `is_evening`, yang merupakan kebalikan dari `is_morning`.
    // Karena `is_morning` adalah `true`, maka `!is_morning` menjadi `false`, sehingga `is_evening` akan bernilai `false`.
    //
    // Operator `!` adalah operator logika negasi dalam Rust. Ia membalik nilai boolean:
    // - `!true` menjadi `false`
    // - `!false` menjadi `true`
    let is_evening = !is_morning;

    // Karena `is_evening` bernilai `false`, maka bagian ini tidak akan mencetak apapun.
    if is_evening {
        println!("Selamat malam!");
    }
}
