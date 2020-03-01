
use rand::Rng;

struct Heap {
    data: Vec<i64>,
    size: usize,
    capacity: usize,
}

impl Heap {

    fn new() -> Heap {
        return Heap {
            data: Vec::new(),
            size: 0,
            capacity: 0
        };
    }

    // get index relative to another //

    fn get_left_child_index(&self, parent_index: usize) -> usize{
        return 2 * parent_index + 1;
    }

    fn get_right_child_index(&self, parent_index: usize) -> usize{
        return 2 * parent_index + 2;
    }

    fn get_parent_index(&self, child_index: usize) -> usize {
        if child_index == 0 {
            return 0;
        } else {
            return (child_index - 1) / 2;
        }   
    }

    
    // check if relative index exists //

    fn has_left_child(&self, index: usize) -> bool {
        return self.get_left_child_index(index) < self.size;
    }

    fn has_right_child(&self, index: usize) -> bool {
        return self.get_right_child_index(index) < self.size;
    }

    fn has_parent(&self, index: usize) -> bool {
        return self.get_parent_index(index) >= 0;
    }


    // get relative index value //

    fn get_left_child(&self, index: usize) -> i64 {
        return self.data[ self.get_left_child_index(index) ];
    }

    fn get_right_child(&self, index: usize) -> i64 {
        return self.data[ self.get_right_child_index(index) ];
    }

    fn get_parent(&self, index: usize) -> i64 {
        return self.data[ self.get_parent_index(index) ];
    }


    // swap two value in the heap //
    fn swap(&mut self, index_one: usize, index_two: usize) {

        if index_one < (self.size as usize)-1 && index_two < (self.size as usize)-1 {
            let tmp: i64 = self.data[index_one];
            self.data[index_one] = self.data[index_two];
            self.data[index_two] = tmp;
        } else {
            ; // error handling
        }
    }

    // check is space is left //
    fn has_space_left(&self) -> bool {
        if( self.size >= self.capacity ){
            return false;
        }
        return true;
    }

    // get min value //
    fn peek(&self) -> i64 {
        if self.size != 0 {
            return self.data[0];
        } else {
            ; // error handling
            return 0;
        }
    }

    // ensure values are correctly placed in heap in a bottom up fashion
    fn heapify_up(&mut self) {
        let mut index: usize = (self.size as usize) - 1; // last node
        while( self.has_parent(index) && self.get_parent(index) > self.data[index] ) {
            self.swap(self.get_parent_index(index), index);
            index = self.get_parent_index(index);
        }
    }

    // ensure values are correctly placed in heap in a top down fashion
    fn heapify_down(&mut self) {

        let mut index: usize = 0; // root node

        while( self.has_left_child(index) ) {
            let mut smaller_child_index = self.get_left_child_index(index);

            if self.has_right_child(index) && self.get_right_child(index) < self.get_left_child(index) {
                smaller_child_index = self.get_right_child_index(index);
            }

            if self.data[index] < self.data[smaller_child_index] {
                break;
            } else {
                self.swap(index, smaller_child_index);
            }
            index = smaller_child_index;
        }
    }

    // pop min value while maintaining heap structure
    fn poll(&mut self) -> i64 {

        let value: i64 = self.data[0];
        self.data[0] = self.data[(self.size as usize)-1];
        self.size -= 1;
        self.heapify_down();

        return value;
    }

    // add value to heap while maintaining structure
    fn add(&mut self, new_value: i64){

        self.has_space_left();
        self.data.push(new_value);
        self.size += 1;
        self.heapify_up();
    }

}

fn main(){

    println!("Heap demo");

    let mut array: [i64; 32] = [0;32];
    for i in 0..32 { array[i] = rand::thread_rng().gen_range(1, 10) }

    let mut my_heap: Heap = Heap::new();

    for i in 0..32 { my_heap.add(array[i]) }

    for i in 0..32 {
        print!("{}\n", my_heap.poll());
        for i in 0..my_heap.data.len() {
            print!("{}", my_heap.data[i]);
        }
    }

}