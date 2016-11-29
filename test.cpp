#include <algorithm>
#include <cstdio>
#include <x86intrin.h>
//#include <windows.h>

struct Item {
    int key;
    int value;
};

extern "C" void sortRoutine(Item *items, int count);

#define STR2(X) #X
#define STR(X) STR2(X)

int main() {
    int itemCount = 1000000;
    Item *items = new Item[itemCount];

    volatile unsigned long long start, end, best = -1;

    int numRuns = 100;
    for (int run = 0; run < numRuns; run++) {
        srand(12345);
        for (int n = 0; n < itemCount; n++) {
            items[n].key = rand();
            items[n].value = rand();
        }

        unsigned int dummy;
        start = __rdtscp(&dummy);
        sortRoutine(items, itemCount);
        end = __rdtscp(&dummy);

        // check it worked
        int prev = -1;
        for (int n = 0; n < itemCount; n++) {
            if (items[n].key < prev) {
                printf("ERROR: sort failed\n");
                return 1;
            }
            prev = items[n].key;
        }
        volatile unsigned long long d = end - start;
        best = std::min(d, best);
    }
    printf("%lld\n", best);
    return 0;
}
