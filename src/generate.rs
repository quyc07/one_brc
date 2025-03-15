use std::{
    collections::HashMap, fs::File, io::{BufWriter, Write}, sync::LazyLock
};

use rand::{seq::IteratorRandom, Rng};

use crate::FILE_PATH;

static  CITY: LazyLock<HashMap<&str, f32>> = LazyLock::new(|| {
    let mut city = HashMap::new();
    city.insert("beijing", 10.0);
    city.insert("shanghai", 13.0);
    city.insert("guangzhou", 20.0);
    city.insert("shenzhen", 21.0);
    city.insert("wuhan", 15.0);
    city.insert("chengdu", 14.0);
    city.insert("hangzhou", 16.0);
    city.insert("nanjing", 16.0);
    city.insert("shenyang", 4.0);
    city.insert("xian", 13.0);
    city.insert("tianjin", 11.0);
    city.insert("zhengzhou", 13.0);
    city.insert("chongqing", 15.0);
    city.insert("heilongjiang", 3.0);
    city.insert("lasa", 7.0);
    city
});

pub(crate) fn generate_data(){
    let file = File::create(FILE_PATH).expect("fail to open file");
    let mut writer = BufWriter::with_capacity(1024 * 1024, file);

    let mut rng = rand::thread_rng();

    for _ in 0..1_000_000_000 {
        if let Some((&city, &value)) = CITY.iter().choose(&mut rng) {
            let temperature = rng.gen_range(value - 10.0..value + 10.0);
            let msg = format!("{city};{:.2}\n",temperature);
            writer.write_all(msg.as_bytes()).unwrap();
        }
    }
}
