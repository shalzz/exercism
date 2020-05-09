#[derive(Debug, PartialEq)]
pub struct DNA {
    strand: String,
}

#[derive(Debug, PartialEq)]
pub struct RNA {
    strand: String,
}

impl DNA {
    pub fn new(dna: &str) -> Result<DNA, usize> {
        for (index, ch) in dna.chars().enumerate() {
            match ch {
                'A' | 'C' | 'G' | 'T' => continue,
                _ => return Err(index),
            }
        }

        Ok(DNA {
            strand: dna.to_string(),
        })
    }

    pub fn into_rna(self) -> RNA {
        RNA {
            strand: self
                .strand
                .chars()
                .map(|ch| match ch {
                    'G' => 'C',
                    'C' => 'G',
                    'T' => 'A',
                    'A' => 'U',
                    _ => unreachable!(),
                })
                .collect(),
        }
    }
}

impl RNA {
    pub fn new(rna: &str) -> Result<RNA, usize> {
        for (index, ch) in rna.char_indices() {
            match ch {
                'A' | 'C' | 'G' | 'U' => continue,
                _ => return Err(index),
            }
        }

        Ok(RNA {
            strand: rna.to_string(),
        })
    }
}
