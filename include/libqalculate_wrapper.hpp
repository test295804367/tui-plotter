
#ifndef LIBQALCULATE_WRAPPER_HPP
#define LIBQALCULATE_WRAPPER_HPP

#include <libqalculate/Calculator.h>

extern "C" {
    Calculator* calculator_new();
    const char* calculator_evaluate(Calculator* calc, const char* expression);
    void calculator_free(Calculator* calc);
}

#endif
