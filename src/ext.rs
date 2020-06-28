
//TODO: implement more smart algorithm:D:
pub fn get_color_from_number(number: u64) -> u64 {
    number & 0xFFFFFF
}

pub fn pretty_time_print(time: u64) -> String {
    let mut minutes = time / 60;
    let seconds = time % 60;
    let hours = minutes / 60;
    minutes = minutes % 60;
    format!("{}h. {}m. {}s.", hours, minutes, seconds)
}
