#![feature(test)]

extern crate test;
extern crate regex;

#[macro_use] extern crate lazy_static;

use regex::Regex;

static tests: [&'static str; 8] = [
    "s/some words/other words/",
    "s/some words//",
    "s/some words/",
    "s/a/a/a/a/a/a/a/a/a/a/a/a/a/a/a/a/a/a/a/a/a/a/a/a/a/a/a/a/a/a/a/a/a/a/a/a/a/a/a/a/a/a/a/a/a/a/a/",
    "s/some words words words words words words words words words words words words words words words words words words words words words words words words words words words words/other words words words words words words words words words words words words words words words words words words words words words words words/",
    "s/These \\/ are \\/ words \\/ with \\/ escaped \\/ sequences \\/ are \\/ words \\/ with \\/ escaped \\/ sequences \\/ are \\/ words \\/ with \\/ escaped \\/ sequences \\/ are \\/ words \\/ with \\/ escaped \\/ sequences \\/ are \\/ words \\/ with \\/ escaped \\/ sequences \\/ are \\/ words \\/ with \\/ escaped \\/ sequences \\/ are \\/ words \\/ with \\/ escaped \\/ sequences \\/ are \\/ words \\/ with \\/ escaped \\/ sequences \\/ are \\/ words \\/ with \\/ escaped \\/ sequences \\/ are \\/ words \\/ with \\/ escaped \\/ sequences \\/ are \\/ words \\/ with \\/ escaped \\/ sequences \\/ are \\/ words \\/ with \\/ escaped \\/ sequences \\/ are \\/ words \\/ with \\/ escaped \\/ sequences \\/ are \\/ words \\/ with \\/ escaped \\/ sequences/Random \\/ Words \\/ From \\/ other \\/ otra \\/ ander \\/ other \\/ otra \\/ ander \\/ other \\/ otra \\/ ander \\/ other \\/ otra \\/ ander \\/ other \\/ otra \\/ ander \\/ other \\/ otra \\/ ander \\/ other \\/ otra \\/ ander \\/ other \\/ otra \\/ ander \\/ other \\/ otra \\/ ander \\/ other \\/ otra \\/ ander \\/ other \\/ otra \\/ ander \\/ other \\/ otra \\/ ander \\/ other \\/ otra \\/ ander \\/ other \\/ otra \\/ ander \\/ other \\/ otra \\/ ander \\/ other \\/ otra \\/ ander \\/ other \\/ otra \\/ ander \\/ other \\/ otra \\/ ander \\/ other \\/ otra \\/ ander \\/ other \\/ otra \\/ ander \\/ other \\/ otra \\/ ander \\/ languages /",
    "s/TheseAAareAAwordsAAwithAAescapedAAsequencesAAareAAwordsAAwithAAescapedAAsequencesAAareAAwordsAAwithAAescapedAAsequencesAAareAAwordsAAwithAAescapedAAsequencesAAareAAwordsAAwithAAescapedAAsequencesAAareAAwordsAAwithAAescapedAAsequencesAAareAAwordsAAwithAAescapedAAsequencesAAareAAwordsAAwithAAescapedAAsequencesAAareAAwordsAAwithAAescapedAAsequencesAAareAAwordsAAwithAAescapedAAsequencesAAareAAwordsAAwithAAescapedAAsequencesAAareAAwordsAAwithAAescapedAAsequencesAAareAAwordsAAwithAAescapedAAsequencesAAareAAwordsAAwithAAescapedAAsequences/RandomAAWordsAAFromAAotherAAotraAAanderAAotherAAotraAAanderAAotherAAotraAAanderAAotherAAotraAAanderAAotherAAotraAAanderAAotherAAotraAAanderAAotherAAotraAAanderAAotherAAotraAAanderAAotherAAotraAAanderAAotherAAotraAAanderAAotherAAotraAAanderAAotherAAotraAAanderAAotherAAotraAAanderAAotherAAotraAAanderAAotherAAotraAAanderAAotherAAotraAAanderAAotherAAotraAAanderAAotherAAotraAAanderAAotherAAotraAAanderAAotherAAotraAAanderAAotherAAotraAAanderAAlanguages /",
    "s/T/h/e/s/e/ \\/ are \\/ words \\/ with \\/ escaped \\/ sequences \\/ are \\/ words \\/ with \\/ escaped \\/ sequences \\/ are \\/ words \\/ with \\/ escaped \\/ sequences \\/ are \\/ words \\/ with \\/ escaped \\/ sequences \\/ are \\/ words \\/ with \\/ escaped \\/ sequences \\/ are \\/ words \\/ with \\/ escaped \\/ sequences \\/ are \\/ words \\/ with \\/ escaped \\/ sequences \\/ are \\/ words \\/ with \\/ escaped \\/ sequences \\/ are \\/ words \\/ with \\/ escaped \\/ sequences \\/ are \\/ words \\/ with \\/ escaped \\/ sequences \\/ are \\/ words \\/ with \\/ escaped \\/ sequences \\/ are \\/ words \\/ with \\/ escaped \\/ sequences \\/ are \\/ words \\/ with \\/ escaped \\/ sequences \\/ are \\/ words \\/ with \\/ escaped \\/ sequences/Random \\/ Words \\/ From \\/ other \\/ otra \\/ ander \\/ other \\/ otra \\/ ander \\/ other \\/ otra \\/ ander \\/ other \\/ otra \\/ ander \\/ other \\/ otra \\/ ander \\/ other \\/ otra \\/ ander \\/ other \\/ otra \\/ ander \\/ other \\/ otra \\/ ander \\/ other \\/ otra \\/ ander \\/ other \\/ otra \\/ ander \\/ other \\/ otra \\/ ander \\/ other \\/ otra \\/ ander \\/ other \\/ otra \\/ ander \\/ other \\/ otra \\/ ander \\/ other \\/ otra \\/ ander \\/ other \\/ otra \\/ ander \\/ other \\/ otra \\/ ander \\/ other \\/ otra \\/ ander \\/ other \\/ otra \\/ ander \\/ other \\/ otra \\/ ander \\/ other \\/ otra \\/ ander \\/ languages /",
];

