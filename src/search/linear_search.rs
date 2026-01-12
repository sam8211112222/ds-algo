// search/linear_search.rs
// 線性搜尋 / Linear Search
//
// ✅ 複雜度 / Complexity
// - Best: O(1)            -> 第一個元素就命中
// - Average: O(n)
// - Worst: O(n)           -> 找不到或在最後一個
// - Space: O(1)
//
// ✅ 這個演算法在做什麼 / What it does
// 從陣列的「第一個元素」開始，一個一個往後比對：
// - 如果目前元素 == target，就回傳 index
// - 如果走完整個陣列都沒找到，就回傳 None
//
// ✅ 直覺理解
// - 就像翻書從第一頁一路翻到最後一頁
// - 不管資料有沒有排序，都一定能用
pub fn linear_search<T: PartialEq>(arr: &[T], target: &T) -> Option<usize> {
    println!("[linear_search] start, len={}", arr.len());

    for (i, v) in arr.iter().enumerate() {
        println!("[linear_search] compare index={}", i);
        if v == target {
            println!("[linear_search] found at index={}", i);
            return Some(i);
        }
    }

    println!("[linear_search] not found");
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_linear_search_found() {
        let v = vec![3, 1, 4, 1, 5];
        let idx = linear_search(&v, &4);
        assert_eq!(idx, Some(2));
    }

    #[test]
    fn test_linear_search_not_found() {
        let v = vec![3, 1, 4, 1, 5];
        let idx = linear_search(&v, &9);
        assert_eq!(idx, None);
    }

    #[test]
    fn test_linear_search_empty() {
        let v: Vec<i32> = vec![];
        let idx = linear_search(&v, &1);
        assert_eq!(idx, None);
    }
}
