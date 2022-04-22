use colored::*;
use rand::Rng;
fn main() {
    let random: u32 = rand::thread_rng().gen_range(1..=2);
    if random < 2 {
      println!("{} {} 👍","The sistem is:".bold(), "Rial".green());
      println!("ريال وهمي");
      println!("يا إلهي، نظامك حقيقي.");
    }else {
      println!("{} {} 👎","The sistem is:".bold(), "Fake".red());
      println!("ريال وهمي");
      println!("يا إلهي ، نظامك مزيف");
    }
}
