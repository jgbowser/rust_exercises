// Chapter 10 : Generic Types, Traits, and Lifetimes

mod generic_types;

fn main() {
    // before we really dig into all this, lets first dig into removing duplication
    // in our code by refactoring it into functions; functions that can
    // take generics to make them even more reusable
    let number_list = vec![34, 50, 25, 100, 65];
    let mut largest = &number_list[0];
    for number in &number_list {
        if number > largest {
            largest = number;
        }
    }

    println!("the largest number is {}", largest);

    // if we want to be able to use this functionality for more than just
    // our initial list we need to break this out into a reusable function
    fn find_largest(list: &[i32]) -> &i32 {
        let mut largest = &list[0];
        for item in list {
            if item > largest {
                largest = item;
            }
        }

        largest
    }

    let number_list2 = vec![34, 50, 25, 100, 65];
    let number_list3 = vec![102, 34, 6000, 89, 54, 2, 43, 8];
    let result1 = find_largest(&number_list2);
    println!("largest in list 1 is: {}", result1);
    let result2 = find_largest(&number_list3);
    println!("largest in list 2 is: {}", result2);

    // In section 1 we'll use generics to reduce duplication even more

    generic_types::run();
}
