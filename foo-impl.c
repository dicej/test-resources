#include "foo.h"

void foo_test(void) {
    imports_y_owned_t y_owned = imports_y_constructor(42.0);
    imports_y_borrowed_t y_borrowed = imports_y_borrow(y_owned);
    if (imports_y_get_a(y_borrowed) != 42.0) {
        abort();
    }

    imports_y_set_a(y_borrowed, 43.0);
    if (imports_y_get_a(y_borrowed) != 43.0) {
        abort();
    }

    imports_y_owned_t y2_owned = imports_y_add(imports_y_constructor(44.0), 45.0);
    imports_y_borrowed_t y2_borrowed = imports_y_borrow(y2_owned);
    if (imports_y_get_a(y2_borrowed) != 89.0) {
        abort();
    }

    imports_y_drop_owned(y_owned);
    imports_y_drop_owned(y2_owned);
}

struct exports_x_t {
    double a;
};

exports_x_owned_t exports_x_constructor(double a) {
    exports_x_t* x = malloc(sizeof(exports_x_t));
    if (x == NULL) {
        abort();
    }
    x->a = a;
    return exports_x_new(x);
}

double exports_x_get_a(exports_x_t *x) {
    return x->a;
}

void exports_x_set_a(exports_x_t *x, double a) {
    x->a = a;
}

exports_x_owned_t exports_x_add(exports_x_owned_t other, double a) {
    exports_x_owned_t result = exports_x_constructor(a + exports_x_owned_rep(other)->a);
    exports_x_drop_owned(other);
    return result;
}

int main(void) {
    return 0;
}
