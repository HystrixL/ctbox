use std::mem::swap;

pub fn encrypt(decrypt: bool, source: &str) -> String {
    let mut first_key = "689abcrstu%012345vwxyABCDEFGdefghMNOPQRijklmnpqHIJKSTUVWXYZ";
    let mut second_key = "rsHYZ23tFhiIJjku9abP5QRScABd8DVWXElmGvwK%01xyC4npqMgNOTU6ef";
    if decrypt {
        swap(&mut first_key, &mut second_key);
    }

    source
        .chars()
        .enumerate()
        .map(|(index, value)| {
            if let Some(index_in_first) = first_key.chars().position(|x| x == value) {
                let mut index_in_first = index_in_first as i32;
                index_in_first += if decrypt {
                    index as i32
                } else {
                    -(index as i32)
                };
                index_in_first %= first_key.len() as i32;
                if index_in_first < 0 {
                    index_in_first += first_key.len() as i32;
                }
                let index_in_first = index_in_first as usize;

                second_key.chars().nth(index_in_first).unwrap()
            } else {
                source.chars().last().unwrap()
            }
        })
        .collect()
}
