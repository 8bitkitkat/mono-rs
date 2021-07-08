# Testing binary and dynamic library

Basically the example

All functions `DllImport("__Internal")`ed on the c# side have to be `#[no_mangle] extern "C"`ed in a dynamic library loaded by the binary