use nom::{
    bytes::complete::tag,
    character::complete::{digit1, space1},
    combinator::map_res,
    multi::separated_list0,
    sequence::tuple,
    IResult,
};

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct Scratchcard {
    pub number: u32,
    pub winning: Vec<u32>,
    pub have: Vec<u32>,
}

// Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
pub fn parse_card(input: &str) -> IResult<&str, Scratchcard> {
    map_res(
        tuple((
            tag("Card"),
            space1,
            map_res(digit1, str::parse::<u32>),
            tag(":"),
            space1,
            separated_list0(space1, map_res(digit1, str::parse::<u32>)),
            tag(" |"),
            space1,
            separated_list0(space1, map_res(digit1, str::parse::<u32>)),
        )),
        |parsed| -> Result<Scratchcard, ()> {
            Ok(Scratchcard {
                number: parsed.2,
                winning: parsed.5,
                have: parsed.8,
            })
        },
    )(input)
}
