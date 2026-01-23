# Sorting Algorithms (Rust)

本資料夾包含多種排序演算法的 Rust 實作。  
每個演算法各自一個檔案與 function，方便學習、比較與除錯（含 println 教學版）。

---

## 排序演算法總整理表

| 演算法 | 比較排序 | 穩定 | 最佳時間 | 平均時間 | 最壞時間 | 空間 | **什麼情況特別快** | **什麼情況特別慢 / 不適合** | 需要先排序嗎 |
|---|---|---|---|---|---|---|---|---|---|
| **Insertion Sort** | 是 | 是 | O(n) | O(n²) | O(n²) | O(1) | ✔ 資料量小<br>✔ 幾乎已排序 | ✘ 資料量大<br>✘ 完全亂序 | ✔ 已排序會更快 |
| **Selection Sort** | 是 | 否 | O(n²) | O(n²) | O(n²) | O(1) | ✔ 交換成本很高時（交換次數少） | ✘ 幾乎所有實務情況 | ✘ 不影響 |
| **Bubble Sort** | 是 | 是 | O(n) | O(n²) | O(n²) | O(1) | ✔ 幾乎已排序（有 early exit） | ✘ 大多數情況 | ✔ 已排序會更快 |
| **Shell Sort** | 是 | 否 | ~O(n log n)\* | ~O(n^1.3~n^2) | O(n²) | O(1) | ✔ 中等資料量<br>✔ insertion 太慢時 | ✘ 需要穩定排序 | ✘ 不需要 |
| **Quick Sort** | 是 | 否 | O(n log n) | O(n log n) | O(n²) | O(log n)~O(n) | ✔ 一般隨機資料<br>✔ cache 友善 | ✘ 幾乎已排序（pivot 固定） | ✘ 不需要 |
| **Merge Sort** | 是 | 是 | O(n log n) | O(n log n) | O(n log n) | O(n) | ✔ 需要穩定排序<br>✔ 最壞情況保證 | ✘ 記憶體受限 | ✘ 不需要 |
| **Heap Sort** | 是 | 否 | O(n log n) | O(n log n) | O(n log n) | O(1) | ✔ 記憶體很小<br>✔ 要保證上界 | ✘ cache 效率差 | ✘ 不需要 |
| **Counting Sort** | 否 | 是 | O(n+k) | O(n+k) | O(n+k) | O(n+k) | ✔ 整數<br>✔ 值域 k 小 | ✘ k 很大（爆記憶體） | ✘ 不需要 |
| **Radix Sort** | 否 | 是 | O(d·n) | O(d·n) | O(d·n) | O(n+r) | ✔ 整數<br>✔ 位數固定 | ✘ 非數字資料 | ✘ 不需要 |

\* Shell Sort 複雜度取決於 gap 序列，通常只能給估計範圍。

---

## 補充說明（很重要）

### 1️⃣ 「什麼情況特別快 / 特別慢」在看什麼？

排序是否快，主要取決於：

- 資料量大小（n）
- 是否「幾乎已排序」
- 是否需要穩定排序
- 資料型態（整數 / 一般物件）
- 記憶體是否受限

👉 **沒有一個排序在所有情況下都是最好的**

---

### 2️⃣ 什麼叫「需要先排序嗎？」

這一欄的意思是：

> **資料「已經接近排序完成」時，這個演算法會不會變快？**

- ✔ 表示：已排序 / 幾乎排序會明顯加速
    - Insertion Sort
    - Bubble Sort（有 early exit）

- ✘ 表示：
    - 不管有沒有排序過，時間幾乎一樣
    - 例如 Selection / Merge / Heap / Counting / Radix

---

### 3️⃣ Stable（穩定排序）是什麼意思？

**定義一句話版（考試用）：**

> 若兩個元素的 key 相同，排序後仍保持原本的相對順序，則稱為穩定排序。

#### 例子

原始資料（照出現順序）：
(A, 90), (B, 80), (C, 90)

只依「分數」排序：

