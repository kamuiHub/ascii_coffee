use std::process::Command;
use std::thread;
use std::time;
fn main() {
    let mut steam = [
        [0, 1, 0, 0, 1, 0, 0, 1],
        [1, 0, 0, 0, 1, 0, 1, 0],
        [1, 0, 0, 1, 0, 0, 1, 0],
        [0, 1, 0, 1, 0, 0, 0, 1],
        [0, 1, 0, 0, 1, 0, 0, 1],
    ];

    let mut iter = 1;
    loop {
        clear_terminal_screen();
        print_steam(&steam);
        println!(
            "
           ██████████████████████
           ██                  ██
           ██                  ███████
           ██                  ██   ██
           ██                  ███████
             ██              ██
        █████████████████████████████
        ██                         ██
          █████████████████████████
        "
        );

        steam = animation(steam, iter);
        iter += 1;
        thread::sleep(time::Duration::from_millis(650));
    }
}

fn animation(steam: [[i32; 8]; 5], iter: u8) -> [[i32; 8]; 5] {
    if iter % 2 != 0 {
        return switch_odd(steam);
    } else {
        return switch_couples(steam);
    }
}

fn switch_odd(mut steam: [[i32; 8]; 5]) -> [[i32; 8]; 5] {
    for row in 0..steam.len() {
        if row % 2 == 0 {
            for val in 0..steam[row].len() {
                if val != 2 && val != 5 {
                    if steam[row][val] == 0 {
                        steam[row][val] = 1;
                    } else {
                        steam[row][val] = 0;
                    }
                }
            }
        }
    }
    steam
}

fn switch_couples(mut steam: [[i32; 8]; 5]) -> [[i32; 8]; 5] {
    for row in 0..steam.len() {
        if row % 2 != 0 {
            for val in 0..steam[row].len() {
                if val != 2 && val != 5 {
                    if steam[row][val] == 0 {
                        steam[row][val] = 1;
                    } else {
                        steam[row][val] = 0;
                    }
                }
            }
        }
    }
    steam
}

fn print_steam(steam: &[[i32; 8]; 5]) {
    for row in steam.iter() {
        print!("               ");
        for e in row.iter() {
            if *e == 0 {
                print!("  ");
            } else {
                print!("█");
            }
        }
        println!();
    }
}

pub fn clear_terminal_screen() {
    if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(["/c", "cls"])
            .spawn()
            .expect("cls command failed to start")
            .wait()
            .expect("failed to wait");
    } else {
        Command::new("clear")
            .spawn()
            .expect("clear command failed to start")
            .wait()
            .expect("failed to wait");
    };
}
