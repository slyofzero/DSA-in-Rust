// mod stack;
// mod linked_list;
// mod queue;
mod priority_queue;
use priority_queue::{PriorityQueue, Sort};

fn main() {
    let mut queue = PriorityQueue::<isize>::new(Sort::Min);

    queue.add(11);
    queue.add(5);
    queue.add(2);
    queue.add(5);
    queue.add(1);
    queue.add(1);
    queue.add(-5);
    queue.add(99);

    let index = queue.map.get(&100).map_or(None,|set| set.first());

    println!("{:?}", queue.map);
    println!("{:?}", queue.heap);
    println!("{:?}", index);
}