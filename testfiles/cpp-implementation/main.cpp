#pragma GCC target("avx2,tune=native")

#include <iostream>
#include <fstream>
#include <stdlib.h>
#include "SuffixTree.h"

using namespace std;

int main(int argc, char** argv)
{
    if (argc <= 1)
    {
       cout << "USAGE : ./executable file1 file2 file3 ...\n"; 
       return 1;
    }
    string line;
    int size = argc - 1;
    string* words = new string[size];
    for (int index = 1; index < argc; index++)
    {
        string filename = argv[index];
        ifstream file(filename);
        if (file.is_open())
        {
            while(getline(file, line))
            {
                words[index - 1] += line;
            }
        }
        else
        {
            cout << filename << " : No such file\n";
        }
        file.close();
    }
    SuffixTree* deneme = new SuffixTree(words,size);
    deneme -> output();
}
