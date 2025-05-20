#include <stdint.h>
#include <time.h>
#include <stdio.h>

void *new_vec();
void push_to_vec(void *v, uint64_t i);
void free_vec(void *v);

void *build_vec(int n)
{
    void *vec = new_vec();
    for (int i = 0; i < n; i++)
    {
        push_to_vec(vec, i);
    }
    return vec;
}

int main()
{
    struct timespec start, end;
    timespec_get(&start, TIME_UTC);

    for (int i = 0; i < 1000000; i++)
    {
        free_vec(build_vec(1000));
    }

    timespec_get(&end, TIME_UTC);

    long long elapsed_ns = (end.tv_sec - start.tv_sec) * 1000000000LL + (end.tv_nsec - start.tv_nsec);
    printf("%lf seconds\n", ((double)elapsed_ns) / 1e9);

    return 0;
}