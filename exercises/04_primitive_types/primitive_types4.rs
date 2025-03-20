fn main() {
    // You can optionally experiment here.
    let numbers = [1, 2, 3, 4, 5];
    // create a slice of 2nd and 3rd element
    let slice = &numbers[1..3];

    println!("array = {:?}", numbers);
    println!("slice = {:?}", slice);
}

#[cfg(test)]
mod tests {

    #[test]
    fn slice_out_of_array() {
        let a = [1, 2, 3, 4, 5];

        // TODO: Get a slice called `nice_slice` out of the array `a` so that the test passes.
        // an array of numbers
        let nice_slice = &a[1..4];

        assert_eq!([2, 3, 4], nice_slice);
    }
}
