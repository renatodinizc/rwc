use rwc::{count, get_args, print_totals};

fn main() {
    let input = get_args();

    let mut totals = Vec::new();
    for file in &input.files {
        totals.push(count(file, &input));
    }

    if totals.len() > 1 {
        let sum = totals.into_iter().reduce(|a, b| a.add(b)).unwrap();

        print_totals(sum, &input);
    }
}
