use std::vec;

pub fn foo() {
 let x = vec!(0, 1, 2, 3, 4, 5, 6, 7);
 let y = 0;
 for ele in x.iter() {
     y += ele
 }
}

pub unsafe fn unsafe_foo() {
 let x = vec!(0, 1, 2, 3, 4, 5, 6, 7);
 let y = 0;
 for ele in x.iter() {
     y += ele
 }

}

// comment test
pub fn bar() {
    let x = vec!(0, 1, 2, 3, 4, 5, 6, 7);

    unsafe {
        let y = 0;
        for ele in x.iter() {
            y += ele
        }
    }
}
/* total functions: 3
 * not safe functions: 1
 * not safe blocks: 1
 * total lines of not safe: 9 (counting the line with close bracket)
 */
