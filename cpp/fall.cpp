#include <random>

#include "fall.h"

vector<vector<int>> make2darrWithZero(int x, int y) {
    vector<vector<int>> arr = {};
    vector<int> hold = {};

    for (int i = 0; i < y; i++) {
        for (int j = 0; j < x; j++) {
            hold.push_back(0);
        }
        arr.push_back(hold);
        hold.clear();
    }

    return arr;
}

vector<vector<int>> fall(vector<vector<int>> arr) {
    vector<vector<int>> newArr = make2darrWithZero(arr[0].size(), arr.size());
    int hold[] = { 1, -1 };
    int dir = hold[rand() % 2];

    for (int y = 0; y < arr.size(); y++) {
        for (int x = 0; x < arr[y].size(); x++) {
            if (arr[y][x] > 0) {
                if (y + 1 < arr.size()) {
                    if (arr[y + 1][x] == 0) {
                        newArr[y + 1][x] = arr[y][x];
                    }
                    else if (x + 1 < arr[0].size() && dir == 1) {
                        if (arr[y + 1][x + 1] == 0) {
                            newArr[y + 1][x + 1] = arr[y][x];
                        }
                        else {
                            newArr[y][x] = arr[y][x];
                        }
                    }
                    else if (x - 1 >= 0 && dir == -1) {
                        if (arr[y + 1][x - 1] == 0) {
                            newArr[y + 1][x - 1] = arr[y][x];
                        }
                        else {
                            newArr[y][x] = arr[y][x];
                        }
                    }
                    else {
                        newArr[y][x] = arr[y][x];
                    }
                }
                else {
                    newArr[y][x] = arr[y][x];
                }
            }
        }
    }

    return newArr;
}
