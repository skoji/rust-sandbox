pub mod balance_tree;
mod double_linked;
fn main() {
    let mut mylog = double_linked::MyLog::new_empty();
    mylog.append("first".to_string());
    mylog.append("second".to_string());
    mylog.append("third".to_string());
    let mut iter = double_linked::LinkIterator::new(&mylog);

    println!("{}", iter.next().unwrap());
    // println!("{}", iter.next().unwrap());
    // println!("{}", iter.next_back().unwrap());
    // println!("{}", iter.next_back().unwrap());
}
