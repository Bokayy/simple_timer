use std::time::{Duration, SystemTime, Instant};
use std::thread::sleep;
use std::io;

fn main() {

    let now = Instant::now();
   let mut is_counting = false;
   let mut input_string = String::new();
   let mut old_value : String = String::new();

/* loop{
        dbg!(now.elapsed());
        print!("{}[2J", 27 as char); //clear console
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    } */
/*    let now = SystemTime::now();
   loop{
    match now.elapsed(){
        Ok(elapsed) => {
            print!("{}", elapsed.as_secs());
            print!("{}[2J", 27 as char); //clear console
            print!("{esc}[2J{esc}[1;1H", esc = 27 as char); //place cursor row 1 column 1
        }
        Err(e) => {
            print!("Error: {e:?}");
        }
    }
   } */

     loop{
        println!("press anything then enter to start counting, press anything then enter to stop.");
        io::stdin()
            .read_line(&mut input_string)
            .expect("Failed to read line");

        if input_string != old_value{
            println!("Value changed");
            is_counting = !is_counting;
        }
        old_value = input_string.clone();
        input_string = String::new();
        dbg!(is_counting);

        if is_counting{
            loop{
                dbg!(now.elapsed());
                print!("{}[2J", 27 as char); //clear console
                print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
            }
    }
    
    }
}

   

/*    if is_counting{
    match now.elapsed() {
        Ok(elapsed) => {
            // it prints '2'
            println!("{}", elapsed.as_secs());
            print!("{}[2J", 27 as char);
        }
        Err(e) => {
            // an error occurred!
            println!("Error: {e:?}");
        }
    }
    }
}
*/