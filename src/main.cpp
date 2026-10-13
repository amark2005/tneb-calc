#include <print>
#include "utils.hpp"
#include "disp.hpp"

extern "C" int rust_stod(
    const char* input,
    double* output
);

int main([[maybe_unused]]int argc,char *argv[]){
    if (argc != 2) {
        std::println("usage: {} <units>",argv[0]);
        return 1;
    }
    double argu=0.0;
    if(rust_stod(argv[1],&argu)==1){
        std::println("Invalid Number\nexitinggg....");
        return 1;
    }
    
    if(argu<=500){
        const double bill=under500(&argu);
        disp(&bill);
    }
    else {  
    const double bill=above500(&argu);
    disp(&bill);
    }


    return 0;
}