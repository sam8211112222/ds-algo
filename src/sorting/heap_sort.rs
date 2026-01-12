// sorting/heap_sort.rs
// 堆積排序 / Heap Sort
//
// ✅ 複雜度 / Complexity
// - Best: O(n log n)
// - Average: O(n log n)
// - Worst: O(n log n)
// - Space: O(1)           -> 原地
// - Stable: No            -> swap 會打亂相等元素順序
//
// ✅ 這個演算法在做什麼 / What it does
// Heap sort 使用「最大堆 (max-heap)」：
// - 最大堆性質：父節點 >= 子節點，所以堆頂 arr[0] 永遠是最大值
//
// 主要分兩段：
// 1) Build max-heap：把整個陣列調整成最大堆（O(n)）
// 2) Repeated extract max：
//    - 把最大值（arr[0]）和尾端 arr[end] 交換
//    - end--（尾端變成已排序區）
//    - 再對新的堆頂做 sift_down（恢復最大堆性質，O(log n)）
//    - 重複直到 end==0
pub fn heap_sort<T: Ord + std::fmt::Debug>(arr: &mut [T]) {
    let n = arr.len();
    println!("原始陣列 arr = {:?}", arr);

    if n <= 1 {
        return;
    }

    println!("\n=== Step 1: Build max heap ===");

    // 建最大堆
    for i in (0..=(n / 2)).rev() {
        println!("\n[Build heap] 對 root = {} 做 sift_down", i);
        sift_down(arr, i, n);
        println!("目前 heap 狀態 = {:?}", arr);
    }

    println!("\n=== Step 2: Extract max repeatedly ===");

    // 不斷把最大值丟到尾端
    for end in (1..n).rev() {
        println!("\n[Extract max]");
        println!("swap arr[0] = {:?} 與 arr[{}] = {:?}", arr[0], end, arr[end]);
        arr.swap(0, end);
        println!("swap 後 arr = {:?}", arr);

        println!("對新的 root = 0 做 sift_down（heap_size = {}）", end);
        sift_down(arr, 0, end);
        println!("heap 調整後 = {:?}", arr);
        println!("已排序區 = arr[{}..]", end);
    }

    println!("\n排序完成 arr = {:?}", arr);
}

fn sift_down<T: Ord + std::fmt::Debug>(
    arr: &mut [T],
    mut root: usize,
    heap_size: usize,
) {
    loop {
        let left = 2 * root + 1;
        let right = left + 1;

        println!(
            "  root={}, left={}, right={}, heap_size={}",
            root, left, right, heap_size
        );

        // 如果沒有左子節點，結束
        if left >= heap_size {
            println!("  left 超出 heap_size，停止");
            break;
        }

        // 選較大的子節點
        let mut child = left;
        if right < heap_size && arr[right] > arr[left] {
            println!(
                "  右子節點 {:?} > 左子節點 {:?}，選 right",
                arr[right], arr[left]
            );
            child = right;
        } else if right < heap_size {
            println!(
                "  左子節點 {:?} >= 右子節點 {:?}，選 left",
                arr[left], arr[right]
            );
        } else {
            println!("  只有左子節點 {:?}", arr[left]);
        }

        // 比較 root 與較大的 child
        if arr[child] > arr[root] {
            println!(
                "  child {:?} > root {:?} → swap",
                arr[child], arr[root]
            );
            arr.swap(root, child);
            println!("  swap 後 arr = {:?}", arr);
            root = child;
        } else {
            println!(
                "  root {:?} >= child {:?}，heap 性質成立，停止",
                arr[root], arr[child]
            );
            break;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_heap_sort() {
        let mut v = vec![4, 10, 3, 5, 1];
        heap_sort(&mut v);
        assert_eq!(v, vec![1, 3, 4, 5, 10]);
    }
}
