// Ig let's just try a basic sorting algo first?

fn merge_sort(array_to_sort: &[i32]) -> Vec<i32> {
   let array_length: usize = array_to_sort.len();
   merge_sort_indexed(&array_to_sort[0 ..array_length])
}

fn merge_sort_indexed(array_to_sort: &[i32]) -> Vec<i32> {
   if array_to_sort.len() <= 1 {
      return array_to_sort.to_vec();
   }
   let mid: usize = array_to_sort.len() / 2;
   let left = &array_to_sort[..mid];
   let right = &array_to_sort[mid..];
   merge(merge_sort_indexed(left).as_ref(), merge_sort_indexed(right).as_ref())
}

fn merge<'a>(left: &'a[i32], right: &'a[i32]) -> Vec<i32> {
   let mut merged = vec![0; left.len() + right.len()];
   let mut i = 0;
   let mut j = 0;
   while i < left.len() && j < right.len() {
      if left[i] < right[j] {
         merged[i + j] = left[i];
         i += 1;
      }  else {
         merged[i + j] = right[j];
         j += 1;
      }
   };
   while i < left.len() {
      merged[i + j] = left[i];   
      i += 1;
   }
   while j < right.len() {
      merged[i + j] = right[j];
      j += 1;
   } 
  merged 
}

fn quicksort(array_to_sort: &[i32]) -> Vec<i32> {
   if array_to_sort.len() <= 1 {
      return array_to_sort.to_vec();
   }
   let pivot = generate_pivot(&array_to_sort);
   let (left, right) = move_over_pivot(array_to_sort, pivot);
   [quicksort(&left[0..left.len() - 1]), Vec::from([left[left.len() - 1]]), quicksort(&right)].concat()
}

fn move_over_pivot(array: &[i32], pivot_index: usize) -> (Vec<i32>,Vec<i32>) {
   let mut left = vec!();
   let mut right = vec!();
   let pivot_value = array[pivot_index];
   for i in 0..array.len(){
      if i == pivot_index { continue } 
      if array[i] <= pivot_value {
         left.push(array[i]);
      } else {
         right.push(array[i]);
      }
   }
   left.push(pivot_value);
   (left, right)
}

fn generate_pivot(p0: &&[i32]) -> usize {
   p0.len()/2
}

fn main() {
   let array = [1];
   let result = merge_sort(&array);
   let result2 = quicksort(&array);
   assert_eq!(result, result2);
   println!("{:?}", result);
   println!("{:?}", result2);
}
