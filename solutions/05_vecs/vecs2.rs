fn vec_loop(input: &[i32]) -> Vec<i32> {
    let mut output = Vec::new();

    for element in input {
        output.push(2 * element);
    }

    output
}

fn vec_map_example(input: &[i32]) -> Vec<i32> {
    // Contoh menggunakan iter dan map untuk menambah 1 ke tiap elemen.
    input.iter().map(|element| element + 1).collect()
}

fn vec_map(input: &[i32]) -> Vec<i32> {
    // Menggunakan pendekatan deklaratif dengan iterator dan map.
    // Lebih efisien karena `collect()` tahu ukuran akhir dan bisa mengalokasi lebih cepat.
    input.iter().map(|element| 2 * element).collect()
}

fn main() {
    // Opsional: bereksperimen di sini.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vec_loop() {
        let input = [2, 4, 6, 8, 10];
        let ans = vec_loop(&input);
        assert_eq!(ans, [4, 8, 12, 16, 20]);
    }

    #[test]
    fn test_vec_map_example() {
        let input = [1, 2, 3];
        let ans = vec_map_example(&input);
        assert_eq!(ans, [2, 3, 4]);
    }

    #[test]
    fn test_vec_map() {
        let input = [2, 4, 6, 8, 10];
        let ans = vec_map(&input);
        assert_eq!(ans, [4, 8, 12, 16, 20]);
    }
}
