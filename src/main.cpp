#include <iostream>
#include "utils.hpp"
#include "disp.hpp"

int main([[maybe_unused]]int argc,char *argv[]){
    std::ios_base::sync_with_stdio(false);
    if (argc != 2) {
        std::cerr << "usage: " << argv[0] << " <units>\n";
        return 1;
    }
    
    const double argu=std::stod(argv[1]);
    
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