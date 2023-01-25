#include <vector>
#include <utility>
#include <algorithm>
// solution from https://yurukute.github.io/Blog/en/post/sumofintervals/
int sum_intervals(std::vector<std::pair<int, int>> intervals)
{
    std::sort(intervals.begin(), intervals.end());
    int x = intervals[0].first, res = 0;
    for (auto &i : intervals)
    {
        if (i.second >= x)
        {
            res += i.second - (i.first > x ? i.first : x);
            x = i.second;
        }
    }
    return res;
}

long long Carmichael(long long n){
	if (n < 1) return 0;
	if (n == 1) return 1;
	std::vector<long long> factors;
	for(long long i = 2; i*i <= n; i += 2){
		long long w = 0;
		while(n % i == 0){
			w++;
			n /= i;
		}
		if (i == 2 && w >= 3)
			factors.push_back((pow(i, w-1) * (i-1))/2);
		else if(i >= 2 && w > 0)
			factors.push_back(pow(i, w-1) * (i-1));
		if(i == 2) i--;
	}
	if(n != 1) factors.push_back(n-1);
}