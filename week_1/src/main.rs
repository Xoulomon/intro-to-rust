fn main() {
    println!("Hello, world!");


    // let c = 100;
    // let f = cel_to_fah(c);
    // println!("The fahrenheit conversion of {c} is {f}");


    // let fah = 212;
    // let cel = fah_to_cel(fah);
    // println!("The celcius conversion of {fah} is {cel}");

    // let fib = 10;
    // println!("The fibonacci sequence of {fib}");
    // fib_seq(fib);

    xmas_song()
}

fn fah_to_cel(fah:i32) -> i32{
    (fah - 32) * 5 / 9
}

fn cel_to_fah(cel:i32)-> i32{
    (cel * 9 / 5) + 32
}

fn fib_seq(val:u8) {
    let mut a = 0;
    let mut b = 1;

    for _i in 0..val {
        print!("{a} ");
        let next = a + b;
        a=b;
        b=next;
    }
    println!(" ");
}

fn xmas_song() {
    let position = ["First", "Second", "Third", "Fourth", "Fifth", "Sixth", "Seventh", "Eighth", "Ninth", "Tenth", "Eleventh", "Twelfth"];
    let lyrics = ["A partridge in a pear tree", "Two turtle doves and", "Three french hens", "Four calling birds", "Five golden rings", "Six geese a-laying", "Seven swans a-swimming", "Eight maids a-milking", "Nine ladies dancing", "Ten lords a-leaping", "Eleven pipers piping", "Twelve drummers drumming"];

    let len = position.len();

    for i in 0..len {
        let pos = position[i];
        println!("On the {pos} of christmas, my true love sent to me");

        for j in (0..i).rev() {
            let lyric = lyrics[j+1];
            println!("{lyric}");
        }

    let first_lyric = lyrics[0];
    if i>0 {
        println!("and {first_lyric}");
    } else {
        println!("{first_lyric}");       
    }
    println!("  ");        
    }

}