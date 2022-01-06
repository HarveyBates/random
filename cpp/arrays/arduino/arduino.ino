int iArr[10];
char cArr[11];
long lArr[12];
float fArr[13];

// Must be forward declared (in the header if available)
template <size_t SIZE, class T> size_t ArraySize(T (&arr)[SIZE]);

void setup(){

	Serial.begin(9600);
	delay(200);

	// This results in a size of 1
	//int s = ArrSizeIncorrect(arr);
	//Serial.println(s);

	// Examples
	int ss = ArraySize(iArr);
	Serial.println(ss);

	ss = ArraySize(cArr);
	Serial.println(ss);

	ss = ArraySize(lArr);
	Serial.println(ss);

	ss = ArraySize(fArr);
	Serial.println(ss);

}

// INCORRECT!
int ArraySizeIncorrect(int *arr){
	// Results in sizeof() returning the size of a pointer to arr
	return sizeof(arr) / sizeof(arr[0]);
}


template <size_t SIZE, class T> size_t ArraySize(T (&arr)[SIZE]){
	return SIZE;
}

void loop(){

}

