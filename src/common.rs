//create a function to draw a tree
pub fn draw_tree() {
    println!("
        *
       ***
      *****
     *******
    *********
   ***********
  *************
 ***************
       |||
       |||
       |||
    --------");
}
// fuction to draw a square
pub fn draw_square() {
    println!("
    ********
    *      *
    *      *
    *      *
    *      *
    ********");
}

// function to draw a circle
pub fn draw_circle() {
    println!("
       ****
     *      *
    *        *
    *        *
    *        *
     *      *
       ****");
}
// function to find maximum of vector
pub fn find_max(v: Vec<i32>) -> i32 {
    let mut max = v[0];
    for i in &v {
        if *i > max {
            max = *i;
        }
    }
    max
}
//