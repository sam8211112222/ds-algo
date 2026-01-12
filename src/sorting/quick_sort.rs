// sorting/quick_sort.rs
// 快速排序 / Quick Sort (Lomuto partition)
//

// ✅ 複雜度 / Complexity
// - Best: O(n log n)      -> pivot 大致把陣列對半分
// - Average: O(n log n)   -> 隨機資料下通常表現很好
// - Worst: O(n^2)         -> pivot 每次都選到最小/最大（例如已排序且 pivot 固定取尾端）
// - Space: O(log n) ~ O(n)
//   - 平均遞迴深度約 log n
//   - 最差遞迴深度可能到 n（堆疊爆很深）
// - Stable: No            -> partition/交換會打亂相等元素順序
//
// ✅ 這個演算法在做什麼 / What it does
// Quick sort 的核心：
// 1) 選一個 pivot（這裡選 arr[hi]）
// 2) 做 partition：把「<= pivot」放左邊，「> pivot」放右邊
// 3) pivot 會被放到它的最終位置 p
// 4) 對左半邊、右半邊遞迴做同樣的事
//
// ✅ Lomuto partition（這版用的）怎麼分？
// - i: 指向「下一個應該放 <= pivot 的位置」
// - j: 從 lo 掃到 hi-1
//   - 若 arr[j] <= pivot：交換 arr[i] 和 arr[j]，然後 i++
// - 最後把 pivot（arr[hi]）跟 arr[i] 交換，pivot 就定位在 i
pub fn quick_sort<T: Ord + std::fmt::Debug>(arr: &mut [T]) {
    println!("quick_sort 呼叫 arr = {:?}", arr);
    if arr.len() <= 1 {
        println!("長度 <= 1，直接返回\n");
        return;
    }
    quick_sort_range(arr, 0, arr.len() - 1);
    println!("quick_sort 結束 arr = {:?}\n", arr);
}

fn quick_sort_range<T: Ord + std::fmt::Debug>(arr: &mut [T], lo: usize, hi: usize) {
    println!("quick_sort_range(lo={}, hi={}) arr={:?}", lo, hi, arr);

    if lo >= hi {
        println!("lo >= hi，停止\n");
        return;
    }

    let p = partition(arr, lo, hi);
    println!("pivot 定位在 index {}，arr={:?}", p, arr);

    if p > 0 {
        quick_sort_range(arr, lo, p - 1);
    }
    quick_sort_range(arr, p + 1, hi);
}

fn partition<T: Ord + std::fmt::Debug>(arr: &mut [T], lo: usize, hi: usize) -> usize {
    println!(
        "partition lo={}, hi={}, pivot={:?}",
        lo, hi, arr[hi]
    );

    let mut i = lo;
    for j in lo..hi {
        println!(
            "比較 arr[{}]={:?} 與 pivot={:?}",
            j, arr[j], arr[hi]
        );

        if arr[j] <= arr[hi] {
            println!("→ <= pivot，swap arr[{}] 與 arr[{}]", i, j);
            arr.swap(i, j);
            i += 1;
            println!("  arr = {:?}", arr);
        }
    }

    println!("最後 swap pivot 到正確位置 {}", i);
    arr.swap(i, hi);
    println!("partition 完成 arr={:?}", arr);

    i
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quick_sort() {
        let mut v = vec![10, 7, 8, 9, 1, 5];
        quick_sort(&mut v);
        assert_eq!(v, vec![1, 5, 7, 8, 9, 10]);
    }
}
