fn main() {
    creating_an_iterator();
    using_an_iterator_in_a_for_loop();
    methods_that_produce_other_iterators();
    using_closure_that_capture_environment();
}

fn creating_an_iterator() {
    let v1 = vec![1, 2, 3];
    let _v1_iter = v1.iter();
}

fn using_an_iterator_in_a_for_loop() {
    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();

    for val in v1_iter {
        println!("Got: {val}");
    }
}

#[test]
fn calling_the_next_method_on_a_iterator() {
    let v1 = vec![1, 2, 3];

    let mut v1_iter = v1.iter();

    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);
}

#[test]
fn calling_the_sum_method() {
    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();

    let total: i32 = v1_iter.sum();

    assert_eq!(total, 6);
}

fn methods_that_produce_other_iterators() {
    let v1: Vec<i32> = vec![1, 2, 3];

    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();

    for number in v2.iter() {
        println!("{number}");
    }
}

#[cfg(test)]
mod tests {
    #[derive(PartialEq, Debug)]
    struct Shoe {
        size: u32,
        style: String,
    }

    #[test]
    fn filters_by_size() {
        let shoes = vec![
            Shoe {
                size: 10,
                style: String::from("sneaker"),
            },
            Shoe {
                size: 13,
                style: String::from("sandal"),
            },
            Shoe {
                size: 10,
                style: String::from("boot"),
            },
        ];

        let in_my_size: Vec<Shoe> = shoes.into_iter().filter(|s| s.size == 10).collect();

        assert_eq!(
            in_my_size,
            vec![
                Shoe {
                    size: 10,
                    style: String::from("sneaker"),
                },
                Shoe {
                    size: 10,
                    style: String::from("boot"),
                }
            ]
        )
    }
}

fn using_closure_that_capture_environment() {}
