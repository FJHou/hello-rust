/**!SECTION 1 */

// fn main() {
//     let string1 = String::from("string1");
//     println!("this is {}", string1);

//     {
//         let string2 = String::from("string2");
//     }
//     // 这里直接报错，原因和js表现一致：大括号的作用域限定了string2的使用范围
//     println!("this is {}", string2);
// }

// fn main() {
//     // 和js不同，rust的每个变量只有一个所有者，变量之间的赋值等于所有权的变更，不仅仅是值的复制
//     let string1 = String::from("string1");
//     let string2 = string1; // 变更所有权
//     // 如果一个值又两个所有者，将会使rust的内存回收机制失效
//     println!("string2 = {}", string2)
// }

/**!SECTION 举例描述rust内存安全，以及“如果一个值又两个所有者，将会使rust的内存回收机制失效”原因 */

fn main() {
    let string1 = String::from("data");
    {
        // 下面的代码交换了string1的所有权到string2,当此行执行完毕后，string2的内存会被回收，所以打印string1的代码就会出错
        let string2 = string1; // 所有权转移
    }

    // println!("{}", string1)
}