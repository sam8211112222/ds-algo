// sorting/shell_sort.rs
// 希爾排序 / Shell Sort
//
// ✅ 複雜度 / Complexity (取決於 gap 序列 / depends on gap sequence)
// - Best: 可能接近 O(n log n) 或更好（資料分布與 gap 影響很大）
// - Average: 常見估計約 O(n^1.3 ~ n^1.5)（非精確，取決於 gap）
// - Worst: O(n^2)（某些 gap 會退化）
// - Space: O(1)           -> 原地
// - Stable: No            -> gap 跳躍交換可能改變相等元素順序
//
// ✅ 這個演算法在做什麼 / What it does
// Shell sort 是「插入排序的加速版」：
// - 插入排序慢在：元素一次只能往左移一格
// - Shell sort 先用較大的 gap 讓元素可以「一次跳很多格」靠近正確位置
// - gap 逐漸縮小，最後 gap=1 就變成普通 insertion sort，收尾完成排序
//
// ✅ 這裡用 Ciura gap 序列（常見且表現不錯）
// Gaps: 701, 301, 132, 57, 23, 10, 4, 1
//
// ✅ 每個 gap 做什麼？
// 對每個 i：
// - 比較 arr[i] 與 arr[i-gap]
// - 若 arr[i] 比較小，就往左以 gap 為步長交換（gapped insertion sort）
pub fn shell_sort<T: Ord + std::fmt::Debug>(arr: &mut [T]) {
    println!("shell_sort 開始 arr = {:?}", arr);

    let n = arr.len();
    if n <= 1 {
        return;
    }

    let mut gaps = vec![701, 301, 132, 57, 23, 10, 4, 1];
    gaps.retain(|&g| g < n);

    for gap in gaps {
        println!("\n=== gap = {} ===", gap);

        for i in gap..n {
            let mut j = i;
            println!("處理 index {} value {:?}", i, arr[i]);

            while j >= gap && arr[j] < arr[j - gap] {
                println!(
                    "swap arr[{}]={:?} 與 arr[{}]={:?}",
                    j, arr[j], j - gap, arr[j - gap]
                );
                arr.swap(j, j - gap);
                println!("arr = {:?}", arr);
                j -= gap;
            }
        }

        println!("gap={} 結束 arr={:?}", gap, arr);
    }

    println!("\nshell_sort 完成 arr = {:?}", arr);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shell_sort() {
        let mut v = vec![9, 8, 3, 7, 5, 6, 4, 1];
        shell_sort(&mut v);
        assert_eq!(v, vec![1, 3, 4, 5, 6, 7, 8, 9]);
    }
}
