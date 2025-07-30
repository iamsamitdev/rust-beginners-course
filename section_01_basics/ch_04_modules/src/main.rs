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

    // greetings::morning::good_morning();
    // greetings::evening::good_evening();

    // morning::good_morning();
    // evening::good_evening();

    // morning::good_morning();
    // evening::good_evening();

    good_morning();
    good_evening();
    special::holiday::happy_new_year();
}
