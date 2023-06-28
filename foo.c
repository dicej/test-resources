// Generated by `wit-bindgen` 0.8.0. DO NOT EDIT! Just kidding; edit all you want.
#include "foo.h"

imports_y_borrowed_t imports_y_borrow(imports_y_owned_t y_owned) {
    imports_y_borrowed_t y_borrowed;
    y_borrowed.__handle = y_owned.__handle;
    return y_borrowed;
}

__attribute__((__import_module__("imports"), __import_name__("[resource-drop-own]y")))
void __wasm_import_imports_y_drop(int32_t);

void imports_y_drop_owned(imports_y_owned_t y) {
   __wasm_import_imports_y_drop(y.__handle);
}

__attribute__((__import_module__("imports"), __import_name__("[constructor]y")))
int32_t __wasm_import_imports_y_constructor(double);

__attribute__((__import_module__("imports"), __import_name__("[method]y.get-a")))
double __wasm_import_imports_y_get_a(int32_t);

__attribute__((__import_module__("imports"), __import_name__("[method]y.set-a")))
void __wasm_import_imports_y_set_a(int32_t, double);

__attribute__((__import_module__("imports"), __import_name__("[static]y.add")))
int32_t __wasm_import_imports_y_add(int32_t, double);

__attribute__((__weak__, __export_name__("cabi_realloc")))
void *cabi_realloc(void *ptr, size_t old_size, size_t align, size_t new_size) {
    (void) old_size;
    if (new_size == 0) return (void*) align;
    void *ret = realloc(ptr, new_size);
    if (!ret) abort();
    return ret;
}

imports_y_owned_t imports_y_constructor(double a) {
    imports_y_owned_t y;
    y.__handle = __wasm_import_imports_y_constructor(a);
    return y;
}

double imports_y_get_a(imports_y_borrowed_t y) {
    return __wasm_import_imports_y_get_a(y.__handle);
}

void imports_y_set_a(imports_y_borrowed_t y, double a) {
  __wasm_import_imports_y_set_a(y.__handle, a);
}

imports_y_owned_t imports_y_add(imports_y_owned_t y, double a) {
    imports_y_owned_t result;
    result.__handle = __wasm_import_imports_y_add(y.__handle, a);
    return result;
}

__attribute__((__export_name__("test")))
void __wasm_export_foo_test(void) {
  foo_test();
}

__attribute__((__import_module__("[export]exports"), __import_name__("[resource-new]x")))
int32_t __wasm_import_exports_x_new(int32_t);

exports_x_owned_t exports_x_new(exports_x_t *x) {
    exports_x_owned_t x_owned;
    x_owned.__handle = __wasm_import_exports_x_new((int32_t) x);
    return x_owned;
}

__attribute__((__import_module__("[export]exports"), __import_name__("[resource-rep]x")))
int32_t __wasm_import_exports_x_rep(int32_t);

exports_x_t *exports_x_owned_rep(exports_x_owned_t x) {
    return (exports_x_t *) __wasm_import_exports_x_rep(x.__handle);
}

__attribute__((__import_module__("[export]exports"), __import_name__("[resource-drop-own]x")))
void __wasm_import_exports_x_drop(int32_t);

void exports_x_drop_owned(exports_x_owned_t x) {
    __wasm_import_exports_x_drop(x.__handle);
}

__attribute__((__export_name__("exports#[constructor]x")))
int32_t __wasm_export_exports_x_constructor(double arg) {
    return exports_x_constructor(arg).__handle;
}

__attribute__((__export_name__("exports#[method]x.get-a")))
double __wasm_export_exports_x_get_a(exports_x_t *x) {
    return exports_x_get_a(x);
}

__attribute__((__export_name__("exports#[method]x.set-a")))
void __wasm_export_exports_x_set_a(exports_x_t *x, double arg0) {
    exports_x_set_a(x, arg0);
}

__attribute__((__export_name__("exports#[static]x.add")))
int32_t __wasm_export_exports_x_add(int32_t handle, double arg0) {
    exports_x_owned_t x = { .__handle = handle };
    return exports_x_add(x, arg0).__handle;
}
