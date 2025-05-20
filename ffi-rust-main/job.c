#include <stdint.h>

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

void do_the_job()
{
    for (int i = 0; i < 1000000; i++)
    {
        free_vec(build_vec(1000));
    }
}