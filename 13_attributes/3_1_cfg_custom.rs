// Some conditionals like 'target_os' are implicitly provided by 'rustc',
// but custom conditionals must be passed to 'rustc' using the '--cfg' flag

#[cfg(some_condition)]
fn conditional_function() {
    println!("condition met!");
}

fn main() {
    conditional_function();
}

// To successfully run, you need to run with the custom conditional flag
// 'rustc --cfg some_condition 3_1_cfg_custom.rs && ./3_1_cfg_custom.rs'