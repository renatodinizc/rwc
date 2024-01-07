use rwc::{ get_args, display };

fn main() {
    let input = get_args();

    for file in &input.files {
        display(file, &input);
     }
}
