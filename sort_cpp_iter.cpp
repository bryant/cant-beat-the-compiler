// C++ version (explicit stack management)

struct Item {
    int key;
    int value;
};
struct Pair {
    Item *items;
    int count;
};

static Pair stack[1000];

extern "C" void sortRoutine(Item *items, int count) {
    Pair *top = stack;

    count--;
    if (count <= 0)
        return;

    *top++ = {0, 0};

    while (true) {
        // Pick the pivot.
        Item pivot = items[count];
        int low = 0;
        for (int pos = 0; pos < count; pos += 1) {
            if (items[pos].key <= pivot.key) {
                // swap elements
                Item t0 = items[low];
                Item t1 = items[pos];
                items[low] = t1;
                items[pos] = t0;
                low++;
            }
        }

        // move pivot into place
        Item tmp = items[low];
        items[count] = tmp;
        items[low] = pivot;

        // recurse
        count -= low;
        count--;
        if (count > 0) {
            // push right side
            *top++ = {items + low + 1, count};
        }

        // move to left side
        count = low - 1;
        if (count > 0)
            continue;

        // no left side, pop off stack
        Pair p = *--top;
        items = p.items;
        count = p.count;
        if (!items)
            break;
    }
}
