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