- **穩定排序結果**
- (B, 80), (A, 90), (C, 90)


- **不穩定排序可能結果**
- (B, 80), (C, 90), (A, 90)

👉 分數相同的 A、C 順序被換了 → 不穩定

---

### 為什麼穩定很重要？

✔ **多重排序（先 A 再 B）**

例如：
1. 先依「名字」排序
2. 再依「分數」排序（用穩定排序）

→ 結果會同時符合「分數優先、名字次序」

---


### 小 + 幾乎排序好 → Insertion Sort

**為什麼**
- Insertion Sort 的成本來自「元素需要往前移動多少」
- 幾乎排序好的資料 → 幾乎不用移動 → 接近 O(n)
- 資料量小時，即使 O(n²) 也不慢，且實作簡單、常數小

**時間複雜度**
- Best：O(n)
- Average / Worst：O(n²)

**特性**
- 穩定（Stable）
- In-place（O(1) 額外空間）

**常見考題關鍵字**
- nearly sorted
- small input size
- online sorting（資料一筆一筆來）

**陷阱**
- 資料量大又完全亂序 → 非常慢

---

### 一般情況最快 → Quick Sort

**為什麼**
- 平均時間複雜度 O(n log n)
- In-place，額外記憶體需求小
- 常數因子小，實務上通常最快

**時間複雜度**
- Best / Average：O(n log n)
- Worst：O(n²)

**特性**
- 不穩定（Unstable）
- In-place（遞迴 stack 另計）

**常見考題關鍵字**
- fastest on average
- divide and conquer
- pivot selection

**陷阱**
- 已排序資料 + 壞 pivot → 退化成 O(n²)
- 題目若強調「worst-case guarantee」，不要選 Quick

---

### 最壞也要保證快 → Merge Sort / Heap Sort

#### Merge Sort

**為什麼**
- 不論資料如何，時間複雜度永遠是 O(n log n)
- 適合不能接受效能退化的情況

**時間複雜度**
- Best / Average / Worst：O(n log n)

**特性**
- 穩定（Stable）
- 需要 O(n) 額外空間

**適合情境**
- 外部排序（External Sorting）
- 需要穩定排序
- 在意最壞情況

---

#### Heap Sort

**為什麼**
- 最壞情況仍為 O(n log n)
- 不需要額外陣列

**時間複雜度**
- Best / Average / Worst：O(n log n)

**特性**
- 不穩定
- In-place（O(1) 額外空間）

**適合情境**
- 記憶體受限
- 需要 worst-case 保證

**陷阱**
- 實務上常數較大，通常比 Quick 慢

---

### 要穩定排序 → Merge / Insertion / Counting / Radix

**穩定排序（Stable）定義**
- 若兩個元素 key 相同，排序後它們的相對順序不變

**為什麼重要**
- 多欄位排序（先排次要欄位，再排主要欄位）
- 保留原始時間順序、輸入順序

**各演算法為什麼穩定**
- Insertion：不會跨越相等元素
- Merge：合併時相等元素先取左邊
- Counting：使用累積計數 + 從後往前放
- Radix：每一輪都使用穩定排序

**陷阱**
- Quick / Heap 都是不穩定（考試很愛問）

---

### 整數 + 值域小 → Counting Sort

**為什麼**
- 不用比較，只用「值當索引」
- 時間複雜度與值域有關，而非 n log n

**時間複雜度**
- O(n + k)，k = 值域大小

**特性**
- 穩定（標準作法）
- 非比較式排序

**適合情境**
- 分數 0–100
- 年齡 0–120
- 字元 ASCII（0–255）

**陷阱**
- 值域很大（例如 0 ~ 10⁹）→ 不適合

---

### 整數 + 位數固定 → Radix Sort

**為什麼**
- 將整數拆成「位數」來排
- 每一位用穩定排序（通常是 Counting）

**時間複雜度**
- O(d × (n + b))
    - d：位數
    - b：基底（10、256）

**特性**
- 穩定
- 非比較式排序

**適合情境**
- 固定長度整數
- IP address
- Student ID
- Timestamp

