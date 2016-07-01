
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

#[inline]
fn append_arg(s: &mut String, args: &Vec<String>, index: usize ) {
    if args.len() == index {
        let x = format!("Out of range on %{}", index );
        s.push_str(&x);
    } else {
        s.push_str( &args[index] );
    }
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
                    Option::Some('0') => append_arg(&mut out, &args, 0),
                    Option::Some('1') => append_arg(&mut out, &args, 1),
                    Option::Some('2') => append_arg(&mut out, &args, 2),
                    Option::Some('3') => append_arg(&mut out, &args, 3),
                    Option::Some('4') => append_arg(&mut out, &args, 4),
                    Option::Some('5') => append_arg(&mut out, &args, 5),
                    Option::Some('6') => append_arg(&mut out, &args, 6),
                    Option::Some('7') => append_arg(&mut out, &args, 7),
                    Option::Some('8') => append_arg(&mut out, &args, 8),
                    Option::Some('9') => append_arg(&mut out, &args, 9),
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
