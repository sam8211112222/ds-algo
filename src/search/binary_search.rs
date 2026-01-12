// search/binary_search.rs
// 二分搜尋 / Binary Search
//
// ⚠️ 使用前提
// - 陣列必須「已排序」
//
// ✅ 複雜度 / Complexity
// - Best: O(1)            -> 一開始 mid 就命中
// - Average: O(log n)
// - Worst: O(log n)
// - Space: O(1)
//
// ✅ 這個演算法在做什麼 / What it does
// 在已排序陣列中，維護一個搜尋區間 [left, right]：
// - 每次取中間 mid
// - 若 arr[mid] == target → 找到
// - 若 arr[mid] < target  → 搜尋右半邊
// - 若 arr[mid] > target  → 搜尋左半邊
//
// ✅ 直覺理解
// - 像猜數字遊戲
// - 每一次都直接排除「一半」不可能的範圍
pub fn binary_search<T: Ord>(arr: &[T], target: &T) -> Option<usize> {
    println!("[binary_search] start, len={}", arr.len());

    if arr.is_empty() {
        println!("[binary_search] empty array");
        return None;
    }

    let mut left: isize = 0;
    let mut right: isize = arr.len() as isize - 1;

    while left <= right {
        let mid = left + (right - left) / 2;
        println!(
            "[binary_search] left={}, mid={}, right={}",
            left, mid, right
        );

        if &arr[mid as usize] == target {
            println!("[binary_search] found at index={}", mid);
            return Some(mid as usize);
        } else if &arr[mid as usize] < target {
            println!("[binary_search] arr[mid] < target, move left");
            left = mid + 1;
        } else {
            println!("[binary_search] arr[mid] > target, move right");
            right = mid - 1;
        }
    }

    println!("[binary_search] not found");
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_binary_search_found() {
        let v = vec![1, 3, 5, 7, 9, 11];
        let idx = binary_search(&v, &7);
        assert_eq!(idx, Some(3));
    }

    #[test]
    fn test_binary_search_not_found() {
        let v = vec![1, 3, 5, 7, 9, 11];
        let idx = binary_search(&v, &8);
        assert_eq!(idx, None);
    }

    #[test]
    fn test_binary_search_single_element() {
        let v = vec![10];
        let idx = binary_search(&v, &10);
        assert_eq!(idx, Some(0));
    }

    #[test]
    fn test_binary_search_empty() {
        let v: Vec<i32> = vec![];
        let idx = binary_search(&v, &1);
        assert_eq!(idx, None);
    }
}
