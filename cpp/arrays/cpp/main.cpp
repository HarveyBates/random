#include <iostream>

int ArraySizeError(int *test){
	// Won't work as its getting the size of a 
	// pointer to the array "test"
	return sizeof(test) / sizeof(test[0]);
}

template <size_t n>
void PrintArray(int (&test)[n]){
	// Correct method of getting an arrays size from
	// within a function
	for(int i = 0; i < n; i++){
		printf("%d\n", test[i]);
	}
}

int main(){
	int test[10];
	for(int i = 0; i < sizeof(test) / sizeof(test[0]); i++){
		test[i] = i;
	}

	int s = ArraySizeError(test);
	printf("%d\n", s);

	PrintArray(test);

	return 0;
}

