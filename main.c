#include <stdio.h>

int sum(int n)
{
    if (n > 100)
    {
        return n - 10;
    }
    else
    {
        return fun(fun(n + 11));
    }
}

int main()
{
    return 0;
}
