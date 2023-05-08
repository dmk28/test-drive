fn main() {
   
    //this syntax can create a vector
    let mut vectors = vec![1, 2, 3, 4];
    //we are declaring type Vectors and data type inside the vectors.
    //you can add items to a vector using .push()
    //you can remove items from a vector using .remove();
    //you need to use iter_mut and the
    vectors.iter_mut().for_each(|x| *x *= 2);

    //alternatively:
    //use into_iter() with .map() and .collect()
    vectors = vectors.into_iter().map(|x| x * 4).collect();

        //.iter() is a method that allows you to iterate through the array to print it all, like so:

    for element in vectors.iter() {
        println!("{}", element);
    }

}
