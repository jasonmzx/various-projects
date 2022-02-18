#include <string>
#include <map>
#include <vector>
#include <utility> 
#include <algorithm>
#include <math.h>

/*
jasonmzx Roman numeral converter c++ class feb 18 2022
*/

//Compare structure to be used in find_if util from algorithm lib:
struct compare
{
    int key;
    compare(int const& i) : key(i) {}

    bool operator()(int const& i) {
        return (i == key);
    }
};

class RomanHelper {
public:
    std::string to_roman(unsigned int n) {
        std::map<int, std::string> characters = { //I should've refactored the name
      { 1, "I" },{ 2, "II"},{3, "III"},{4, "IV"},{ 5, "V" },{6, "VI"},{7, "VII"},{8, "VIII"},{9, "IX"},{ 10, "X" },
      {11, "XI"},{12, "XII"},{13, "XIII"},{14, "XIV"},{15, "XV"},{16, "XVI"},{17, "XVII"},{18, "XVIII"},{19, "XIX"},
      {40, "XL"},{50, "L"},{90, "XC"},{100, "C"},{400, "CD"},{500, "D"},{900, "CM"},{1000, "M"}
        };

        std::string roman_str = "";

        std::vector<int> divmod_steps = { 1000,900,500,400,100,90,50,40,10 }; //Steps i'm divmodding
        std::map<int, int> Result = {}; //Result Object i'll simultaniously to parse this number as string whilst divmodding (1 pass)

        for (int i = 0; i < divmod_steps.size(); i++) {

            Result[divmod_steps[i]] = std::floor(n / (divmod_steps[i])); //Append to Result Parse Obj

            n -= std::floor(n / (divmod_steps[i])) * divmod_steps[i]; //Substract divmod*step from init number

          //To avoid -1 index & 0 results
            if (i >= 1 && Result[divmod_steps[i - 1]] >= 1) {

                //If loopable, like MMM or CCC or XXX
                if (divmod_steps[i - 1] == 1000 || divmod_steps[i - 1] == 100 || divmod_steps[i - 1] == 10) {

                    //Iter results & append
                    for (int v = 0; v < Result[divmod_steps[i - 1]]; v++) {
                        roman_str += characters[divmod_steps[i - 1]];
                    }

                }
                else {
                    //Append once (for like CM, XL) or those 2 digit descending char pairs
                    roman_str += characters[divmod_steps[i - 1]];
                }


            } //end of str add


        } // Last Part since 10 divmod was neglected in this one pass;


        for (int k = 0; k < Result[10]; k++) {
            roman_str += "X";
        }
        roman_str += characters[n];


        return roman_str;
    }
    int from_roman(std::string rn) {

        //Char reference:
        std::map<char, int> characters = {
        {'I',1},
        {'V',5},
        {'X',10},
        {'L',50},
        {'C',100},
        {'D',500},
        {'M',1000}
        };




        //Tokenizing Numbers for easy addition  
        std::vector<std::pair<int, int>> numberToken;
        std::vector<int> dPairs; //tracking desending pairs (Remark how all numbers with a substraction in the front is only 2 chars long) IX, CM, XL, IV 

        int total = 0;


        for (int i = 0; i < rn.length(); i++) {

            if (characters[rn[i]] < characters[rn[i + 1]]) {
                numberToken.push_back(std::make_pair(i, i + 1));
                i += 1;
                continue;
            }


        }

        for (int k; k < numberToken.size(); k++) {
            total += characters[rn[numberToken[k].second]] - characters[rn[numberToken[k].first]];
            dPairs.push_back(numberToken[k].first);
            dPairs.push_back(numberToken[k].second);

            //std::cout<< "POS: " << k << std::endl;
            //std::cout<< "FIRST: " << numberToken[k].first <<std::endl;
            //std::cout<< "SECOND: " << numberToken[k].second << "\n" <<std::endl;
        }

        for (int j = 0; j < rn.length(); j++) {
            if (std::find_if(dPairs.begin(), dPairs.end(), compare(j)) == dPairs.end()) {
                total += characters[rn[j]];
            }


        }


        return total;
    }
} RomanNumerals;