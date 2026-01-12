// sorting/merge_sort.rs
// 合併排序 / Merge Sort
//
// ✅ 複雜度 / Complexity
// - Best: O(n log n)
// - Average: O(n log n)
// - Worst: O(n log n)
// - Space: O(n)           -> 合併時需要額外 buffer
// - Stable: Yes           -> 合併時相等元素保持原順序（此版本用 <= 先取左邊）
//
// ✅ 這個演算法在做什麼 / What it does
// Merge sort 的核心：分而治之（Divide & Conquer）
// 1) 把陣列切成左右兩半
// 2) 對左半、右半分別排序（遞迴）
// 3) 把兩個「已排序」的陣列合併成一個排序好的陣列
//
// ✅ 為什麼需要 Clone？
// 因為合併時會把元素 push 到 buffer，最後再 copy 回原陣列。
pub fn merge_sort<T: Ord + Clone + std::fmt::Debug>(arr: &mut [T]) {
    println!("merge_sort 呼叫，arr = {:?}", arr);

    let n = arr.len();
    if n <= 1 {
        println!("長度 <= 1，直接返回\n");
        return;
    }

    let mid = n / 2;
    println!("切成 left={:?}, right={:?}", &arr[..mid], &arr[mid..]);

    merge_sort(&mut arr[..mid]);
    merge_sort(&mut arr[mid..]);

    println!("開始合併 left={:?}, right={:?}", &arr[..mid], &arr[mid..]);

    let mut buf = Vec::with_capacity(n);
    let (left, right) = arr.split_at(mid);

    let mut i = 0;
    let mut j = 0;

    while i < left.len() && j < right.len() {
        println!(
            "比較 left[{}]={:?} 與 right[{}]={:?}",
            i, left[i], j, right[j]
        );

        if left[i] <= right[j] {
            println!("→ 取 left");
            buf.push(left[i].clone());
            i += 1;
        } else {
            println!("→ 取 right");
            buf.push(right[j].clone());
            j += 1;
        }
    }

    while i < left.len() {
        println!("left 剩餘 {:?}，直接加入", left[i]);
        buf.push(left[i].clone());
        i += 1;
    }

    while j < right.len() {
        println!("right 剩餘 {:?}，直接加入", right[j]);
        buf.push(right[j].clone());
        j += 1;
    }

    println!("合併完成 buf = {:?}", buf);
    arr.clone_from_slice(&buf);
    println!("寫回 arr = {:?}\n", arr);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merge_sort() {
        let mut v = vec![12, 11, 13, 5, 6, 7];
        merge_sort(&mut v);
        assert_eq!(v, vec![5, 6, 7, 11, 12, 13]);
    }
}
