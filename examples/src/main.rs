extern crate segment_tree;

use segment_tree::SegmentTree;

#[derive(Debug, Default, Clone)]
struct Sum(i32);

impl segment_tree::Aggregator for Sum {
    fn aggregate(&self, other: Sum) -> Sum {
        Sum(self.0 + other.0)
    }
}

impl From<i32> for Sum {
    fn from(i: i32) -> Sum {
        Sum(i)
    }
}

impl Into<i32> for Sum {
    fn into(self) -> i32 {
        self.0
    }
}

fn main() {
    let mut st: SegmentTree<Sum> = SegmentTree::build(&[1, 2, 3, 4, 5]);
    // Original data
    println!("{:?}", st.make_vec::<i32>());
    println!("Sum in range [1, 3] = {:?}", st.query(1, 3));
    // After update
    st.update(2, 36);
    println!("{:?}", st.make_vec::<i32>());
    println!("Sum in range [1, 3] = {:?}", st.query(1, 3));
}
