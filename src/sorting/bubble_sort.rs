// sorting/bubble_sort.rs
// 氣泡排序 / Bubble Sort
//
// ✅ 複雜度 / Complexity
// - Best: O(n)            -> 若已排序，第一輪沒有交換就提早結束
// - Average: O(n^2)
// - Worst: O(n^2)
// - Space: O(1)
// - Stable: Yes           -> 只在 > 才交換，相等不換順序
//
// ✅ 這個演算法在做什麼 / What it does
// 每一輪都「從左到右」比較相鄰的兩個元素：
// - 如果 arr[i] > arr[i+1]，就交換
// 這樣一輪跑完後：最大的元素會被一路「冒泡」到最右邊
//
// ✅ 直覺理解
// - 第 1 輪：把最大值推到最後
// - 第 2 輪：把次大值推到倒數第二
// - ...
//
// ✅ 這版有 early exit（提早結束）
// - 如果某一輪完全沒有交換，代表已經排好了，直接 break
pub fn bubble_sort<T: Ord>(arr: &mut [T]) {
    let n = arr.len();
    if n <= 1 {
        return;
    }

    for end in (1..n).rev() {
        let mut swapped = false;

        for i in 0..end {
            if arr[i] > arr[i + 1] {
                arr.swap(i, i + 1);
                swapped = true;
            }
        }

        if !swapped {
            break;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bubble_sort() {
        let mut v = vec![3, 2, 1];
        bubble_sort(&mut v);
        assert_eq!(v, vec![1, 2, 3]);
    }
}
