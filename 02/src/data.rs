use nom::{
    bytes::complete::tag,
    character::complete::{digit1, space0, space1},
    combinator::map_res,
    multi::separated_list0,
    sequence::tuple,
    IResult,
};

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct CubeSet {
    pub n_red: u32,
    pub n_green: u32,
    pub n_blue: u32,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct Game {
    pub id: u32,
    pub reveals: Vec<CubeSet>,
}

impl CubeSet {
    pub fn new() -> CubeSet {
        CubeSet {
            n_red: 0,
            n_green: 0,
            n_blue: 0,
        }
    }
}

/// "3 blue, 4 red"
fn parse_cube_set(input: &str) -> IResult<&str, CubeSet> {
    map_res(
        separated_list0(
            tuple((tag(","), space0)),
            tuple((
                map_res(digit1, str::parse::<u32>),
                space1,
                nom::branch::alt((tag("red"), tag("green"), tag("blue"))),
            )),
        ),
        |l| -> Result<CubeSet, _> {
            let mut result = CubeSet {
                n_red: 0,
                n_green: 0,
                n_blue: 0,
            };
            for color in l {
                match color.2 {
                    "red" => result.n_red += color.0,
                    "green" => result.n_green += color.0,
                    "blue" => result.n_blue += color.0,
                    _ => return Err("unknown color"),
                }
            }
            Ok(result)
        },
    )(input)
}

/// Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
pub fn parse_game(input: &str) -> IResult<&str, Game> {
    map_res(
        tuple((
            tag("Game "),
            map_res(digit1, str::parse::<u32>),
            tag(": "),
            separated_list0(tuple((tag(";"), space0)), parse_cube_set),
        )),
        |parsed| -> Result<Game, &str> {
            Ok(Game {
                id: parsed.1,
                reveals: parsed.3,
            })
        },
    )(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_cube_set() {
        assert_eq!(
            parse_cube_set("3 blue, 4 red"),
            Ok((
                "",
                CubeSet {
                    n_red: 4,
                    n_green: 0,
                    n_blue: 3
                }
            ))
        );
        assert_eq!(
            parse_cube_set("1 blue"),
            Ok((
                "",
                CubeSet {
                    n_red: 0,
                    n_green: 0,
                    n_blue: 1
                }
            ))
        );
        assert_eq!(
            parse_cube_set("1 blue, 2 red, 3 green"),
            Ok((
                "",
                CubeSet {
                    n_red: 2,
                    n_green: 3,
                    n_blue: 1
                }
            ))
        );
    }

    #[test]
    fn test_parse_game() {
        assert_eq!(
            parse_game("Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"),
            Ok((
                "",
                Game {
                    id: 5,
                    reveals: vec![
                        CubeSet {
                            n_red: 6,
                            n_blue: 1,
                            n_green: 3
                        },
                        CubeSet {
                            n_red: 1,
                            n_blue: 2,
                            n_green: 2
                        }
                    ]
                }
            ))
        )
    }
}
