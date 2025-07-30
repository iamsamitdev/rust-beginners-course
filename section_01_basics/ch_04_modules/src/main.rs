mod meetings;
mod greetings;

// use greetings::morning;
// use greetings::evening;

// use greetings::{
//     morning,
//     evening,
// };

use greetings::morning::*;
use greetings::evening::*;
use greetings::special;

fn main() {

    meetings::hello();
    meetings::goodbye();

    good_morning();
    good_evening();
    special::holiday::happy_new_year();
}
