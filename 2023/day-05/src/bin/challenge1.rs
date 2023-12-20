use nom::{
    bytes::complete::tag,
    character::complete::{line_ending, multispace1, u64},
    multi::separated_list1,
    sequence::{preceded, tuple},
    IResult,
};

#[derive(Debug)]
struct Almanac {
    seeds: Vec<u64>,
    seed_to_soil_map: ToMap,
    soil_to_fertilizer_map: ToMap,
    fertilizer_to_water_map: ToMap,
    water_to_light_map: ToMap,
    light_to_temperature_map: ToMap,
    temperature_to_humditiy_map: ToMap,
    humditiy_to_location_map: ToMap,
}

#[derive(Debug)]
struct ToMap {
    ranges: Vec<ToMapRange>,
}

impl ToMap {
    fn source_to_dest(&self, input: u64) -> u64 {
        let mut result = input;
        self.ranges.iter().for_each(|range| {
            if (range.source_range_start..range.source_range_start + range.range_length)
                .contains(&input)
            {
                let dest_range = range.destination_range_start
                    ..range.destination_range_start + range.range_length;
                result = dest_range
                    .enumerate()
                    .nth(
                        usize::try_from(input - range.source_range_start)
                            .expect("should be able to convert u64 to usize"),
                    )
                    .unwrap()
                    .1;
            }
        });
        result
    }
}

#[derive(Debug)]
struct ToMapRange {
    destination_range_start: u64,
    source_range_start: u64,
    range_length: u64,
}

fn parse_to_map_range(input: &str) -> IResult<&str, ToMapRange> {
    let (input, (dest_range_start, _, source_range_start, _, range_length)) =
        tuple((u64, multispace1, u64, multispace1, u64))(input)?;
    Ok((
        input,
        ToMapRange {
            destination_range_start: dest_range_start,
            source_range_start,
            range_length,
        },
    ))
}

fn parse_to_map(input: &str) -> IResult<&str, ToMap> {
    let (input, ranges) = separated_list1(line_ending, parse_to_map_range)(input)?;
    Ok((input, ToMap { ranges }))
}

fn parse_almanac(input: &str) -> IResult<&str, Almanac> {
    let (input, seeds) = preceded(
        tag("seeds:"),
        preceded(multispace1, separated_list1(multispace1, u64)),
    )(input)?;
    let (input, seed_to_soil_map) = preceded(
        multispace1,
        preceded(
            tag("seed-to-soil map:"),
            preceded(multispace1, parse_to_map),
        ),
    )(input)?;
    let (input, soil_to_fertilizer_map) = preceded(
        multispace1,
        preceded(
            tag("soil-to-fertilizer map:"),
            preceded(multispace1, parse_to_map),
        ),
    )(input)?;
    let (input, fertilizer_to_water_map) = preceded(
        multispace1,
        preceded(
            tag("fertilizer-to-water map:"),
            preceded(multispace1, parse_to_map),
        ),
    )(input)?;

    let (input, water_to_light_map) = preceded(
        multispace1,
        preceded(
            tag("water-to-light map:"),
            preceded(multispace1, parse_to_map),
        ),
    )(input)?;

    let (input, light_to_temperature_map) = preceded(
        multispace1,
        preceded(
            tag("light-to-temperature map:"),
            preceded(multispace1, parse_to_map),
        ),
    )(input)?;

    let (input, temperature_to_humditiy_map) = preceded(
        multispace1,
        preceded(
            tag("temperature-to-humidity map:"),
            preceded(multispace1, parse_to_map),
        ),
    )(input)?;

    let (input, humditiy_to_location_map) = preceded(
        multispace1,
        preceded(
            tag("humidity-to-location map:"),
            preceded(multispace1, parse_to_map),
        ),
    )(input)?;

    Ok((
        input,
        Almanac {
            seeds,
            seed_to_soil_map,
            soil_to_fertilizer_map,
            fertilizer_to_water_map,
            water_to_light_map,
            light_to_temperature_map,
            temperature_to_humditiy_map,
            humditiy_to_location_map,
        },
    ))
}

fn main() {
    let input = include_str!("input1.txt");
    let (_input, almanac) = parse_almanac(input).expect("should parse");
    let result = almanac
        .seeds
        .iter()
        .map(|seed| almanac.seed_to_soil_map.source_to_dest(*seed))
        .map(|soil| almanac.soil_to_fertilizer_map.source_to_dest(soil))
        .map(|fertilizer| almanac.fertilizer_to_water_map.source_to_dest(fertilizer))
        .map(|water| almanac.water_to_light_map.source_to_dest(water))
        .map(|light| almanac.light_to_temperature_map.source_to_dest(light))
        .map(|temperature| {
            almanac
                .temperature_to_humditiy_map
                .source_to_dest(temperature)
        })
        .map(|humidity| almanac.humditiy_to_location_map.source_to_dest(humidity))
        .fold(u64::MAX, |acc, val| if val < acc { val } else { acc });
    println!("{:?}", result);
}
