// Generated by `wit-bindgen` 0.8.0. DO NOT EDIT! Just kidding; edit all you want.
#ifndef __BINDINGS_FOO_H
#define __BINDINGS_FOO_H

#include <stdlib.h>
#include <stdint.h>
#include <stdbool.h>

typedef struct {
    int32_t __handle;
} imports_y_owned_t;

typedef struct {
    int32_t __handle;
} imports_y_borrowed_t;

imports_y_borrowed_t imports_y_borrow(imports_y_owned_t y);
void imports_y_drop_owned(imports_y_owned_t y);

// Imported Functions from `imports`
imports_y_owned_t imports_y_constructor(double a);
double imports_y_get_a(imports_y_borrowed_t y);
void imports_y_set_a(imports_y_borrowed_t y, double a);
imports_y_owned_t imports_y_add(imports_y_owned_t y, double a);

// Exported Functions from `foo`
void foo_test(void);

typedef struct exports_x_t exports_x_t;
typedef struct {
    int32_t __handle;
} exports_x_owned_t;

exports_x_owned_t exports_x_new(exports_x_t *x);
exports_x_t *exports_x_owned_rep(exports_x_owned_t x);
void exports_x_drop_owned(exports_x_owned_t x);

// Exported Functions from `exports`
exports_x_owned_t exports_x_constructor(double a);
double exports_x_get_a(exports_x_t *x);
void exports_x_set_a(exports_x_t *x, double a);
exports_x_owned_t exports_x_add(exports_x_owned_t x, double a);

#endif