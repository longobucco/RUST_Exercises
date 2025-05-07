use std::fs::File;
use std::io::{self, Read};

fn stats(text: &str) -> [u32; 26] {
    let mut counts = [0; 26];

    for c in text.chars() {
        let c = c.to_ascii_lowercase();
        if c.is_ascii_alphabetic() {
            let index = (c as u8 - b'a') as usize;
            counts[index] += 1;
        }
    }

    counts
}

fn is_pangram(counts: &[u32]) -> bool {
    counts.iter().all(|&count| count > 0)
}

pub fn run_pangram() -> io::Result<()> {
    let mut file = File::open("sentence.txt")?;

    let mut string = String::new();
    file.read_to_string(&mut string)?;

    println!("Contenuto del file:\n{}", string);

    // Usa la funzione stats per ottenere il conteggio delle lettere
    let letter_count = stats(&string);
    println!("Conteggio delle lettere:");
    for (i, &count) in letter_count.iter().enumerate() {
        println!("{}: {}", (b'a' + i as u8) as char, count);
    }

    // Verifica se la stringa è un pangramma
    let is_pangram = is_pangram(&letter_count);
    println!("La stringa è un pangramma: {}", is_pangram);

    Ok(())
}

// Modulo di test
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_ones() {
        let counts = [1; 26];
        assert!(is_pangram(&counts));
    }

    #[test]
    fn test_some_zeros() {
        let mut counts = [0; 26];
        counts[0] = 0;
        counts[1] = 0;
        assert!(!is_pangram(&counts));
    }

    #[test]
    fn test_increasing_counts() {
        let mut counts = [0; 26];
        for i in 0..26 {
            counts[i] = i as u32 + 1;
        }
        assert!(is_pangram(&counts));
    }

    #[test]
    fn test_wrong_size() {
        let counts = [1; 25];
        assert!(!is_pangram(&counts));
    }

    #[test]
    fn test_stats_on_full_alphabet() {
        let counts = stats("abcdefghijklmnopqrstuvwxyz");
        for c in counts {
            assert!(c == 1);
        }
    }

    #[test]
    fn test_stats_on_empty_string() {
        let counts = stats("");
        for c in counts {
            assert!(c == 0);
        }
    }

    #[test]
    fn test_stats_missing_char() {
        let counts = stats("abcdefghijklmnopqrstuvwxy");
        for c in counts.iter().take(25) {
            assert!(*c == 1);
        }
        assert!(counts[25] == 0);
    }

    #[test]
    fn test_stats_on_full_string() {
        let contents = "The quick brown fox jumps over the lazy dog";
        let counts = stats(contents);
        for c in counts {
            assert!(c > 0);
        }
    }

    #[test]
    fn test_stats_with_punctuation() {
        let contents = "The quick brown fox jumps over the lazy dog!";
        let counts = stats(contents);
        for c in counts {
            assert!(c > 0);
        }
    }

    #[test]
    fn test_missing_char_on_full_string() {
        let contents = "The quick brown fox jumps over the laz* dog";
        let counts = stats(contents);
        println!("{:?}", counts);
        for (i, c) in counts.iter().enumerate() {
            if i == 24 {
                assert!(*c == 0);
            } else {
                assert!(*c > 0);
            }
        }
    }

    #[test]
    fn test_is_pangram() {
        let counts = stats("The quick brown fox jumps over the lazy dog");
        assert!(is_pangram(&counts));
    }
}

fn main() {
    if let Err(e) = run_pangram() {
        eprintln!("Errore durante l'esecuzione: {}", e);
    }
}