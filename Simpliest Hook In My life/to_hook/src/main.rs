fn function_to_hook(value: i32, printout: bool) {
    if printout {
        println!("Number [ {:?} ] You passed", value);
    }
}

fn main() {
    function_to_hook(0, false);

    loop {
    }
}