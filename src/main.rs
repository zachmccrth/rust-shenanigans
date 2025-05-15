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
   println!("{:?} {:?}", left, right);
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

fn main() {
   let array = [8,2,3,4,5,6,7,8];
   let result = merge_sort(&array);
   println!("{:?}", result);
}
