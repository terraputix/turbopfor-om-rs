#include <stdio.h>
#include <stdint.h>
#include <string.h>
#include "ic.h" // Include the header file for the library

void test_round_trip()
{
    const size_t n = 3;
    uint16_t nums[] = {33, 44, 77};
    uint8_t compressed[1000] = {0};
    uint16_t recovered[100] = {0}; // Larger array to detect overflows

    // Initialize recovered array with a known value
    for (size_t i = 0; i < 100; i++)
    {
        recovered[i] = 0xFFFF;
    }

    // Call the encoding and decoding functions
    p4nzenc128v16(nums, n, compressed);
    p4nzdec128v16(compressed, n, recovered);

    // Compare the recovered data with the original input data
    if (memcmp(nums, recovered, n * sizeof(uint16_t)) == 0)
    {
        printf("Test passed: recovered data matches original data.\n");
    }
    else
    {
        printf("Test failed: recovered data does not match original data.\n");
    }

    // Check for overflow by verifying that elements beyond n are unchanged
    int overflow_detected = 0;
    for (size_t i = n; i < 100; i++)
    {
        if (recovered[i] != 0xFFFF)
        {
            overflow_detected = 1;
            printf("Overflow detected: recovered[%zu] = %u\n", i, recovered[i]);
        }
    }

    if (!overflow_detected)
    {
        printf("No overflow detected: elements beyond n are unchanged.\n");
    }
}

int main()
{
    test_round_trip();
    return 0;
}
