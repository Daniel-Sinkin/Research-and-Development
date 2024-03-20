class Solution:
    def wordPattern(self, pattern: str, s: str) -> bool:
        Dict = dict()
        sp = s.split(' ')

        if(len(pattern) != len(sp)):
            return False

        for (key, val) in zip(pattern, sp):
            if key in Dict:
                if Dict[key] != val:
                    return False
            elif val in Dict.values():
                return False
            else:
                Dict[key] = val

        return True