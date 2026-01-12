// sorting/insertion_sort.rs
// 插入排序 / Insertion Sort
//
// ✅ 複雜度 / Complexity
// - Best (最佳): O(n)          -> 當資料幾乎已排序，內層 while 幾乎不跑
// - Average (平均): O(n^2)     -> 平均會有很多元素要往左移/交換
// - Worst (最差): O(n^2)       -> 當資料完全反序，每個 i 都要一路移到最左邊
// - Space (空間): O(1)         -> 原地排序，只用常數額外空間
// - Stable (穩定): Yes         -> 相等元素不會改變原本相對順序（此版本只在 < 才交換）
//
// ✅ 這個演算法在做什麼 / What it does
// 想像你在「整理手上的撲克牌」：
// - 你每次拿到一張新牌（arr[i]）
// - 把它往左邊已經排好的牌堆中，插到正確的位置
//
// ✅ 你問的關鍵：是不是看 arr[i] 跟 arr[i-1]？
// 是的，核心判斷就是：
// - 如果 arr[i] >= arr[i-1]，代表它已經在正確位置，不用插入
// - 如果 arr[i] < arr[i-1]，就一路往左比較並交換，直到放到該去的位置
//
// ✅ 實際流程（以 i 為當前要插入的元素）
// 1) 左邊區間 arr[0..i) 保證已排序
// 2) 讓 j = i
// 3) 當 j>0 且 arr[j] < arr[j-1]：就交換 arr[j] 和 arr[j-1]，並 j -= 1
// 4) 停下來時，arr[j] 就在正確位置，左側仍保持排序
pub fn insertion_sort<T: Ord + std::fmt::Debug>(arr: &mut [T]) {
    println!("初始陣列: {:?}\n", arr);

    for i in 1..arr.len() {
        println!("=== 外層 i = {} ===", i);
        println!(
            "準備插入元素 arr[{}] = {:?}",
            i, arr[i]
        );

        let mut j = i;

        while j > 0 {
            println!(
                "  比較 arr[{}] = {:?} 與 arr[{}] = {:?}",
                j - 1,
                arr[j - 1],
                j,
                arr[j]
            );

            if arr[j] < arr[j - 1] {
                println!("  → 右邊較小，進行 swap");
                println!("  swap 前: {:?}", arr);

                arr.swap(j, j - 1);

                println!("  swap 後: {:?}", arr);
                j -= 1;
            } else {
                println!("  → 不需要交換，插入完成");
                break;
            }
        }

        if j == 0 {
            println!("  已移到最左邊，插入完成");
        }

        println!("本輪結束後陣列: {:?}\n", arr);
    }

    println!("排序完成: {:?}", arr);
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insertion_sort() {
        let mut v = vec![5, 2, 4, 6, 1, 3];
        insertion_sort(&mut v);
        assert_eq!(v, vec![1, 2, 3, 4, 5, 6]);
    }
}
