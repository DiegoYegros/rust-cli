use std::env;
use std::process::exit;
mod rest_client;
use std::thread;
use std::time::Instant;

fn main() {
    loop {
        clearscreen();

        println!(
            "Select an option:
    1. Search for a word in a file
    2. Consume REST API
    3. Multi-threaded Test
    0. Exit"
        );
        let num: i32 = get_input_from_user();
        if num == 0 {
            println!("It has been nice to see you! good bye!");
            exit(0);
        }
        if num == 1 {
            command_search_word_in_file();
        } else if num == 2 {
            command_consume_rest_api();
        } else if num == 3 {
            command_multithread_test();
        } else {
            println!("Hmm, that didn't work. maybe try again?");
            press_enter_to_continue();
        }
    }
}

fn command_search_word_in_file() {
    println!("Great! What word are we looking for?");
    let mut word: String = String::new();
    while word.trim().is_empty() {
        std::io::stdin().read_line(&mut word).unwrap();
        if word.trim().is_empty() {
            println!(
            "hmm, an empty word is no word, and with no word i cannot work. please give me a word!"
        );
        }
    }

    println!("I think I can do it, where should I look for it?");
    let exe_path = env::current_exe().unwrap();
    let base_dir = exe_path
        .ancestors()
        .nth(3)
        .unwrap()
        .to_str()
        .unwrap()
        .to_owned()
        + "\\data";
    println!("note: base directory is {}", base_dir);
    let mut path: String = String::new();
    while path.trim().is_empty() {
        std::io::stdin().read_line(&mut path).unwrap();
        if path.trim().is_empty() {
            println!("i don't think i can find a word in a file without an actual file. can you help me?");
        }
    }

    let invalid_chars = ["<", ">", ":", "/", "\\", "|", "?", "*"];
    for c in &invalid_chars {
        path = path.replace(*c, "_");
    }
    path = path.trim().to_owned();

    let abs_path = format!("{}\\{}", base_dir, path);
    let contents = match std::fs::read_to_string(&abs_path) {
        Ok(contents) => contents,
        Err(error) => {
            println!("Error opening file {}Error message:{}", abs_path, error);
            press_enter_to_continue();
            return;
        }
    };
    search_words_in_file(contents, word);
    press_enter_to_continue();
}

fn clearscreen() {
    print!("\x1B[2J\x1B[1;1H");
}

fn press_enter_to_continue() {
    println!("Press enter to continue...");
    let mut var: String = String::new();
    std::io::stdin().read_line(&mut var).unwrap();
}

fn search_words_in_file(content: String, word: String) {
    println!("Word is: {} ", word);
    let lines = content.lines();
    let mut line_num: i32 = 0;
    for line in lines {
        if line.contains(&word.trim()) {
            println!("Found it! Line {}: {}", line_num, line)
        }
        line_num += 1;
    }
}

fn get_input_from_user() -> i32 {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    let num = line.trim().parse::<i32>().unwrap();
    return num;
}

fn command_consume_rest_api() {
    let _ = rest_client::main();
    press_enter_to_continue();
}

fn command_multithread_test() {
    test_single_thread();
    test_multi_thread();
    press_enter_to_continue();
}

fn test_single_thread() {
    let mut now = Instant::now();
    let mut x: u64 = 0;
    for i in 1..100000001 {
        x = i;
    }
    let mut x: u64 = 0;
    for i in 1..100000001 {
        x = i;
    }
    let mut x: u64 = 0;
    for i in 1..100000001 {
        x = i;
    }
    println!(
        "Elapsed time is {}ms with a single thread.",
        now.elapsed().as_millis()
    );
}

fn test_multi_thread() {
    let counter = std::sync::Arc::new(std::sync::Mutex::new(0));
    let mut handles = Vec::new();

    let start = Instant::now();

    for _ in 0..3 {
        let counter = std::sync::Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num: u64 = 0;
            for i in 1..100000001 {
                num += i;
            }
            let mut guard = counter.lock().unwrap();
            *guard += num;
        });
        handles.push(handle);
    }

    for h in handles {
        h.join().unwrap();
    }

    println!(
        "Elapsed time is {}ms with 3 threads.",
        start.elapsed().as_millis()
    );
}
