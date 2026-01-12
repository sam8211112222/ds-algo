// sorting/counting_sort.rs
// 計數排序 / Counting Sort (stable)
//
// ✅ 複雜度 / Complexity
// - Best: O(n + k)
// - Average: O(n + k)
// - Worst: O(n + k)
//   n = 元素數量, k = 值域大小（max_value）
// - Space: O(n + k)       -> count array (k) + output (n)
// - Stable: Yes           -> 此版本「從右到左」放入 output，保持相等元素原順序
//
// ✅ 這個演算法在做什麼 / What it does
// Counting sort 不比較元素大小（不是 comparison sort），而是「用次數統計」：
// 1) count[v] = 值 v 出現了幾次
// 2) prefix sum：count[v] 變成「<= v 的總數」
// 3) 建 output：
//    - 從右到左掃 arr，每個元素 x 放到 output 的正確位置
//    - 放完後 count[x]--
// 4) 把 output 複製回 arr
//
// ✅ 限制 / Limitation
// - 只能處理非負整數
// - 如果 k 很大（例如值域 0..10^9），count 會爆記憶體
pub fn counting_sort(arr: &mut [usize], max_value: usize) {
    println!("原始陣列 arr = {:?}", arr);
    println!("max_value = {}", max_value);
    println!("--------------------------------");

    // 1️⃣ 建立 count array
    // count[v] = 數字 v 出現的次數
    let mut count = vec![0usize; max_value + 1];
    println!("初始化 count (size = {}): {:?}", count.len(), count);

    for &x in arr.iter() {
        count[x] += 1;
        println!("看到元素 {} → count[{}] += 1 → {:?}", x, x, count);
    }

    println!("--------------------------------");
    println!("完成次數統計 count = {:?}", count);

    // 2️⃣ Prefix Sum（前綴和）
    // count[v] = 有多少元素 <= v
    println!("開始做 prefix sum（前綴和）");

    for v in 1..count.len() {
        count[v] += count[v - 1];
        println!(
            "count[{}] = count[{}] + count[{}] → {:?}",
            v,
            v,
            v - 1,
            count
        );
    }

    println!("完成 prefix sum count = {:?}", count);
    println!("（此時 count[v] 表示：<= v 的元素數量）");

    // 3️⃣ 建立 output array（穩定排序，從右到左）
    let mut output = vec![0usize; arr.len()];
    println!("--------------------------------");
    println!("開始建立 output（從右到左，確保穩定）");

    for &x in arr.iter().rev() {
        println!("處理元素 x = {}", x);

        count[x] -= 1;
        let pos = count[x];

        println!(
            "count[{}] -= 1 → 新位置 pos = {}",
            x, pos
        );

        output[pos] = x;
        println!("output[{}] = {} → {:?}", pos, x, output);
    }

    // 4️⃣ 複製回原陣列
    arr.copy_from_slice(&output);
    println!("--------------------------------");
    println!("排序完成 arr = {:?}", arr);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_counting_sort() {
        let mut v = vec![4usize, 2, 2, 8, 3, 3, 1];
        counting_sort(&mut v, 8);
        assert_eq!(v, vec![1, 2, 2, 3, 3, 4, 8]);
    }
}
