// C++ version (explicit stack management)
#include <algorithm>

struct Item {
    int key;
    int value;
};
struct Pair {
    Item *items;
    unsigned count;
};

static Pair stack[1000];

extern "C" void sortRoutine(Item *items, unsigned count) {
    Pair *top = stack;

    count--;
    if (count <= 0)
        return;

    *top++ = {0, 0};

    while (true) {
        // Pick the pivot.
        Item pivot = items[count];
        unsigned low = 0;
        for (unsigned pos = 0; pos < count; pos += 1) {
            if (items[pos].key <= pivot.key) {
                // swap elements
                std::swap(items[low], items[pos]);
                low++;
            }
        }

        // move pivot into place
        items[count] = items[low];
        items[low] = pivot;

        // recurse
        if (low + 1 < count) {
            *top++ = {&items[low + 1], count - low - 1};
        }

        // move to left side
        if (low > 1) {
            count = low - 1;
            continue;
        }

        // no left side, pop off stack
        Pair p = *--top;
        items = p.items;
        count = p.count;
        if (!items)
            break;
    }
}
