use rwc::{ get_args, count };

fn main() {
    let input = get_args();

    let mut totals = Vec::new();
    for file in &input.files {
        totals.push(count(file, &input));
    }

    let sum = totals.into_iter().reduce(|a, b| a.add(b));
}
