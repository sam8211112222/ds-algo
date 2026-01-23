const testData = [5, 2, 9, 1, 5, 6];

function insertionSort(testData) {
    const list = [...testData];
    console.log("Insertion Sort start, input:", list);

    for (let i = 1; i < list.length; i++) {
        const current = list[i];
        let j = i - 1;

        console.log("\n==============================");
        console.log(`外層迴圈 i = ${i}, current = ${current}`);
        console.log("當前陣列 / current array:", list);

        // 將比 current 大的元素往右移
        // move all elements greater than current one position to the right
        while (j >= 0 && list[j] > current) {
            console.log(
                `  比較 list[${j}] = ${list[j]} 和 current = ${current} → 需要右移 / need shift`
            );

            // 右移 / shift right
            list[j + 1] = list[j];
            console.log(
                `  將 list[${j}] 移到 list[${j + 1}] 之後 / after shift:`,
                list
            );

            j--;
        }

        // 這裡的 j 已經停在「比 current 小或等於」的那個位置
        // 或者 j = -1（代表 current 是目前最小的）
        // here j stops at the last element <= current, or -1 if current is smallest
        const insertPos = j + 1;
        list[insertPos] = current;
        console.log(
            `  將 current = ${current} 插入到 index ${insertPos} / insert position`
        );
        console.log("  插入後陣列 / array after insert:", list);
    }
    
    return list;

}
console.log("Insertion Sort end, output:", insertionSort(testData));