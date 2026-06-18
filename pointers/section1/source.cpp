#include <iostream>
#include <algorithm>
#include <array>
void Print(int *ptr, int size)
{
	for (int i = 0; i < size; ++i)
	{
		std::cout << ptr[i] << ' ';
		// std::cout << *(ptr + i) << ' ';
	}
}

template <typename T, int size>
void Print(T (&ref)[size])
{
	for (int i = 0; i < size; ++i)
	{
		std::cout << ref[i] << ' ';
		// std::cout << *(ptr + i) << ' ';
	}
}

template <typename T, int size>
T *Begin(T (&ref)[size])
{
	return ref;
}

template <typename T, int size>
T *End(T (&ref)[size])
{
	return (ref + size);
}

void Print(std::array<int, 5> &arr)
{
	std::sort(arr.begin(), arr.end());
	// for (int i = 0; i < arr.size(); ++i) {
	//	std::cout << arr[i] << ' ';
	//	//std::cout << *(ptr + i) << ' ';
	// }
	for (const auto &x : arr)
	{
		std::cout << x << ' ';
	}
}
int main()
{
	std::array<int, 5> arr{4, 1, 7, 9, 0};
	// arr.size();
	// arr[0];
	// arr.at(8);
	Print(arr);
	int arr1[5];
	int arr2[5]{};
	int arr3[5]{3, 1, 6, 8, 0};

	// arr3[0] = 100;
	// int(&ref)[5] = arr3;

	// int *p = std::begin(arr3);
	// int *end = std::end(arr3);
	//*(p + 7) = 800;
	// p[2] = 700;
	// std::sort(std::begin(arr3), std::end(arr3));
	////Print(arr3, 10);
	// Print(arr3);
	return 0;
}
