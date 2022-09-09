
mod closures;
mod iterators;

fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    iterators::iterate();

    closures::generate_workout(simulated_user_specified_value, simulated_random_number);
}
