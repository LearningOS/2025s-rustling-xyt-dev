/*
	sort
	This problem requires you to implement a sorting algorithm
	you can use bubble sorting, insertion sorting, heap sorting, etc.
*/

fn sort<T: Ord>(array: &mut [T]){
	//TODO
    let len = array.len();
    for root in (0..len / 2).rev() {
        heapify(array, root, len);
    }
    for e in (1..len).rev() {
        array.swap(0, e);
        heapify(array, 0, e);
    }
}
fn heapify<T: Ord>(array: &mut [T], root: usize, len: usize) {
    let mut largest = root;
    let left = root + 1;
    let right = root + 2;
    if left < len && array[left] > array[largest] {
        largest = left;
    }
    if right < len && array[right] > array[largest] {
        largest = right;
    }
    if largest != root {
        array.swap(largest, root);
        heapify(array, largest, len);
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_1() {
        let mut vec = vec![37, 73, 57, 75, 91, 19, 46, 64];
        sort(&mut vec);
        assert_eq!(vec, vec![19, 37, 46, 57, 64, 73, 75, 91]);
    }
	#[test]
    fn test_sort_2() {
        let mut vec = vec![1];
        sort(&mut vec);
        assert_eq!(vec, vec![1]);
    }
	#[test]
    fn test_sort_3() {
        let mut vec = vec![99, 88, 77, 66, 55, 44, 33, 22, 11];
        sort(&mut vec);
        assert_eq!(vec, vec![11, 22, 33, 44, 55, 66, 77, 88, 99]);
    }
}