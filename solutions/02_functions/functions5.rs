fn square(num: i32) -> i32 {
    // Menghapus titik koma (`;`) di akhir baris akan membuat hasil ekspresi ini
    // dikembalikan secara implisit sebagai nilai kembalian dari fungsi.
    num * num
}

fn main() {
    let answer = square(3);
    println!("Kuadrat dari 3 adalah {answer}");
}
