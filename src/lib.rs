

//This vectorizes testing for equality across an array
macro_rules! vec_eq {
    ($a:expr, $b:expr) => (true);
    ($a:expr, $b:expr, $curr:expr) => ( ($a[$curr]==$b[$curr]) & vec_eq!($a,$b) );
    ($a:expr, $b:expr, $curr:expr $(, $tail:expr)*) => ( ($a[$curr]==$b[$curr]) & vec_eq!($a,$b,$($tail),*) );
}
#[test]
fn test_vectorize() {

    //all index's
    let test_0: [usize;10] = [0,5,10,15,20,25,30,35,40,45];
    let test_1: [usize;10] = [0,5,10,15,20,25,30,35,40,45];
    let x = vec_eq!(test_0,test_1,0,1,2,3,4,5,6,7,8,9);
    assert!( x );

    //skip index 5
    let test_0: [usize;10] = [0,5,10,15,20,2000,30,35,40,45];
    let test_1: [usize;10] = [0,5,10,15,20,25,30,35,40,45];
    let x = vec_eq!(test_0,test_1,0,1,2,3,4,6,7,8,9);
    assert!( x );

    //include index 5
    let test_0: [usize;10] = [0,5,10,15,20,2000,30,35,40,45];
    let test_1: [usize;10] = [0,5,10,15,20,25,30,35,40,45];
    let x = vec_eq!(test_0,test_1,0,1,2,3,4,5,6,7,8,9);
    assert!( ! x );
}


//collect debug information
macro_rules! collect_fmt {
    ($($b:expr),+) => {{
        let mut v = Vec::new();
        $(
            let s = format!("{:?}", $b );
            v.push( s );
        )+
        v
    }}
}


//handle adding things to the text
macro_rules! append {
    ($out_buffer:expr,$args:expr,$index:expr) => {{
        if $args.len() == $index {
            let x = format!("Out of range on %{}", $index );
            $out_buffer.push_str(&x);
        } else {
            $out_buffer.push_str( &$args[$index] );
        }
    }}
}


macro_rules! scand {
    ($a:expr, $($b:expr),+) => {{
        let args = collect_fmt!($($b),+);
        let mut out = String::with_capacity(4000);
        let mut char_ = $a.chars();
        loop {
            match char_.next() {
                Option::None => break,
                Option::Some('%') => match char_.next() {
                    Option::None => {
                        let x = format!("Illegal escape sequence. % followed by EOL");
                        out.push_str(&x);
                        break;
                    },
                    Option::Some('0') => append!(out, args, 0),
                    Option::Some('1') => append!(out, args, 1),
                    Option::Some('2') => append!(out, args, 2),
                    Option::Some('3') => append!(out, args, 3),
                    Option::Some('4') => append!(out, args, 4),
                    Option::Some('5') => append!(out, args, 5),
                    Option::Some('6') => append!(out, args, 6),
                    Option::Some('7') => append!(out, args, 7),
                    Option::Some('8') => append!(out, args, 8),
                    Option::Some('9') => append!(out, args, 9),
                    Option::Some('%') => out.push('%'),
                    Option::Some(x) => {
                        let x  = format!("Illegal escape sequence: \"%{}\". Please use values 0-9", x);
                        out.push_str(&x);
                        break;
                    }
                },
                Option::Some(x) => out.push(x)
            };
        }
        out
    }};
}

#[test]
fn test_scand() {

    let x = scand!("Complex Value: %0+%1i", 15usize, 10.2f32);
    assert_eq!( x, "Complex Value: 15+10.2i" );
}
