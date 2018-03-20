struct InfiniteSeq {
    curr: u32,
}

impl Iterator for InfiniteSeq {
    type Item = u32;

    fn next(&mut self) -> Option<u32> {
        let ret_value = self.curr;
        self.curr = self.curr + 1;
        Some(ret_value)
    }
}

fn get_counter() -> InfiniteSeq {
    InfiniteSeq { curr: 0 }
}

fn main() {
    let counter = get_counter();

    for i in counter.take_while(|x| *x < 10) {
        println!("{}", i);
    }

    println!("Hello, infinite sequence!");
}
