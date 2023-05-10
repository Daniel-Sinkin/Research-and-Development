class Solution {
public:
    bool isValidSudoku(vector<vector<char>>& board) {
        uint64_t encoding = 0;
        uint64_t currDigit = 0;

        // Rows
        for(int r = 0; r < 9; r++) {
            for(int i = 0; i < 9; i++) {
                if(board[r][i] != '.') {
                    currDigit = 1 << (int(board[r][i]) - 48);
                    if((encoding & currDigit) != 0) {
                        return false;
                    }
                    else
                        encoding = encoding + (1 << (int(board[r][i]) - 48));
                }
            }
            encoding = 0;
        }

        // Columns
        for(int i = 0; i < 9; i++) {
            for(int r = 0; r < 9; r++) {
                if(board[r][i] != '.') {
                    currDigit = 1 << (int(board[r][i]) - 48);
                    if((encoding & currDigit) != 0) {
                        return false;
                    }
                    else
                        encoding = encoding + currDigit;
                }
            }
            encoding = 0;
        }

        // 00 01 02 10 11 12 20 21 22
        for(int row = 0; row < 3; row++) {
            for(int col = 0; col < 3; col++) {
                for(int r = 0; r < 3; r++) {
                    for(int c = 0; c < 3; c++) {
                        if(board[r + 3 * row][c + 3 * col] != '.') {
                            currDigit = 1 << (int(board[r + 3 * row][c + 3 * col]) - 48);
                            cout << (bitset<16>(currDigit)).to_string() << endl;
                            if((encoding & currDigit) != 0) {
                                cout << "Broke here!" << endl;
                                return false;
                            }
                            else
                                encoding = encoding + currDigit;
                        }
                    }
                }
                encoding = 0;
            }
            encoding = 0;
        }

        return true;
    }
};