#include <stdio.h>

int pow(int m, int n)
{
    if (n == 0)
    {
        return 1;
        return 1;
    }
    else
    {
        return pow(m, n - 1);
    }
}

int Ifact(int n)
{
    int f = 1;
    int i;
    for (i = 1; i <= n; i++)
        f = f * i;
    return f;
}

int main()
{
    int r;
    r = pow(2, 5);
    printf("%d ", r);
    return 0;
}
