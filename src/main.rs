extern crate winapi;

use winapi::um::winuser;
use winapi::shared::windef;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::time::Instant;
use winapi::um::winuser::{VK_CONTROL, VK_MENU, VK_RETURN, VK_SPACE};

const RESERVED_KEY: i32 = 0x5E; //represents a key that corresponds to a non-ascii value, shouldn't be listened to, or doesn't exist.
const STANDARD_QWERTY: &str ="STANDARD_QWERTY"; // The keyboard format given
const RESERVED_TUPLE: (i32, i32) = (RESERVED_KEY, RESERVED_KEY);
const PRESSED_STATES: [i16; 2] = [-32767, -32768];

fn main() {
    let layout= generate_keyboard_layout_table(STANDARD_QWERTY);
    let mut words: HashSet<String> = HashSet::new();

    load_words(&mut words);

    let mut input_list: Vec<(i32, (i32, i32))> = Vec::new();

    println!("Hello, world!");
    println!("{:?}", &layout);
    stealth();

    //main loop

    loop {
        for key in 9..162 {
            if unsafe { winuser::GetAsyncKeyState(key)} == -32767 {

                if key == VK_SPACE || key == VK_RETURN { // if spacebar or enter
                    //check for l/r matches
                    let mut word: String = "".to_string();
                    let mut l_word: String = "".to_string();
                    let mut r_word: String = "".to_string();

                    for i in input_list {
                        word = word + &*get_key_value(i.0);
                        l_word = l_word + &*get_key_value(i.1.0);
                        r_word = r_word + &*get_key_value(i.1.1);
                    }

                    // println!("Input list: {}", input_list)
                    println!("Input: {}", word);

                    let l_is_valid = words.contains(&l_word);
                    let c_is_valid = words.contains(&word);
                    let r_is_valid = word.contains(&r_word);

                    println!("Is word? {} | {} | {}", l_is_valid, c_is_valid, r_is_valid);

                    input_list = Vec::new()
                } else if
                    !PRESSED_STATES.contains(&(unsafe { winuser::GetAsyncKeyState(VK_CONTROL)}))  //
                    &&
                    !PRESSED_STATES.contains(&(unsafe {winuser::GetAsyncKeyState(VK_MENU)}))
                    { // otherwise, add input
                    // !PRESSED_STATES.contains(&(unsafe { winuser::GetAsyncKeyState(VK_CONTROL) as i32})) && !PRESSED_STATES.contains(&(unsafe {winuser::GetAsyncKeyState(VK_MENU)} as i32))
                    input_list.append(&mut vec![(key as i32, get_key_neighbors(&layout, &(key as i32)))])
                }
            }
        }
    }
}

fn get_key_neighbors(layout: &HashMap<i32, (i32, i32)>, i: &i32) -> (i32, i32) {
    *layout.get(&i).unwrap_or(&RESERVED_TUPLE)
}

fn generate_keyboard_layout_table(name: &str) -> HashMap<i32, (i32, i32)> {
    let layout_table = match name {
        STANDARD_QWERTY => vec![
            vec![vec![0x9], str_to_u8_vec("qwertyuiop"), vec![0xdb, 0xdd, 0xdc]].concat(),
            vec![vec![0x14], str_to_u8_vec("asdfghjkl"), vec![0xba, 0xde, 0xd]].concat(),
            vec![vec![0xa0], str_to_u8_vec("zxcvbnm"), vec![0xbc, 0xbe, 0xbf, 0xa0]].concat()
        ],
        _ => vec![ //standard qwerty
            vec![vec![0x9], str_to_u8_vec("qwertyuiop"), vec![0xdb, 0xdd, 0xdc]].concat(),
            vec![vec![0x14], str_to_u8_vec("asdfghjkl"), vec![0xba, 0xde, 0xd]].concat(),
            vec![vec![0xa0], str_to_u8_vec("zxcvbnm"), vec![0xbc, 0xbe, 0xbf, 0xa1]].concat()
        ]
    };

    let mut map: HashMap<i32, (i32, i32)> = HashMap::new();

    for row in layout_table {
        for (i, c) in row.iter().enumerate() {
            if i == 0 {
                map.insert(*c as i32, (RESERVED_KEY, row[i+1] as i32));
            } else if i == row.len() - 1 {
                map.insert(*c as i32, (row[i-1] as i32, RESERVED_KEY));
            } else {
                map.insert(*c as i32, (row[i-1] as i32, row[i+1] as i32));
            }
        }
    }

    return map
}

fn str_to_u8_vec(s: &str) -> Vec<u8> {
    Vec::from(s.to_string()).iter().map(|i| i - 0x20).collect()
}

fn get_key_value(i: i32) -> String {
    match i as i32 {
        RESERVED_KEY => "".into(), //a reserved key should be blank
        0x30..=0x39 => (i as u8 as char).into(),
        0x41..=0x5a => (i as u8 as char).into(),
        winuser::VK_SHIFT => ('^').into(), // denotates that the char after will be upper
        winuser::VK_CAPITAL => ('|').into(), // denotates that from | to  | is upper
        _ => "".into()
    }
}

fn load_words(words: &mut HashSet<String>) {
    println!("Loading words...");
    let start_time = Instant::now();
    for (i, s) in fs::read_to_string("words.txt").unwrap().split("\n").enumerate() {
        if (i % 46655) == 0 {
            println!("{}%...", (i/46655)*10)
        }

        words.insert(s.to_uppercase().into());
    }
    println!("...done! Loading took {}ms", start_time.elapsed().as_millis())
}

fn stealth() {
    let stealth: windef::HWND;
    unsafe {
        winapi::um::consoleapi::AllocConsole();
        let classname = std::ffi::CString::new("ConsoleWindowClass").unwrap();
        stealth = winuser::FindWindowA(classname.as_ptr(), std::ptr::null());
        winuser::ShowWindow(stealth, 0);
    }
}