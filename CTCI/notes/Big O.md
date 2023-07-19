### Time complexity
- best case, worst case, expected case
- big O describes the rate of increase
- thats why we drop constants

### Space complexity
- recursion adds up as it takes up call stack

### big O facts
- multiply vs add

### amortized time

- sometimes the algorithm may take more than the regular time 
- consider the case of array insertion the worse case we know is O(1)....but it is the amortized time the actual time may be O(n)

### log n runtimes
binary search aka concurrent division
base of the logs dont matter


### important questions
1. an array of strings , first sort each string then sort array, what is the time complexity?
   ```O(s*a(log a+ log s))```
	think about it 
	Hint : Don't forget about comparison
2. in case of trees and graphs give importance to which nodes it visits and how much time it visits


```java
public class Permutation {
    public static void permutation(String str) {
        permutation(str, "");
    }

    private static void permutation(String str, String prefix) {
        if (str.length() == 0) {
            System.out.println(prefix);
        } else {
            for (int i = 0; i < str.length(); i++) {
                String rem = str.substring(0, i) + str.substring(i + 1);
                permutation(rem, prefix + str.charAt(i));
            }
        }
    }

    public static void main(String[] args) {
        String input = "abc";
        permutation(input);
    }
}

```

what is the time complexity?

### recursion tree complexity

$$ N = branches^{depth} $$

### question answers

