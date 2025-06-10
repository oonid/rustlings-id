// Solusi untuk soal closure:
// - `square` diubah menjadi closure anonim yang disimpan di variabel.
// - Sintaks closure: `|parameter| expression`.
// - Expression pada closure secara implisit menjadi nilai kembalian (tanpa `;`).

fn main() {
    // Closure anonim: menerima `num: i32` lalu mengembalikan `num * num`
    let square = |num: i32| num * num;

    let answer = square(3);
    println!("Kuadrat dari 3 adalah {answer}");
}
