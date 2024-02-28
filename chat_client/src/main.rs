use utils::time::Time;

fn main() {
    utils::init();
    let mut now = Time::now_auto_offset();
    println!("{}", now);
    now.set_detailed_display(true);
    println!("{}", now);
}
