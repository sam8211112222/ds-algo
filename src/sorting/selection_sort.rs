// sorting/selection_sort.rs
// 選擇排序 / Selection Sort
//
// ✅ 複雜度 / Complexity
// - Best: O(n^2)          -> 不管資料如何，每輪都要掃描剩下的元素找最小值
// - Average: O(n^2)
// - Worst: O(n^2)
// - Space: O(1)           -> 原地交換
// - Stable: No            -> 交換時可能把相等元素的相對順序打亂
//
// ✅ 這個演算法在做什麼 / What it does
// 每一輪 i：
// 1) 在「尚未排序區」arr[i..n) 找出最小值的 index（min_idx）
// 2) 把那個最小值跟 arr[i] 交換
//
// ✅ 直覺理解
// - 第 1 輪：把全陣列最小的放到 index 0
// - 第 2 輪：把剩下部分最小的放到 index 1
// - ...直到結束
pub fn selection_sort<T: Ord + std::fmt::Debug>(arr: &mut [T]) {
    println!("selection_sort 開始 arr = {:?}", arr);

    let n = arr.len();
    for i in 0..n {
        let mut min_idx = i;
        println!("\n第 {} 輪，假設最小值是 arr[{}]={:?}", i, i, arr[i]);

        for j in (i + 1)..n {
            println!(
                "比較 arr[{}]={:?} 與目前最小 arr[{}]={:?}",
                j, arr[j], min_idx, arr[min_idx]
            );

            if arr[j] < arr[min_idx] {
                min_idx = j;
                println!("→ 更新最小值 index = {}", min_idx);
            }
        }

        if min_idx != i {
            println!("swap arr[{}] 與 arr[{}]", i, min_idx);
            arr.swap(i, min_idx);
        }

        println!("本輪結束 arr = {:?}", arr);
    }

    println!("\nselection_sort 完成 arr = {:?}", arr);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_selection_sort() {
        let mut v = vec![64, 25, 12, 22, 11];
        selection_sort(&mut v);
        assert_eq!(v, vec![11, 12, 22, 25, 64]);
    }
}