**陷阱**
- 位數過多（長字串）→ 輪數太多反而慢

---

## 超快選擇口訣（考前用）

- **小 + 幾乎排序好** → Insertion
- **一般情況最快** → Quick
- **最壞也要保證快** → Merge / Heap
- **要穩定排序** → Merge / Insertion / Counting / Radix
- **整數 + 值域小** → Counting
- **整數 + 位數固定** → Radix

---

## 本專案實作說明

- 每個排序演算法一個 function
- 泛型排序使用 `T: Ord`
- Radix / Counting 僅適用整數

---



## 各排序演算法虛擬碼（Pseudo Code）
---

## 共用：swap 與列印工具

```text
// swap(a, i, j): 交換陣列 a 中 i 與 j 兩個位置的元素
// swap(a, i, j): swap elements at index i and j in array a
procedure swap(a, i, j)
    temp   ← a[i]          // 暫存 a[i] / temporary save a[i]
    a[i]   ← a[j]          // 把 a[j] 放到 a[i] / move a[j] into a[i]
    a[j]   ← temp          // 把暫存的放回 a[j] / put temp back into a[j]
end procedure
```

---

## 1. Insertion Sort（插入排序）

**想法：** 把左邊當成「已排序區」，一個一個從右邊抓數字，往左邊插入正確的位置。

```text
// insertionSort(a): 對陣列 a 做插入排序（由小到大）
// insertionSort(a): sort array a in ascending order using insertion sort
procedure insertionSort(a)
    n ← length(a)

    // 從第二個元素開始，依序把 a[i] 插入前面已排序區
    // start from second element, insert a[i] into the sorted prefix
    for i ← 1 to n - 1
        key ← a[i]           // 暫存要插入的值 / value to be inserted
        j   ← i - 1          // 從已排序區的最後一個開始比較 / start from end of sorted part

        // 只要前面的值比 key 大，就往右移動一格
        // shift elements to the right while they are greater than key
        while j ≥ 0 and a[j] > key
            a[j + 1] ← a[j]  // 往右移動 / shift right
            j ← j - 1        // 往更左邊繼續比較 / move left

        // 找到位置，把 key 插入
        // put key into the correct position
        a[j + 1] ← key
    end for
end procedure
```

---

## 2. Selection Sort（選擇排序）

**想法：** 每一趟在「尚未排序的區間」找出最小值，放到前面。

```text
// selectionSort(a): 對陣列 a 做選擇排序（由小到大）
// selectionSort(a): sort array a in ascending order using selection sort
procedure selectionSort(a)
    n ← length(a)

    // i 左側是已排序區，右側是未排序區
    // for each i, a[0..i-1] is sorted, a[i..n-1] is unsorted
    for i ← 0 to n - 2
        minIndex ← i        // 先假設位置 i 是最小值 / assume a[i] is minimum

        // 在 i+1..n-1 之間找更小的
        // search a smaller value in the remaining unsorted part
        for j ← i + 1 to n - 1
            if a[j] < a[minIndex]
                minIndex ← j
            end if
        end for

        // 如果找到更小的，就跟 i 交換
        // if found a smaller element, swap it with a[i]
        if minIndex ≠ i
            swap(a, i, minIndex)
        end if
    end for
end procedure
```

---

## 3. Bubble Sort（氣泡排序）

**想法：** 不斷「相鄰比較、需要就交換」，大元素會一趟一趟往右邊「浮上去」。
可以加上「若一整趟都沒有交換，就提前結束」（early exit）。

```text
// bubbleSort(a): 對陣列 a 做氣泡排序（由小到大）
// bubbleSort(a): sort array a in ascending order using bubble sort
procedure bubbleSort(a)
    n ← length(a)

    // 外層控制總共要跑幾趟
    // outer loop for passes
    for i ← 0 to n - 1
        swapped ← false      // 紀錄這一趟有沒有交換 / record if any swap happened

        // 每一趟把最大的值「冒泡」到右邊
        // in each pass, push the largest element to the end
        for j ← 0 to n - 2 - i
            // 比較相鄰兩個，如果順序錯誤就交換
            // compare neighbors and swap if out of order
            if a[j] > a[j + 1]
                swap(a, j, j + 1)
                swapped ← true
            end if
        end for

        // 如果這一趟都沒有交換，代表已經完全排序好
        // if no swap in this pass, the array is already sorted
        if swapped = false
            break
        end if
    end for
end procedure
```

