fn encode(input:&str) -> String{

    let mut output=String::new();
    let mut count = 1;
    for (index, value) in input.chars().enumerate() {
        if input.chars().nth(index+1).is_some() {
            // println!("{:?}",value.eq(&input.chars().nth(index+1).unwrap()));
            if value.eq(&input.chars().nth(index+1).unwrap()) && count < 9  {
                count += 1;
            }else{
                output.push_str(&value.to_string());

                output.push_str(&count.to_string());
                count = 1;
            }
        }else{
            output.push_str(&value.to_string());
            output.push_str(&count.to_string());
        }

        // println!("index = {} and value = {}", index, value);
    }
    output
}

fn decode(input:&str)->String{
    let mut output=String::new();
    for (index, value) in input.chars().enumerate() {
          match  value.to_string().parse::<i8>(){
              Ok(val) => {
                  for i in 0..val {
                      output.push_str(&input.chars().nth(index-1).unwrap().to_string());
                  }
              }
              Err(_) => {
                 continue;
              }
          }
    }

    output
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_encode() {
        let val1=encode("SSSSOOOEEERROOOAAYYYYYDDDDOEUUUUUWWWWJJJORRUUUUUUUUUUXXXKHHHHHHMMMMMMGGGLLLLLLLJJJJ");
        assert_eq!(val1, "S4O3E3R2O3A2Y5D4O1E1U5W4J3O1R2U9U1X3K1H6M6G3L7J4");
        let val1=encode("AHDNAJKDAKSDJAHDKAJHUIWQWRHQIWOURHKSNDSDKFSNFLKSFJSLKFJSLEIFJPFOIJFLSKFSLKDFJSLFKJSLKFJSLFKSJFLWFJOWOWIOWIWIQICNVNVVNVNVNFFHJFJFJDKLSLSLSPPOFOFOFKMVVJFJFJDIEIISISISISISOXOXOXXOCOKVVKJFJFJHHDHDHDJHSKS");
        assert_eq!(val1, "A1H1D1N1A1J1K1D1A1K1S1D1J1A1H1D1K1A1J1H1U1I1W1Q1W1R1H1Q1I1W1O1U1R1H1K1S1N1D1S1D1K1F1S1N1F1L1K1S1F1J1S1L1K1F1J1S1L1E1I1F1J1P1F1O1I1J1F1L1S1K1F1S1L1K1D1F1J1S1L1F1K1J1S1L1K1F1J1S1L1F1K1S1J1F1L1W1F1J1O1W1O1W1I1O1W1I1W1I1Q1I1C1N1V1N1V2N1V1N1V1N1F2H1J1F1J1F1J1D1K1L1S1L1S1L1S1P2O1F1O1F1O1F1K1M1V2J1F1J1F1J1D1I1E1I2S1I1S1I1S1I1S1I1S1O1X1O1X1O1X2O1C1O1K1V2K1J1F1J1F1J1H2D1H1D1H1D1J1H1S1K1S1");
        let val1=encode("");
        assert_eq!(val1, "");
    }
    #[test]
    fn test_decode() {
        let val1=decode("S4O3E3R2O3A2Y5D4O1E1U5W4J3O1R2U9U1X3K1H6M6G3L7J4");
        assert_eq!("SSSSOOOEEERROOOAAYYYYYDDDDOEUUUUUWWWWJJJORRUUUUUUUUUUXXXKHHHHHHMMMMMMGGGLLLLLLLJJJJ",val1);
        let val1=decode("A1H1D1N1A1J1K1D1A1K1S1D1J1A1H1D1K1A1J1H1U1I1W1Q1W1R1H1Q1I1W1O1U1R1H1K1S1N1D1S1D1K1F1S1N1F1L1K1S1F1J1S1L1K1F1J1S1L1E1I1F1J1P1F1O1I1J1F1L1S1K1F1S1L1K1D1F1J1S1L1F1K1J1S1L1K1F1J1S1L1F1K1S1J1F1L1W1F1J1O1W1O1W1I1O1W1I1W1I1Q1I1C1N1V1N1V2N1V1N1V1N1F2H1J1F1J1F1J1D1K1L1S1L1S1L1S1P2O1F1O1F1O1F1K1M1V2J1F1J1F1J1D1I1E1I2S1I1S1I1S1I1S1I1S1O1X1O1X1O1X2O1C1O1K1V2K1J1F1J1F1J1H2D1H1D1H1D1J1H1S1K1S1");
        assert_eq!("AHDNAJKDAKSDJAHDKAJHUIWQWRHQIWOURHKSNDSDKFSNFLKSFJSLKFJSLEIFJPFOIJFLSKFSLKDFJSLFKJSLKFJSLFKSJFLWFJOWOWIOWIWIQICNVNVVNVNVNFFHJFJFJDKLSLSLSPPOFOFOFKMVVJFJFJDIEIISISISISISOXOXOXXOCOKVVKJFJFJHHDHDHDJHSKS",val1);
    }
}