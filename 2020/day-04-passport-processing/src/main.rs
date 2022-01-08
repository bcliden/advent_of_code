#![allow(dead_code)]

use std::ops::RangeInclusive;

#[derive(Clone, Copy, PartialEq, Debug)]
struct Year(u64);

#[derive(Clone, Copy, PartialEq, Debug)]
enum Length {
    /// Centimeters (correct)
    Cm(u64),
    /// Inches (incorrect)
    In(u64),
}

#[derive(Clone, Copy, PartialEq, Debug)]
struct Color<'a>(&'a str);

/// An identifier  (pid or cid)
#[derive(Clone, Copy, PartialEq, Debug)]
struct ID<'a>(&'a str);

#[derive(PartialEq, Debug)]
struct Passport<'a> {
    birth_year: Year,
    issue_year: Year,
    expiration_year: Year,
    height: Length,
    hair_color: Color<'a>,
    eye_color: Color<'a>,
    passport_id: ID<'a>,
    country_id: Option<ID<'a>>,
}

#[derive(PartialEq, Debug, Default)]
struct PassportBuilder<'a> {
    birth_year: Option<Year>,
    issue_year: Option<Year>,
    expiration_year: Option<Year>,
    height: Option<Length>,
    hair_color: Option<Color<'a>>,
    eye_color: Option<Color<'a>>,
    passport_id: Option<ID<'a>>,
    country_id: Option<ID<'a>>,
}

#[derive(thiserror::Error, Debug)]
enum Error {
    #[error("missing field: {0}")]
    MissingField(&'static str),

    #[error("could not parse {0}: {1}")]
    ParseError(String, String),
}

impl<'a> PassportBuilder<'a> {
    fn build(self) -> Result<Passport<'a>, Error> {
        // // Verbose first pass...
        // Ok(Passport {
        //     birth_year: self.birth_year.ok_or(Error::MissingField("birth_year"))?,
        //     issue_year: self.issue_year.ok_or(Error::MissingField("issue year"))?,
        //     expiration_year: self
        //         .expiration_year
        //         .ok_or(Error::MissingField("expiration_year"))?,
        //     height: self.height.ok_or(Error::MissingField("height"))?,
        //     hair_color: self.hair_color.ok_or(Error::MissingField("hair color"))?,
        //     eye_color: self.eye_color.ok_or(Error::MissingField("eye_color"))?,
        //     passport_id: self.passport_id.ok_or(Error::MissingField("passport id"))?,
        //     country_id: self.country_id,
        // })

        // // Macro second try (this syntax is _so_ overwhelming)
        macro_rules! build {
            (
                required => {
                    $($req: ident),* $(,)*
                }$(,)*
                optional => {
                    $($opt: ident),* $(,)*
                }$(,)*
            ) => {
                Ok(Passport {
                    $($req: self.$req.ok_or(Error::MissingField(stringify!($req)))?),*,
                    $($opt: self.$opt),*
                })
            }
        }

        build! {
            required => {
                birth_year,
                issue_year,
                expiration_year,
                height,
                hair_color,
                eye_color,
                passport_id,
            },
            optional => {
                country_id
            }
        }
    }

