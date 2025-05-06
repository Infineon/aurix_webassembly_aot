//#include "emscripten.h"

extern void __write__(int* address, int value);
extern int __read__(int* address);

static int big_test;

//EMSCRIPTEN_KEEPALIVE
int  test(){
    big_test = 0;
    __write__(&big_test,0xdeadbeef);

    int read_value = __read__(&big_test);

    return read_value == 0xdeadbeef ? 0: -1;

}

int _start(){
    return test();
}