use std::{
    vec::Vec,
};

pub fn vector_str_to_int(vector: Vec<String>) -> Vec<i16> {
    return vector.into_iter().map(|x| x.parse::<i16>().unwrap()).collect();
}

pub fn chunk_parts(lines: &Vec<String>) -> Vec<String> {
    let mut chunks: Vec<String> = Vec::new();
    let mut chunk = Vec::<String>::new();

    for line in lines {
        if line.is_empty() {
            let contig_chunk = chunk.join(" ");
            chunks.push(contig_chunk);
            chunk = Vec::<String>::new();
            continue;
        }

        chunk.push(line.to_string());
    }

    if !chunk.is_empty() {
        chunks.push(chunk.join(" "));
    }

    return chunks;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn chunk_parts_test() {
        let lines = vec![        
            "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd",
            "byr:1937 iyr:2017 cid:147 hgt:183cm",
            "",
            "iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884",
            "hcl:#cfa07d byr:1929",
            "",
            "hcl:#ae17e1 iyr:2013",
            "eyr:2024",
            "ecl:brn pid:760753108 byr:1931",
            "hgt:179cm",
            "",
            "hcl:#cfa07d eyr:2025 pid:166559648",
            "iyr:2011 ecl:brn hgt:59in"]
            .iter().map(|x| x.to_string()).collect();
        
        let chunked_lines = chunk_parts(&lines);
        assert_eq!("ecl:gry pid:860033327 eyr:2020 hcl:#fffffd byr:1937 iyr:2017 cid:147 hgt:183cm", chunked_lines.get(0).unwrap());
        assert_eq!("iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884 hcl:#cfa07d byr:1929", chunked_lines.get(1).unwrap());
        assert_eq!("hcl:#ae17e1 iyr:2013 eyr:2024 ecl:brn pid:760753108 byr:1931 hgt:179cm", chunked_lines.get(2).unwrap());
        assert_eq!("hcl:#cfa07d eyr:2025 pid:166559648 iyr:2011 ecl:brn hgt:59in", chunked_lines.get(3).unwrap());
    }
}