---

## 4. Shell Sort（希爾排序）

**想法：**
先用較大的「gap 間隔」做類似 insertion sort，讓資料先大致有序，再逐漸縮小 gap 到 1（最後就是一般的 insertion sort 但很快）。

```text
// shellSort(a, gaps): 對陣列 a 做希爾排序
// gaps 是一串間隔（例如 n/2, n/4, ..., 1）
// shellSort(a, gaps): sort array a using shell sort with given gap sequence
procedure shellSort(a, gaps)
    n ← length(a)

    // 依序使用每一個 gap
    // use each gap in the given sequence
    for each gap in gaps
        // 對每一個子序列做「間隔為 gap 的插入排序」
        // perform gapped insertion sort for this gap
        for i ← gap to n - 1
            temp ← a[i]         // 要插入的值 / value to be inserted
            j    ← i

            // 像 insertion sort 一樣往前比較，但步長是 gap
            // shift elements of the sorted subarray by step size gap
            while j - gap ≥ 0 and a[j - gap] > temp
                a[j] ← a[j - gap]
                j ← j - gap
            end while

            // 把值插入正確位置
            // put temp into correct spot
            a[j] ← temp
        end for
    end for
end procedure
```

> 若你不想自己設 gap，可以簡單用：`gaps = [⌊n/2⌋, ⌊n/4⌋, ..., 1]`。

---

## 5. Quick Sort（快速排序）

**想法：**
選一個 pivot，把小於 pivot 的放左邊，大於等於 pivot 的放右邊，然後遞迴處理左右兩邊。

這裡用「原地 partition」的典型寫法（Lomuto 版本，比較好讀）。

```text
// partition(a, low, high): 以 a[high] 當作 pivot，把陣列切成兩邊
// 回傳 pivot 最後所在的 index
// partition(a, low, high): use a[high] as pivot, partition array, return pivot index
function partition(a, low, high)
    pivot ← a[high]      // 選最後一個元素當 pivot / choose last element as pivot
    i ← low - 1          // i 會指向「目前為止最後一個小於 pivot 的位置」
                          // i is the last index of "less than pivot" region

    for j ← low to high - 1
        // 如果 a[j] 比 pivot 小，應該被放在左區域
        // if a[j] < pivot, it belongs to the left region
        if a[j] < pivot
            i ← i + 1
            swap(a, i, j)
        end if
    end for

    // 迴圈結束後，把 pivot 放到中間（i+1 位置）
    // place pivot after the "less than" region
    swap(a, i + 1, high)

    return i + 1          // pivot 的最終位置 / final index of pivot
end function

// quickSort(a, low, high): 對 a[low..high] 做快速排序
// quickSort(a, low, high): quick sort on subarray a[low..high]
procedure quickSort(a, low, high)
    if low < high
        // 先做 partition，取得 pivot 的最終位置 p
        // partition first, get pivot index p
        p ← partition(a, low, high)

        // 遞迴排序 pivot 左邊與右邊
        // recursively sort elements before and after pivot
        quickSort(a, low, p - 1)
        quickSort(a, p + 1, high)
    end if
end procedure
```

---

## 6. Merge Sort（合併排序）

**想法：**
分：一直把陣列一半一半切到只剩 1 個元素。
合：再把兩個已排序的小陣列合併成一個大的排序陣列。

