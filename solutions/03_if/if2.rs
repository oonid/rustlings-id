fn picky_eater(food: &str) -> &str {
    // Percabangan berdasarkan nilai masukan `food`.
    // Setiap kondisi mengembalikan string literal bertipe `&str` sesuai dengan yang diharapkan.
    if food == "strawberry" {
        "Yummy!"
    } else if food == "potato" {
        "I guess I can eat that."
    } else {
        "No thanks!"
    }
}

fn main() {
    // Kamu boleh bereksperimen di sini (opsional).
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn yummy_food() {
        assert_eq!(picky_eater("strawberry"), "Yummy!");
    }

    #[test]
    fn neutral_food() {
        assert_eq!(picky_eater("potato"), "I guess I can eat that.");
    }

    #[test]
    fn default_disliked_food() {
        assert_eq!(picky_eater("broccoli"), "No thanks!");
        assert_eq!(picky_eater("gummy bears"), "No thanks!");
        assert_eq!(picky_eater("literally anything"), "No thanks!");
    }
}
