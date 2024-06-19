// Print the lyrics to the Christmas carol 
// “The Twelve Days of Christmas,” taking advantage of 
// the repetition in the song.

fn main() {
    // the main lyric lines
    let lyric = [
        "and a partridge in a pear tree!",
        "Two turtle doves,",
        "Three French hens,",
        "Four calling birds,",
        "Five golden rings,",
        "Six geese a-laying,",
        "Seven swans a-swimming,",
        "Eight maids a-milking,",
        "Nine ladies dancing,",
        "Ten lords a-leaping,",
        "Eleven pipers piping,",
        "Twelve drummers drumming,"
        ];
    // the written out days
    let day = [
        "first",
        "second",
        "third",
        "fourth",
        "fifth",
        "sixth",
        "seventh",
        "eighth",
        "ninth",
        "tenth",
        "eleventh",
        "twelfth",
        ];
    // loop through the day count for each round
    for i in 0..12{
        println!("On the {} day of Christmas,", day[i]);
        println!("my true love gave to me,");
        let first_round = &lyric[0][4..]; // make a slice to cut out 
                                          // "and a" on the first round
        for j in 0..=i{
            if i == 0{
                println!("{}", first_round);
            }else{
                // count backward through the array
                println!("{}", lyric[lyric.len()-1-j]);
            }
        }
        // print a new line between each round except last
        if i != 11 {
            println!("");
        }
    }

}
