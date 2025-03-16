use std::{
    collections::HashMap,
    fs::File,
    io::{self, BufRead, BufReader},
};

use crate::FILE_PATH;

struct CityData {
    name: String,
    temperature: f32,
}

struct CityAgg {
    min: f32,
    max: f32,
    sum: f32,
    count: i32,
}

pub(crate) fn calc() -> io::Result<()> {
    let file = File::open(FILE_PATH)?;
    let reader = BufReader::with_capacity(1024 * 1024, file);
    let mut city_agg: HashMap<String, CityAgg> = HashMap::with_capacity(15);
    reader
        .lines()
        .map(|x| {
            let line = x.unwrap();
            let mut split = line.split(";");
            let name = split.next().unwrap();
            let temperature = split.next().unwrap().parse::<f32>().unwrap();
            return CityData {
                name: name.to_string(),
                temperature,
            };
        })
        .for_each(|city_data| {
            city_agg
                .entry(city_data.name.clone())
                .and_modify(|city_agg| {
                    city_agg.min = f32::min(city_agg.min, city_data.temperature);
                    city_agg.max = f32::max(city_agg.max, city_data.temperature);
                    city_agg.sum = city_agg.sum + city_data.temperature;
                    city_agg.count = city_agg.count + 1;
                })
                .or_insert(CityAgg {
                    min: city_data.temperature,
                    max: city_data.temperature,
                    sum: city_data.temperature,
                    count: 1,
                });
        });
    let mut list = city_agg
        .into_iter()
        .map(|(k, v)| (k, v))
        .collect::<Vec<(String, CityAgg)>>();
    list.sort_by_key(|(k, _)| k.clone());
    list.iter().for_each(|(name, city_agg)| {
        println!("{name}:{:.2}", city_agg.sum / city_agg.count as f32)
    });
    Ok(())
}
