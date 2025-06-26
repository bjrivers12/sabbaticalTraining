#include <stdio.h>

int pow(int m, int n)
{
    if (n == 0)
    {
        return 1;
    }
    else
    {
        return pow(m, n - 1);
    }
}

int main()
{
    int r;
    r = pow(2, 5);
    printf("%d ", r);
    return 0;
}
