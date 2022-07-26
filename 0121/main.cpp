#include <vector>

class Solution {
public:
    int maxProfit(std::vector<int>& prices) {
        int min_price = prices[0];
        int max_profit = prices[0] - min_price;
        int potential_profit = 0;
        int size = prices.size();
        
        for (int i = 1; i < size; ++i) {
            potential_profit = prices[i] - min_price;
            max_profit = std::max(max_profit, potential_profit);
            min_price = std::min(prices[i], min_price);
        }
        
        return max_profit;
    }
};