fn main() {
    for test in &tests {
        //println!("FSM: {:?}", fsm_mode(test));
        //println!("FSM!!{:?}", fsm_mode_psuedo(test));
        //println!("RGX: {:?}", regex_mode(test));
        println!("???: {:?}", get_bounds(test));
        println!();
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct PsuedoVec<T: Copy> {
    data: [T; 32],
    pub length: usize,
}

impl<T: Copy> PsuedoVec<T> {
    #[inline]
    pub fn new(default: T) -> Self {
        PsuedoVec {
            data: [default; 32],
            length: 0,
        }
    }
    
    #[inline]
    pub fn push(&mut self, item: T) {
        self.data[self.length] = item;
        self.length += 1;
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.length
    }
}


impl<T: Copy> std::ops::Index<usize> for PsuedoVec<T> {
    type Output = T;

    #[inline]
    fn index(&self, index: usize) -> &T {
        &self.data[index]
    }
}

#[inline]
fn get_boundaries_psuedo(string: &str) -> PsuedoVec<usize> { // Better than regex
    let mut boundaries = PsuedoVec::new(0usize); // hopefully four is enough
    let mut previous_char = '/';

    for (index,cha) in string[1..].char_indices() {
        if '/' == cha && previous_char != '\\' {
            boundaries.push(index + 1);

            if boundaries.len() > 3 {
                break;
            }
        }

        if cha == '\\' && previous_char == '\\' {
            previous_char = ' ';
        } else {
            previous_char = cha;
        }
    }

    if boundaries[boundaries.len() - 1] != string.len() - 1 {
        boundaries.push(string.len());
    }

    if boundaries.len() == 2 {
        let i = boundaries[1];
        boundaries.push(i+1);
    }

    boundaries
}

#[inline]
fn fsm_mode_psuedo(text: &str) -> Option<(&str, &str)> {
    let boundaries = get_boundaries_psuedo(text);

    if boundaries.len() != 3 {
        return None;
    }

    let s1 = &text[boundaries[0]+1..boundaries[1]];
    let s2 = &text[boundaries[1]+1..boundaries[2]];

    Some((s1, s2))
}


#[inline]
fn get_boundaries(string: &str) -> Vec<usize> { // Better than regex
    let mut boundaries = Vec::with_capacity(8); // hopefully four is enough
    let mut previous_char = '/';

    for (index,cha) in string[1..].char_indices() {
        if '/' == cha && previous_char != '\\' {
            boundaries.push(index + 1);

            if boundaries.len() > 3 {
                break;
            }
        }

        if cha == '\\' && previous_char == '\\' {
            previous_char = ' ';
        } else {
            previous_char = cha;
        }

    }

    if boundaries[boundaries.len() - 1] != string.len() - 1 {
        boundaries.push(string.len());
    }

    if boundaries.len() == 2 {
        let i = boundaries[1];
        boundaries.push(i+1);
    }

    boundaries
}

#[inline]
fn fsm_mode(text: &str) -> Option<(&str, &str)> {
    let boundaries = get_boundaries(text);

    if boundaries.len() != 3 {
        return None;
    }

    let s1 = &text[boundaries[0]+1..boundaries[1]];
    let s2 = &text[boundaries[1]+1..boundaries[2]];

    Some((s1, s2))
}


#[inline]
fn get_bounds_2(string: &str) -> Option<(&str, &str)> { // Better than regex
    let mut boundaries = [0; 3];
    let mut length = 0;
    let mut previous_char = '/';

    for (index,cha) in string[1..].char_indices() {
        if '/' == cha && previous_char != '\\' {
            if length == 3 {
                return None;
            }

            boundaries[length] = index + 1;
            length += 1;
        }

        if cha == '\\' && previous_char == '\\' {
            previous_char = ' ';
        } else {
            previous_char = cha;
        }
    }

    //println!("{:?}", boundaries);

    if length < 2 {
        return None;
    }

    //println!("{:?}", boundaries);

    if boundaries[length - 1] != string.len() - 1 {
        boundaries[2] = string.len();
    }

    let s1 = &string[boundaries[0]+1..boundaries[1]];
    let s2 = &string[boundaries[1]+1..boundaries[2]];
    //println!("\nsubstitute [{}] for [{}]", s1, s2);

    Some((s1, s2))
}


#[inline]
fn get_bounds(string: &str) -> Option<(&str, &str)> { // Better than regex
    let mut boundaries = [0; 3];
    let mut length = 0;
    let mut previous_char = '/';

    for (index,cha) in string[1..].char_indices() {
        if cha == '/' && previous_char != '\\' {
            if length == 3 {
                return None;
            }
            
            boundaries[length] = index + 1;
            length += 1;
        }

        if cha == '\\' && previous_char == '\\' {
            previous_char = ' ';
        } else {
            previous_char = cha;
        }
    }

    if boundaries[length - 1] != string.len() - 1 {
        boundaries[length] = string.len();
        length += 1;
    }

    let s1 = &string[boundaries[0]+1..boundaries[1]];

    let s2 = if length == 2 {
        ""
    } else {
        &string[boundaries[1]+1..boundaries[2]]
    };

    Some((s1, s2))
}


lazy_static! {
    static ref SED_REGEX: Regex = Regex::new(r"^s/((?:\\/|[^/])+)/((?:\\/|[^/])*)/?$").unwrap();
}

fn regex_mode(text: &str) -> Option<(&str, &str)> {
    let caps = SED_REGEX.captures(text)?;
    let s1 = caps.get(1)?.as_str();
    let s2 = caps.get(2)?.as_str();

    Some((s1, s2))
}



#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[bench]
    fn bench_best_0(b: &mut Bencher) {
        b.iter(|| get_bounds(tests[1]));
    }

    #[bench]
    fn bench_best_1(b: &mut Bencher) {
        b.iter(|| get_bounds(tests[1]));
    }

    #[bench]
    fn bench_best_2(b: &mut Bencher) {
        b.iter(|| get_bounds(tests[2]));
    }

    #[bench]
    fn bench_best_3(b: &mut Bencher) {
        b.iter(|| get_bounds(tests[3]));
    }

    #[bench]
    fn bench_best_4(b: &mut Bencher) {
        b.iter(|| get_bounds(tests[4]));
    }

    #[bench]
    fn bench_best_5(b: &mut Bencher) {
        b.iter(|| get_bounds(tests[5]));
    }

    #[bench]
    fn bench_best_6(b: &mut Bencher) {
        b.iter(|| get_bounds(tests[6]));
    }





    #[bench]
    fn bench_fsm_0(b: &mut Bencher) {
        b.iter(|| fsm_mode(tests[0]));
    }

    #[bench]
    fn bench_fsm_1(b: &mut Bencher) {
        b.iter(|| fsm_mode(tests[1]));
    }

    #[bench]
    fn bench_fsm_2(b: &mut Bencher) {
        b.iter(|| fsm_mode(tests[2]));
    }

    #[bench]
    fn bench_fsm_3(b: &mut Bencher) {
        b.iter(|| fsm_mode(tests[3]));
    }

    #[bench]
    fn bench_fsm_4(b: &mut Bencher) {
        b.iter(|| fsm_mode(tests[4]));
    }

    #[bench]
    fn bench_fsm_5(b: &mut Bencher) {
        b.iter(|| fsm_mode(tests[5]));
    }

    #[bench]
    fn bench_fsm_6(b: &mut Bencher) {
        b.iter(|| fsm_mode(tests[6]));
    }



    #[bench]
    fn bench_fsm_psuedovec_0(b: &mut Bencher) {
        b.iter(|| fsm_mode_psuedo(tests[0]));
    }

    #[bench]
    fn bench_fsm_psuedovec_1(b: &mut Bencher) {
        b.iter(|| fsm_mode_psuedo(tests[1]));
    }

    #[bench]
    fn bench_fsm_psuedovec_2(b: &mut Bencher) {
        b.iter(|| fsm_mode_psuedo(tests[2]));
    }

    #[bench]
    fn bench_fsm_psuedovec_3(b: &mut Bencher) {
        b.iter(|| fsm_mode_psuedo(tests[3]));
    }

    #[bench]
    fn bench_fsm_psuedovec_4(b: &mut Bencher) {
        b.iter(|| fsm_mode_psuedo(tests[4]));
    }

    #[bench]
    fn bench_fsm_psuedovec_5(b: &mut Bencher) {
        b.iter(|| fsm_mode_psuedo(tests[5]));
    }

    #[bench]
    fn bench_fsm_psuedovec_6(b: &mut Bencher) {
        b.iter(|| fsm_mode_psuedo(tests[6]));
    }





    #[bench]
    fn bench_regex_0(b: &mut Bencher) {
        b.iter(|| regex_mode(tests[0]));
    }

    #[bench]
    fn bench_regex_1(b: &mut Bencher) {
        b.iter(|| regex_mode(tests[1]));
    }

    #[bench]
    fn bench_regex_2(b: &mut Bencher) {
        b.iter(|| regex_mode(tests[2]));
    }

    #[bench]
    fn bench_regex_3(b: &mut Bencher) {
        b.iter(|| regex_mode(tests[3]));
    }

    #[bench]
    fn bench_regex_4(b: &mut Bencher) {
        b.iter(|| regex_mode(tests[4]));
    }

    #[bench]
    fn bench_regex_5(b: &mut Bencher) {
        b.iter(|| regex_mode(tests[5]));
    }

    #[bench]
    fn bench_regex_6(b: &mut Bencher) {
        b.iter(|| regex_mode(tests[6]));
    }
}