```text
// merge(left, right): 合併兩個已排序的陣列成一個新的排序陣列
// merge(left, right): merge two sorted arrays into a new sorted array
function merge(left, right)
    result ← empty list
    i ← 0
    j ← 0

    // 同時從 left 和 right 的開頭往後掃
    // walk over both arrays from the beginning
    while i < length(left) and j < length(right)
        if left[i] ≤ right[j]
            append left[i] to result   // left 比較小，放進 result / left smaller, push it
            i ← i + 1
        else
            append right[j] to result  // right 比較小，放進 result / right smaller, push it
            j ← j + 1
        end if
    end while

    // 把剩下沒用完的全部接上
    // append remaining elements
    while i < length(left)
        append left[i] to result
        i ← i + 1
    end while

    while j < length(right)
        append right[j] to result
        j ← j + 1
    end while

    return result
end function

// mergeSort(a): 回傳排序好的新陣列（也可以改成 in-place 版本）
// mergeSort(a): return a new sorted array (can be adapted to in-place)
function mergeSort(a)
    n ← length(a)
    if n ≤ 1
        return a           // 0 或 1 個元素，本來就已排序 / already sorted
    end if

    mid ← ⌊n / 2⌋
    left  ← a[0 .. mid - 1]    // 左半邊 / left half
    right ← a[mid .. n - 1]    // 右半邊 / right half

    // 遞迴地對左右做 merge sort
    // recursively sort both halves
    sortedLeft  ← mergeSort(left)
    sortedRight ← mergeSort(right)

    // 再把兩邊合併
    // merge the two sorted halves
    return merge(sortedLeft, sortedRight)
end function
```

---

## 7. Heap Sort（堆積排序）

**想法：**

1. 先把陣列建成「最大堆」(max-heap)。
2. 重複地把最大值（堆頂）拿出來，放到陣列尾巴，然後重新調整剩下的堆。

```text
// heapify(a, n, i): 在陣列 a 的前 n 個元素中，讓以 i 為根的子樹滿足最大堆
// heapify(a, n, i): ensure subtree rooted at i is a max-heap within first n elements
procedure heapify(a, n, i)
    largest ← i         // 先假設根 i 最大 / assume root i is the largest
    left   ← 2 * i + 1  // 左子節點 index / left child index
    right  ← 2 * i + 2  // 右子節點 index / right child index

    // 如果左子節點存在，且比目前 largest 還大
    // if left child exists and is greater than current largest
    if left < n and a[left] > a[largest]
        largest ← left
    end if

    // 如果右子節點存在，且比目前 largest 還大
    // if right child exists and is greater than current largest
    if right < n and a[right] > a[largest]
        largest ← right
    end if

    // 如果 largest 不是根，代表要把最大值拉到根，並且往下修正
    // if largest changed, swap with root and heapify the affected subtree
    if largest ≠ i
        swap(a, i, largest)
        heapify(a, n, largest)
    end if
end procedure

// heapSort(a): 對陣列 a 做堆積排序（由小到大）
// heapSort(a): sort array a in ascending order using heap sort
procedure heapSort(a)
    n ← length(a)

    // 第一步：建堆（從最後一個非葉節點往上 heapify）
    // step 1: build a max-heap
    for i ← ⌊n / 2⌋ - 1 downto 0
        heapify(a, n, i)
    end for

    // 第二步：反覆把堆頂（最大值）換到陣列尾端，再縮小堆的大小
    // step 2: repeatedly move current max to the end and fix the heap
    for i ← n - 1 downto 1
        // 把最大值 a[0] 與 a[i] 交換
        // swap max element at root with element at i
        swap(a, 0, i)

        // 對縮小後的堆（0..i-1）重新 heapify
        // restore max-heap in reduced array (size i)
        heapify(a, i, 0)
    end for
end procedure
```

---

## 8. Counting Sort（計數排序）

**想法：**
適合「整數、且值域 k 不大」的情況。
步驟：

1. 找出最小值 min 與最大值 max（處理有負數的狀況）。
2. 開一個長度為 `k = max - min + 1` 的計數陣列 count。
3. 計算每個值出現幾次。
4. 用 prefix sum 把 count 轉成「該值最終應該放到哪個 index 前一格」。
5. 從右往左掃原陣列，把元素依序放到 output 陣列中（確保穩定性）。

