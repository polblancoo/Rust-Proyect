use std::vec;



fn binarysearch(nums: Vec<i32>,  target: i32  ) -> i32 {
 
    let mut low: usize = 0;
    let mut high: usize = nums.len() - 1 ;
        while low <= high {
            let mid:usize = low + high-low/2 ;
            if nums[mid]==target {
                return mid as i32;
            } else if nums[mid] < target {
                low = mid + 1;
            } else {
                high = mid - 1 ;
            }
        }
        -1
    }




fn main() {
    
let   array: Vec<i32> = vec![1, 2, 101, 192, 3004];
let array2: Vec<i32> = array.clone();
let objective: i32 = 101;

let result =binarysearch(array,  objective);

if result == (-1) {
    print!("El elemento  NO se encontro. " );
} else {
    let r = result as usize;
    print!("El elemento se encontro en la posicion: {} , \n (Dato : {} )\n", result, array2[r] );
}

}