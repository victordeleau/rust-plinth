use std::vec::Vec;
use rand::Rng;
use rand::thread_rng;

// W I P //

fn quick_sort(array: &Vec<i64>, low: usize, high: usize){
    /* 
     * 
     */

    if low < high {
        let pivot = partition(&array, low, high);
        quick_sort(&array, low, pivot-1);
        quick_sort(&array, pivot+1, high);
    }

    fn partition(array: &Vec<i64>, low: usize, high: usize) -> usize {
        /*
         *
         */
        let pivot = low-1;

        for j in low..high-1 {

            if array[j] <= array[high] {
                pivot += 1;

                // swap values
                let tmp = array[pivot];
                array[pivot] = array[j];
                array[j] = tmp;
            }
        }
        // swap values
        let tmp = array[pivot+1];
        array[pivot+1] = array[high];
        array[high] = tmp;

        return pivot;
    }
}

fn main() {

    let SIZE: usize = 2;

    // create random number generator
    let mut rng = thread_rng();

    // create vector of random number
    let mut data: Vec<i64> = (0..SIZE).map(|_| {
        rng.gen_range(0, SIZE as i64) 
    }).collect();

    // apply sorting algorithm
    quick_sort(&data, 1, SIZE);

    println!("{:?}", data);
}
