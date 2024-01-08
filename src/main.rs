use rwc::{ get_args, display };

fn main() {
    let input = get_args();

    let mut totals = Vec::new();
    for file in &input.files {
        totals.push(display(file, &input));
    }

    let _sum = totals.into_iter().reduce(|a, b| a.add(b));
}
