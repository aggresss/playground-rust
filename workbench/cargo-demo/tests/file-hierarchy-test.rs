// tests/sample.rs (sample integration test)

#[test]
fn my_sample() {
    assert_eq!(123, 123); // Integration tests (and benchmarks) 'depend' to the crate like
} // a 3rd party would. Hence, they only see public items.
