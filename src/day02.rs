mod parser {
    use nom::{
        branch::permutation,
        bytes::complete::{tag, take_while1, take_while_m_n},
        combinator::{map_res, opt},
        multi::separated_list1,
        sequence::{pair, terminated, tuple},
        IResult,
    };
    use std::str::FromStr;

    #[derive(Debug, PartialEq)]
    pub struct Game {
        pub id: usize,
        pub rounds: Vec<Round>,
    }

    #[derive(Debug, PartialEq)]
    pub struct Round {
        pub red: usize,
        pub green: usize,
        pub blue: usize,
    }

    #[derive(Debug, PartialEq)]
    pub enum Color {
        Red(usize),
        Green(usize),
        Blue(usize)
    }

    fn number(input: &str) -> IResult<&str, usize> {
        map_res(take_while1(is_numeric), from_number)(input)
    }

    fn is_numeric(c: char) -> bool {
        c.is_numeric()
    }

    fn from_number(input: &str) -> Result<usize, std::num::ParseIntError> {
        usize::from_str(input)
    }

    pub fn games(input: &str) -> IResult<&str, Vec<Game>> {
        separated_list1(tag("\n"), game)(input)
    }

    fn game(input: &str) -> IResult<&str, Game> {
        let (input, _) = tag("Game ")(input)?;
        let (input, id) = number(input)?;
        let (input, _) = tag(": ")(input)?;
        let (input, rounds) = separated_list1(tag("; "), round)(input)?;
        Ok((input, Game { id, rounds }))
    }

    fn round(input: &str) -> IResult<&str, Round> {
        let (input, (red, blue, green)) = permutation!((opt(red), opt(blue), opt(green)))(input)?;
        Ok((
            input,
            Round {
                red: red.unwrap_or(0),
                blue: blue.unwrap_or(0),
                green: green.unwrap_or(0),
            },
        ))
    }

    fn red(input: &str) -> IResult<&str, Color> {
        map_res(
            terminated(number, pair(tag(" red"), opt(tag(", ")))),
            |i| Color::Red(i)
        )(input)
    }

    fn blue(input: &str) -> IResult<&str, usize> {
        terminated(number, pair(tag(" blue"), opt(tag(", "))))(input)
    }

    fn green(input: &str) -> IResult<&str, usize> {
        terminated(number, pair(tag(" green"), opt(tag(", "))))(input)
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_game() {
            let text = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
            assert_eq!(
                game(text),
                Ok((
                    &""[..],
                    Game {
                        id: 1,
                        rounds: vec![
                            Round {
                                red: 4,
                                green: 0,
                                blue: 3
                            },
                            Round {
                                red: 1,
                                green: 2,
                                blue: 6
                            },
                            Round {
                                red: 0,
                                green: 2,
                                blue: 0
                            },
                        ]
                    }
                ))
            );
        }

        #[test]
        fn test_round() {
            let text = "1 red, 2 green, 6 blue";
            assert_eq!(
                round(text),
                Ok((
                    &""[..],
                    Round {
                        red: 1,
                        green: 2,
                        blue: 6
                    },
                ))
            );
            let text = "3 blue, 4 red";
            assert_eq!(
                round(text),
                Ok((
                    &""[..],
                    Round {
                        red: 4,
                        green: 0,
                        blue: 3
                    },
                ))
            );
            let text = "2 green";
            assert_eq!(
                round(text),
                Ok((
                    &""[..],
                    Round {
                        red: 0,
                        green: 2,
                        blue: 0
                    },
                ))
            );
        }

        #[test]
        fn test_blue() {
            let text = "6 blue, ";
            assert_eq!(
                blue(text),
                Ok((
                    &""[..],
                    6
                ))
            );

            let text = "6 blue";
            assert_eq!(
                blue(text),
                Ok((
                    &""[..],
                    6
                ))
            );
        }
    }
}

mod part1 {
    use super::parser;

    pub fn parse(text: &str) -> usize {
        let (_rest, games) = parser::games(text).unwrap();
        games
            .iter()
            .filter(|game| {
                game.rounds
                    .iter()
                    .all(|round| round.red <= 12 && round.green <= 13 && round.blue <= 14)
            })
            .map(|game| game.id)
            .sum()
    }

    #[cfg(test)]
    mod tests {
        use crate::day02::parser::{self, Game, Round};

        use super::parse;

        #[test]
        fn example() {
            let text = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
            Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
            Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
            Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
            Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
            assert_eq!(parse(text), 8);
        }

        #[test]
        fn input() {
            assert_eq!(parse(include_str!("../input/day02.txt")), 54159);
        }
    }
}
