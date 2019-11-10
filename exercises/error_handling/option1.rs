// option1.rs
// This example panics because the second time it calls `pop`, the `vec`
// is empty, so `pop` returns `None`, and `unwrap` panics if it's called
// on `None`. Handle this in a more graceful way than calling `unwrap`!
// Scroll down for hints :)

pub fn pop_too_much() -> bool {
    let mut list = vec![3];

    let last = list.pop().unwrap();
    println!("The last item in the list is {:?}", last);

    // match list.pop() {
    //     Some(second_to_last) => {
    //         println!(
    //             "The second-to-last item in the list is {:?}",
    //             second_to_last
    //         );
    //     },
    //     None => {
    //         println!("Vector is empty");
    //     }
    // }

    // if let last = list.pop().unwrap_or(-1);
    // println!("The last item in the list is {:?}", last);

    // let second_to_last = list.pop().unwrap_or(-1);
    // println!(
    //     "The second-to-last item in the list is {:?}",
    //     second_to_last
    // );

    // if let Some(last) = list.pop() {
    //     println!("The last item in the list is {:?}", last);
    // }

    if let Some(second_to_last) = list.pop() {
        println!(
            "The second-to-last item in the list is {:?}",
            second_to_last
        );
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_not_panic() {
        assert!(pop_too_much());
    }
}























// Try using a `match` statement where the arms are `Some(thing)` and `None`.
// Or set a default value to print out if you get `None` by using the
// function `unwrap_or`.
// Or use an `if let` statement on the result of `pop()` to both destructure
// a `Some` value and only print out something if we have a value!
