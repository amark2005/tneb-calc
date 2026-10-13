//#include <iostream>
#include "utils.hpp"

double under500(const double *units) {
    if (*units < 0.0 || *units > 500.0) {
        return -1.0;
    }

    if (*units <= 200.0) {
        return 0.0;
    } else if (*units <= 400.0) {
        return (*units - 200.0) * 4.70;
    } else {
        return (200.0 * 4.70) + ((*units - 400.0) * 6.30);
    }
}

double above500(const double *units) {
    if (*units <= 500.0) {
        return -1.0;
    }

    double base = (300.0 * 4.70) + (100.0 * 6.30);
    double rate;

    if (*units <= 600.0) {
        rate = 8.40;
    } else if (*units <= 800.0) {
        rate = 9.45;
    } else if (*units <= 1000.0) {
        rate = 10.50;
    } else {
        rate = 11.55;
    }
    

    return base + ((*units - 500.0) * rate);
}



