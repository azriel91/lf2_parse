use std::convert::TryFrom;

use pest::Parser;

use lf2_parse::{Error, ObjectData, ObjectDataParser, Rule};

fn run() -> Result<(), Error<'static>> {
    let mut object_data_pairs = ObjectDataParser::parse(
        Rule::Object,
        r#"<bmp_begin>
name: Frozen
head: frozen\frozen_f.bmp
small: frozen\frozen_s.bmp
file(0-69): frozen\frozen_0.bmp  w: 79  h: 79  row: 10  col: 7
file(70-139): frozen\frozen_1.bmp  w: 79  h: 79  row: 10  col: 7
file(140-209): frozen\frozen_2.bmp  w: 79  h: 79  row: 10  col: 7
walking_frame_rate 3
walking_speed 5.000000
walking_speedz 2.570000
running_frame_rate 3
running_speed 11.000000
running_speedz 1.670000
heavy_walking_speed 3.700000
heavy_walking_speedz 1.900000
heavy_running_speed 7.000000
heavy_running_speedz 1.200000
jump_height -16.000000
jump_distance 10.000000
jump_distancez 3.750000
dash_height -11.500000
dash_distance 19.000000
dash_distancez 5.000000
rowing_height -2.000000
rowing_distance 6.000000
<bmp_end>"#,
    )?;

    object_data_pairs.try_for_each(|pair| {
        println!("{:?}", pair.as_rule());

        match pair.as_rule() {
            Rule::Object => {
                let object_data = ObjectData::try_from(pair)?;
                println!("{:?}", object_data);

                Ok(())
            }
            _ => Ok(()),
        }
    })
}

fn main() -> Result<(), Error<'static>> {
    if let Err(e) = run() {
        eprintln!("{}", e);
    }
    Ok(())
}