    fn parse(input: &'a str) -> Result<Self, Error> {
        let mut b: Self = Default::default();

        peg::parser! {
            grammar parser() for str {
                pub(crate) rule root(b: &mut PassportBuilder<'input>)
                    = (field(b) separator()*)* ![_]

                rule separator()
                    = ['\n' | '\r' | ' ']

                rule field(b: &mut PassportBuilder<'input>)
                    // years
                    = byr(b) / iyr(b) / eyr(b)
                    // height
                    / hgt(b)
                    // colors
                    / hcl(b) / ecl(b)
                    // IDs
                    / pid(b) / cid(b)

                rule byr(b: &mut PassportBuilder<'input>)
                    = "byr:" year:year((1920..=2002)) { b.birth_year = Some(year) }

                rule iyr(b: &mut PassportBuilder<'input>)
                    = "iyr:" year:year((2010..=2020)) { b.issue_year = Some(year) }

                rule eyr(b: &mut PassportBuilder<'input>)
                    = "eyr:" year:year((2020..=2030)) { b.expiration_year = Some(year) }

                rule hgt(b: &mut PassportBuilder<'input>)
                    = "hgt:" height:length() {?
                        match &height {
                            Length::Cm(v) if !(150..=193).contains(v) => {
                                Err("bad height (cm)")
                            },
                            Length::In(v) if !(59..=76).contains(v) => {
                                Err("bad height (in")
                            },
                            _ => {
                                b.height = Some(height);
                                Ok(())
                            }
                        }
                    }

                rule pid(b: &mut PassportBuilder<'input>)
                    = "pid:" id:$(['0'..='9']*<9,9>) { b.passport_id = Some(ID(id)) }

                rule cid(b: &mut PassportBuilder<'input>)
                    = "cid:" id:$((!separator()[_])+) { b.country_id = Some(ID(id)) }

                rule hcl(b: &mut PassportBuilder<'input>)
                    = "hcl:" color:hcl0() { b.hair_color = Some(color) }

                rule hcl0() -> Color<'input>
                    = s:$("#" ['0'..='9' | 'a'..='f']*<6,6>) { Color(s) }

                rule ecl(b: &mut PassportBuilder<'input>)
                    = "ecl:" color:ecl0() { b.eye_color = Some(color) }

                rule ecl0() -> Color<'input>
                    = s:$("amb" / "blu" / "brn" / "gry" / "grn" / "hzl" / "oth") { Color(s) }

                rule year(range: RangeInclusive<u64>) -> Year
                    = num:num() {?
                    if range.contains(&num) {
                        Ok(Year(num))
                    } else {
                        Err("year out of range")
                    }
                }

                rule length() -> Length
                    = num:num() "cm" { Length::Cm(num) }
                    / num:num() "in" { Length::In(num) }

                rule num() -> u64
                    = s:$(['0'..='9']+) { s.parse().unwrap() }

                rule color() -> Color<'input>
                    = s:$((!separator()[_])*) { Color(s) }
            }
        }

        parser::root(input, &mut b).map_err(|e| Error::ParseError(input.into(), e.to_string()))?;
        Ok(b)
    }
}

// // Made obsolete by parser changes for Pt. 2
// fn calc_part_one(s: &str) -> usize {
//     let results = s
//         .split("\n\n")
//         .map(PassportBuilder::parse)
//         .map(PassportBuilder::build);
//     dbg!(&results);
//     results.filter(Result::is_ok).count()
// }

fn calc_part_two(s: &str) -> usize {
    let results = s
        .split("\n\n")
        .map(|input| PassportBuilder::parse(input).and_then(|b| b.build()));

    results.filter(Result::is_ok).count()
}

fn main() {
    // dbg!(FULL);
    // let num_valid = calc_part_one(FULL);
    // println!("Found {}", num_valid);

    let num_valid = calc_part_two(FULL);
    println!("Found {}", num_valid);
}

const EXAMPLE: &str = r#"ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in"#;

const FULL: &str = include_str!("input.txt");

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn part_one_example() {
    //     let num_valid = calc_part_one(EXAMPLE);
    //     assert_eq!(num_valid, 2)
    // }

    // #[test]
    // fn part_one_full() {
    //     let num_valid = calc_part_one(FULL);
    //     assert_eq!(num_valid, 200)
    // }

    #[test]
    fn part_two_full() {
        let num_valid = calc_part_two(FULL);
        assert_eq!(num_valid, 116)
    }

    #[test]
    fn test_builder() {
        assert!(PassportBuilder {
            ..Default::default()
        }
        .build()
        .is_err());
        assert!(PassportBuilder {
            birth_year: Some(Year(2014)),
            issue_year: Some(Year(2017)),
            expiration_year: Some(Year(2023)),
            height: Some(Length::Cm(195)),
            hair_color: Some(Color("#ffffff")),
            eye_color: Some(Color("#ee7812")),
            passport_id: Some(ID("00023437")),
            country_id: None,
        }
        .build()
        .is_ok());
    }
}
