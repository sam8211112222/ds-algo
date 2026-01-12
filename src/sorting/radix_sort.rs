// sorting/radix_sort.rs
// 基數排序 / Radix Sort (LSD, base 10)
//
// ✅ 複雜度 / Complexity
// - Best: O(d * n)
// - Average: O(d * n)
// - Worst: O(d * n)
//   d = 位數（digit count），n = 元素數量
// - Space: O(n + r)
//   n = output buffer
//   r = base（這裡是 10，所以 count 是固定大小 10）
// - Stable: Yes
//   因為每一輪用的 counting-by-digit 是穩定的（從右到左填 output）
//
// ✅ 這個演算法在做什麼 / What it does
// LSD（Least Significant Digit first）：從「最低位」開始排
// 例：170, 45, 75, 90, 802, 24, 2, 66
// - 第 1 輪：按「個位數」排序（0~9）
// - 第 2 輪：按「十位數」排序
// - 第 3 輪：按「百位數」排序
// ...
// 因為每一輪都是「穩定排序」，所以高位排序時不會破壞低位已建立的順序
//
// ✅ 注意
// - 這裡只實作 u32（非負整數）
// - base=10 方便讀懂；實務上也常用 base=256（byte）提升效率
pub fn radix_sort_u32(arr: &mut [u32]) {
    println!("radix_sort 開始 arr = {:?}", arr);

    if arr.len() <= 1 {
        return;
    }

    let mut max_val = 0u32;
    for &x in arr.iter() {
        if x > max_val {
            max_val = x;
        }
    }

    let n = arr.len();
    let mut output = vec![0u32; n];
    let mut exp: u32 = 1;

    while max_val / exp > 0 {
        println!("\n=== 依第 {} 位排序 ===", exp);

        let mut count = [0usize; 10];

        for &x in arr.iter() {
            let digit = ((x / exp) % 10) as usize;
            count[digit] += 1;
        }
        println!("count（次數）= {:?}", count);

        for i in 1..10 {
            count[i] += count[i - 1];
        }
        println!("count（prefix sum）= {:?}", count);

        for &x in arr.iter().rev() {
            let digit = ((x / exp) % 10) as usize;
            count[digit] -= 1;
            output[count[digit]] = x;
            println!(
                "放 {} 到 output[{}] → {:?}",
                x, count[digit], output
            );
        }

        arr.copy_from_slice(&output);
        println!("本輪結果 arr = {:?}", arr);

        exp *= 10;
    }

    println!("\nradix_sort 完成 arr = {:?}", arr);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_radix_sort_u32() {
        let mut v = vec![170u32, 45, 75, 90, 802, 24, 2, 66];
        radix_sort_u32(&mut v);
        assert_eq!(v, vec![2, 24, 45, 66, 75, 90, 170, 802]);
    }
}
