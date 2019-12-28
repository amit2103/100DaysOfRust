use chrono::prelude::*;
use chrono::{DateTime, Duration, Utc};
fn main() {
    let utc: DateTime<Utc> = Utc::now();   
    println!("Today day is {} ",utc.day()) ;
    println!("Today month is {} ",utc.month()) ;
    println!("Today Year is {} ",utc.year()) ;



    let chirstmas : DateTime<Utc>= Utc.ymd(2019, 12, 25).and_hms(23, 0, 0);
    let days_left = chirstmas.signed_duration_since(utc).num_days();
    
    if days_left == 0 {
        println!("Merry Chirstmas");
    } else if days_left < 0 {
        println!("Chirstmas is over for 2019, have a wonderful 2020")
    } else {
        println!("Sorry there are still {} left to Chirstmas" , days_left)
    }

    println!("-------------------DURATION EXERCISES----------------");

    test_duartion();

}

fn test_duartion() {
    let duration = Duration::days(4) + Duration::hours(10);
    println!("Number of Days in duration {} ",duration.num_days());
    println!("Number of hours in duration {} ",duration.num_hours());
    println!("Number of seconds in duration {} ",duration.num_seconds());

    let today: DateTime<Utc> = Utc::now(); 

    println!("Today  {} ", today);
    
    let eta = Duration::hours(6);

    let today = today.checked_add_signed(eta);

    match today {
        Some(x) => println!("Expected time of arrival is {}", x),
        None => eprintln!("Error in adding ETA"),
    }
}

