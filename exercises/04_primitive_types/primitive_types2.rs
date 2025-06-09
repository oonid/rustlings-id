// Karakter (`char`)

fn main() {
    // Perhatikan penggunaan tanda petik tunggal (_single quotes_).
    // Ini berbeda dengan tanda petik ganda yang biasa kamu lihat di string.

    let my_first_initial = 'C';

    if my_first_initial.is_alphabetic() {
        println!("Bersifat alfabet!");
    } else if my_first_initial.is_numeric() {
        println!("Bersifat numerik!");
    } else {
        println!("Bukan alfabet maupun numerik!");
    }

    // TODO:
    // - Seperti contoh di atas, deklarasikan variabel bernama `your_character`
    // - Coba isi dengan huruf, angka, karakter khusus, huruf dari bahasa lain, atau emoji.
    // - Gunakan tanda petik tunggal (contoh: 'a', '9', '@', '😀')
    //
    // Petunjuk:
    // - Gunakan fungsi `.is_alphabetic()` untuk mengecek apakah karakter berupa huruf.
    // - Gunakan fungsi `.is_numeric()` untuk mengecek apakah karakter berupa angka.
    // - Fungsi-fungsi ini tersedia langsung untuk tipe data `char`.

    // let your_character = '';

    if your_character.is_alphabetic() {
        println!("Bersifat alfabet!");
    } else if your_character.is_numeric() {
        println!("Bersifat numerik!");
    } else {
        println!("Bukan alfabet maupun numerik!");
    }
}