```text
// countingSort(a): 對整數陣列 a 做穩定的計數排序
// countingSort(a): stable counting sort for integer array a
function countingSort(a)
    n ← length(a)
    if n = 0
        return a
    end if

    // 找最小值與最大值（考慮可能有負數）
    // find min and max (support negative integers)
    minVal ← a[0]
    maxVal ← a[0]
    for i ← 1 to n - 1
        if a[i] < minVal
            minVal ← a[i]
        end if
        if a[i] > maxVal
            maxVal ← a[i]
        end if
    end for

    k ← maxVal - minVal + 1  // 值域大小 / range size

    // 建立計數陣列並初始化為 0
    // create count array filled with 0
    count[0..k-1] ← 0

    // 第一步：計數每個值出現幾次
    // step 1: count occurrences
    for i ← 0 to n - 1
        index ← a[i] - minVal   // 將值平移到 [0..k-1]
        count[index] ← count[index] + 1
    end for

    // 第二步：做 prefix sum，讓 count[x] 表示「<= x 的元素數量」
    // step 2: prefix sum so that count[x] = number of elements ≤ value x
    for i ← 1 to k - 1
        count[i] ← count[i] + count[i - 1]
    end for

    // 第三步：從右往左掃，將元素放到正確位置，確保穩定
    // step 3: fill output array from right to left for stability
    output[0..n-1]  // 建一個輸出陣列 / create output array
    for i ← n - 1 downto 0
        val   ← a[i]
        index ← val - minVal
        pos   ← count[index] - 1  // 該 val 最後應該放的 index
        output[pos] ← val
        count[index] ← count[index] - 1
    end for

    return output
end function
```

---

## 9. Radix Sort（基數排序，LSD）

**想法：**
以個位數、十位數、百位數……為順序，一位一位做穩定排序（通常用 counting sort 當子程序）。
下面用「非負整數，10 進位，LSD（從個位數開始）」的版本。

```text
// getDigit(num, d): 取得 num 在 10 進位下，第 d 位的數字
// d = 0 表示個位數, d = 1 表示十位數, 以此類推
// getDigit(num, d): get the d-th digit of num in base 10, d=0 is ones
function getDigit(num, d)
    return (num / 10^d) mod 10   // 整數除法 + 取餘數 / integer division and modulo
end function

// countingSortByDigit(a, d): 依照第 d 位數字做穩定計數排序
// countingSortByDigit(a, d): stable counting sort by digit d
function countingSortByDigit(a, d)
    n ← length(a)
    base ← 10                 // 0~9 共 10 種數字 / base 10
    count[0..base-1] ← 0

    // 計數每一個 digit 出現次數
    // count occurrences of each digit
    for i ← 0 to n - 1
        digit ← getDigit(a[i], d)
        count[digit] ← count[digit] + 1
    end for

    // prefix sum：count[d] 表示 <= d 的 digit 數量
    // prefix sum so count[d] = number of elements with digit ≤ d
    for i ← 1 to base - 1
        count[i] ← count[i] + count[i - 1]
    end for

    // 從右往左，依照該位 digit 放入輸出陣列（維持穩定）
    // place elements from right to left for stability
    output[0..n-1]
    for i ← n - 1 downto 0
        digit ← getDigit(a[i], d)
        pos   ← count[digit] - 1
        output[pos] ← a[i]
        count[digit] ← count[digit] - 1
    end for

    return output
end function

// radixSort(a, maxDigits): 對非負整數陣列 a 做 LSD 基數排序
// maxDigits 是最大位數（例如最大值是 999 -> maxDigits = 3）
// radixSort(a, maxDigits): LSD radix sort for non-negative integers
function radixSort(a, maxDigits)
    // 從個位數 (d=0) 一直到最高位數 (d=maxDigits-1)
    // from least significant digit to most significant digit
    for d ← 0 to maxDigits - 1
        a ← countingSortByDigit(a, d)
    end for

    return a
end function
```

---







