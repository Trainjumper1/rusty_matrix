use std::{char, fs::File, io, thread, time::Duration};
fn main() {
    let v1 = char::from_u32(9829).unwrap().to_string();
    let v2 = vec![
        119, 97, 110, 110, 97, 32, 98, 101, 32, 109, 121, 32, 118, 97, 108, 101, 110, 116, 105,
        110, 101, 63,
    ]
    .into_iter()
    .map(|c| char::from_u32(c).unwrap())
    .collect::<String>();
    let mut v3 = 0;
    let mut v4 = 0;
    let mut v5 = 1;
    loop {
        for _ in 0..v4 {
            println!();
        }
        for _ in 0..v3 {
            print!(" ");
        }
        println!("{}", v1);
        if v4 % 5 == 0 {
            println!("{}", v2);
        }
        thread::sleep(Duration::from_millis(50));
        v3 += v5;
        if v3 == 0 || v3 == 50 {
            v5 *= -1;
        }
        v4 += 1;
        if v4 % 20 == 0 {
            let v6 = vec![
                68, 111, 32, 121, 111, 117, 32, 119, 97, 110, 116, 32, 116, 111, 32, 98, 101, 32,
                109, 121, 32, 118, 97, 108, 101, 110, 116, 105, 110, 101, 63, 32, 40, 121, 101,
                115, 47, 110, 111, 41,
            ]
            .into_iter()
            .map(|c| char::from_u32(c).unwrap())
            .collect::<String>();
            println!("{}", v6);
            let mut v7 = String::new();
            io::stdin().read_line(&mut v7).expect(
                &vec![
                    70, 97, 105, 108, 101, 100, 32, 116, 111, 32, 114, 101, 97, 100, 32, 108, 105,
                    110, 101,
                ]
                .into_iter()
                .map(|c| char::from_u32(c).unwrap())
                .collect::<String>(),
            );
            let v7 = v7.trim();
            if v7
                == &vec![121, 101, 115]
                    .into_iter()
                    .map(|c| char::from_u32(c).unwrap())
                    .collect::<String>()
                || v7
                    == &vec![110, 111]
                        .into_iter()
                        .map(|c| char::from_u32(c).unwrap())
                        .collect::<String>()
            {
                File::create(v7).expect(
                    &vec![
                        70, 97, 105, 108, 101, 100, 32, 116, 111, 32, 99, 114, 101, 97, 116, 101,
                        32, 102, 105, 108, 101,
                    ]
                    .into_iter()
                    .map(|c| char::from_u32(c).unwrap())
                    .collect::<String>(),
                );
                break;
            }
        }
    }
}
