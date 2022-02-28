#include <stdio.h>
// #include <stdlib.h>
#define true 1

/* Returns index of the pivot in the modified vector, with unsorted greater elements to the right, lesser to the left
Worst case is greatest/least O(n^2), n-1 comparisons, best case median */
/*int partition(int left, int right, int array[])
{
    int pivot = array[right];

    // i = index of smaller element (memory)
    int i = left;

    // j = the current index
    for (int j = left; j < right; j++)
    {
        if (array[j] < pivot)
        {
            // swap the smaller and larger value
            int temp = array[i];
            array[i] = array[j];
            array[j] = temp;
            // swap(array[i], array[j]);
            i++;
        }
    }

    int temp = array[i];
    array[i] = array[right];
    array[right] = temp;
    // swap(array[i], array[right]);

    // swap pivot element (rightmost element) and its found position (sandwiched in the middle)
    return i;
}*/

/*int rand_pivot(int left, int right, int array[])
{
    // rand() % (max_number + 1 - minimum_number) + minimum_number
    return rand() % (right + 1 - left) + left;
}

int median(int a, int b, int c)
{
    // use switch instead

    // copied median
    if ((a <= b) && (b <= c))
        return b; // a b c
    if ((a <= c) && (c <= b))
        return c; // a c b
    if ((b <= a) && (a <= c))
        return a; // b a c
    if ((b <= c) && (c <= a))
        return c; // b c a
    if ((c <= a) && (a <= b))
        return a; // c a b
    return b;     // c b a

    /* if (a < b)
    {
        if (b < c)
        {
            return b;
        }
        else if (a < c)
        {
            return c;
        }
        else
        {
            return a;
        }
    }
    if (a < c)
    {
        return a;
    }
    else if (b < c)
    {
        return c;
    }
    else
    {
        return b;
    } 
}

int pick_pivot(int left, int right, int array[])
{
    int candidate1 = array[rand_pivot(left, right, array)];
    int candidate2 = array[rand_pivot(left, right, array)];
    int candidate3 = array[rand_pivot(left, right, array)];
    return median(candidate1, candidate2, candidate3);
}*/

int insertion_sort(int left, int right, int array[])
{
    for (int i = left + 1; i < right + 1; i++)
    {
        int key = array[i];
        int j = i - 1;

        while (j >= left && array[j] > key)
        {
            array[j + 1] = array[j];
            j--; 
        }

        array[j + 1] = key;
    }
}

int hoares_partition(int left, int right, int array[])
{
    int pivot = array[(left + right) / 2]; // pick_pivot(left, right, array);

    int i = left - 1;
    int j = right + 1;

    while (true)
    {
        do
        {
            i += 1;
        } while (array[i] < pivot);

        do
        {
            j -= 1;
        } while (array[j] > pivot);

        if (i >= j)
        {
            return j;
        }

        // swap
        // swap(array[i], array[j]);
        int temp = array[i];
        array[i] = array[j];
        array[j] = temp;
    }
}

/* Recursive subroutine that chops the vector at the pivot
Choosing a better pivot is pivotal, but this implementation picks the last element */
// https://alienryderflex.com/quicksort/
void quick_sort(int left, int right, int array[])
{
    // if it's greater than, do nothing because it's already sorted
    if (left < right)
    {
        if (right - left < 24)
        {
            insertion_sort(left, right, array);
        }
        else
        {
            int pivot_index = hoares_partition(left, right, array);

            // An optimization is to, if left is smaller, handle right iteratively 

            // Sort the left vector (all values lesser than the pivot)
            // right is length -1 if indexing starts at zero
            quick_sort(left, pivot_index, array);

            // Sort the right vector (all values greater than the pivot.
            quick_sort(pivot_index + 1, right, array);
        }
    }
}

int main()
{
    int num_elements;

    // read input to array
    // cin >> num_elements;

    // int debug_array[] = {1, 25, 2, 3, 5, 10};

    // quick_sort(0, 5, debug_array);

    scanf("%d", &num_elements);

    int array[num_elements];

    for (int i = 0; i < num_elements; i++)
    {
        scanf("%d", &array[i]);
    }

    // num_elements - 1 because index
    quick_sort(0, num_elements - 1, array);

    // print sorted array
    for (int i = 0; i < num_elements; i++)
    {
        printf("%d ", array[i]);
        // cout << array[i]; //<< " ";
    }

    return 0;

    /* getline(std::cin, line);
    istringstream stream(line);
    while (stream >> number)
        array.push_back(number);

    size_t length = sizeof array / sizeof array[0];

    quick_sort(0, length, array); */
}
