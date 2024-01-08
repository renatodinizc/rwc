use rwc::{ get_args, display };

fn main() {
    let input = get_args();

    let mut totals = Vec::new();
    for file in &input.files {
        totals.push(display(file, &input));
    }

    let _sum = totals.into_iter().reduce(|a, b| {
        (a.0 + b.0, a.1 + b.1, a.2 + b.2, a.3 + b.3)
    }).unwrap();
}
