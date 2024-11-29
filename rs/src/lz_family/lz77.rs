use std::collections::HashMap;

fn update_window_map(
    cursor: usize,
    input_chars: &[char],
    match_length: usize,
    window_map: &mut HashMap<String, usize>,
) {
    for length in 1..=match_length + 1 {
        if cursor + length <= input_chars.len() {
            let substring: String = input_chars[cursor..cursor + length].iter().collect();
            window_map.insert(substring, cursor);
        }
    }
}

fn lz77_encode(input: &str, window_size: usize) -> Vec<(usize, usize, char)> {
    let mut encoded = Vec::new(); // Encoded output
    let input_chars: Vec<char> = input.chars().collect();
    let mut cursor = 0;
    let mut window_map: HashMap<String, usize> = HashMap::new();

    while cursor < input_chars.len() {
        let (match_distance, match_length) =
            find_longest_match(cursor, &input_chars, &window_map, window_size);

        let next_char = if cursor + match_length < input_chars.len() {
            input_chars[cursor + match_length]
        } else {
            '\0'
        };
        encoded.push((match_distance, match_length, next_char));
        update_window_map(cursor, &input_chars, match_length, &mut window_map);
        cursor += match_length + 1;
    }

    encoded
}
fn find_longest_match(
    cursor: usize,
    input_chars: &[char],
    window_map: &HashMap<String, usize>,
    window_size: usize,
) -> (usize, usize) {
    let mut match_distance = 0;
    let mut match_length = 0;

    for length in 1..=window_size.min(input_chars.len() - cursor) {
        let substring: String = input_chars[cursor..cursor + length].iter().collect();

        if let Some(&start_pos) = window_map.get(&substring) {
            match_distance = cursor - start_pos;
            match_length = length;
        } else {
            break;
        }
    }

    (match_distance, match_length)
}
fn lz77_decode(encoded: Vec<(usize, usize, char)>) -> String {
    let mut decoded = String::new();

    for (distance, length, next_char) in encoded {
        if distance > 0 {
            let start = decoded.len() - distance;
            let match_str: String = decoded[start..start + length].to_string();
            decoded.push_str(&match_str);
        }

        if next_char != '\0' {
            decoded.push(next_char);
        }
    }

    decoded
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode() {
        let input_string = "abracaddddjkejejvvdddddaadasdasdbra";
        let window_size = 15;
        let encoded = lz77_encode(input_string, window_size);
        println!("Encoded: {:?}", encoded);
        // Encode
        let encoded = lz77_encode(input_string, window_size);
        // Decode
        let decoded = lz77_decode(encoded);
        println!("Decoded: {}", decoded);
        assert_eq!(input_string, decoded)
    }
}
