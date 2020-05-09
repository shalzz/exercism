use std::collections::HashMap;

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    for item in dna.chars().chain(std::iter::once(nucleotide)) {
        if !matches!(item, 'A' | 'C' | 'G' | 'T') {
            return Err(item);
        }
    }

    Ok(dna.chars().filter(|&ch| ch == nucleotide).count())
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    let mut map = HashMap::new();
    for item in &['A', 'C', 'G', 'T'] {
        map.insert(*item, count(*item, dna)?);
    }

    Ok(map)
}
